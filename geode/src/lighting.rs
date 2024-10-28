use std::sync::atomic::{AtomicBool, Ordering};

use isle_math::vector::d3::Vec3;
use wgpu::util::DeviceExt;

use crate::renderer::Renderer;

pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
}

pub struct SpotLight {
    pub position: Vec3,
    pub color: Vec3,
    pub direction: Vec3,
    pub limit: f32,
    pub decay: f32,
}

pub struct Lighting {
    pub ambient_color: Vec3,
    pub ambient_intensity: f32,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) buffer: wgpu::Buffer,
    point_lights: Vec<PointLight>,
    spot_lights: Vec<SpotLight>,
    dirty: AtomicBool,
}

#[repr(C, align(16))]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LightingRaw {
    ambient_color: [f32; 3],
    ambient_intensity: f32,
    num_point_lights: u32,
    num_spot_lights: u32,
    _padding: [u32; 2],
}

pub struct LightingSettings {
    ambient_color: Vec3,
    ambient_intensity: f32,
}

impl Default for LightingSettings {
    fn default() -> Self {
        Self {
            ambient_color: Vec3(1.0, 1.0, 1.0),
            ambient_intensity: 0.03,
        }
    }
}

const MAX_LIGHTS: usize = 128;

impl Lighting {
    pub fn new(device: &wgpu::Device, settings: LightingSettings) -> Self {
        let mut buffer_content = bytemuck::bytes_of(&LightingRaw {
            ambient_color: settings.ambient_color.into(),
            ambient_intensity: settings.ambient_intensity,
            num_point_lights: 0,
            num_spot_lights: 0,
            _padding: [0; 2],
        }).to_vec();

        const LIGHTS_SIZE: usize = std::mem::size_of::<PointLightRaw>() + std::mem::size_of::<SpotLightRaw>();
        buffer_content.extend_from_slice(&[0u8; LIGHTS_SIZE * MAX_LIGHTS]);

        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Lighting Buffer"),
                contents: bytemuck::cast_slice(buffer_content.as_slice()),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = Self::bind_group_layout(&device);

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }
                ],
                label: Some("Lighting Bind Group"),
            }
        );

        Self {
            point_lights: Vec::new(),
            spot_lights: Vec::new(),
            bind_group_layout,
            bind_group,
            buffer,
            dirty: AtomicBool::new(false),
            ambient_color: settings.ambient_color,
            ambient_intensity: settings.ambient_intensity,
        }
    }

    fn to_raw(&self) -> Vec<u8> {
        let raw = LightingRaw {
            ambient_color: self.ambient_color.into(),
            ambient_intensity: self.ambient_intensity,
            num_point_lights: self.point_lights.len() as u32,
            num_spot_lights: self.spot_lights.len() as u32,
            _padding: [0; 2],
        };

        let point_lights = self.point_lights.iter().map(PointLight::to_raw).collect::<Vec<_>>();
        let mut buffer = bytemuck::bytes_of(&[raw]).to_vec();
        buffer.extend_from_slice(bytemuck::cast_slice(&point_lights));
        let padding_size = std::mem::size_of::<PointLightRaw>() * (MAX_LIGHTS - self.point_lights.len());
        buffer.extend_from_slice(&vec![0u8; padding_size]);

        let spot_lights = self.spot_lights.iter().map(SpotLight::to_raw).collect::<Vec<_>>();
        buffer.extend_from_slice(bytemuck::cast_slice(&spot_lights));
        let padding_size = std::mem::size_of::<SpotLightRaw>() * (MAX_LIGHTS - self.spot_lights.len());
        buffer.extend_from_slice(&vec![0u8; padding_size]);

        buffer
    }

    pub(crate) fn update_buffer(&self, queue: &wgpu::Queue) {
        if self.dirty.swap(false, Ordering::SeqCst) {
            queue.write_buffer(&self.buffer, 0, &self.to_raw());
        }
    }

    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: true,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Light Bind Group Layout"),
        })
    }

    pub fn add_point_light(&mut self, light: PointLight) {
        self.point_lights.push(light);
        self.dirty.store(true, Ordering::SeqCst);
    }

    pub fn add_spot_light(&mut self, light: SpotLight) {
        self.spot_lights.push(light);
        self.dirty.store(true, Ordering::SeqCst);
    }
}

impl PointLight {
    pub(crate) fn to_raw(&self) -> PointLightRaw {
        PointLightRaw {
            position: self.position.into(),
            _padding0: 0,
            color: self.color.into(),
            _padding1: 0,
        }
    }

}

impl SpotLight {
    pub(crate) fn to_raw(&self) -> SpotLightRaw {
        SpotLightRaw {
            position: self.position.into(),
            _padding0: 0,
            color: self.color.into(),
            _padding1: 0,
            direction: self.direction.into(),
            limit: self.limit,
            decay: self.decay,
            _padding2: [0; 3],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct PointLightRaw {
    position: [f32; 3],
    _padding0: u32,
    color: [f32; 3],
    _padding1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct SpotLightRaw {
    position: [f32; 3],
    _padding0: u32,
    color: [f32; 3],
    _padding1: u32,
    direction: [f32; 3],
    limit: f32,
    decay: f32,
    _padding2: [u32; 3],
}
