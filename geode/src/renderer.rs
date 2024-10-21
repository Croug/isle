use std::vec;

use isle_math::vector::d2::Vec2;
use wgpu::VertexBufferLayout;

use crate::{camera::{Camera, CameraCreationSettings, CameraProjection}, geometry::Geometry, material::{IntoBindGroup, Material}, texture::Texture};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3],
    // pub(crate) normal: [f32; 3],
    pub(crate) uv: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 2 => Float32x2];

    pub fn desc() -> VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>, // contains unsafe reference to window
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: Vec2,
    camera_bind_group_layout: wgpu::BindGroupLayout,

    #[allow(dead_code)]
    window: wgpu::SurfaceTarget<'a>,

    main_camera: usize,

    cameras: Vec<Camera>,
    geometries: Vec<Geometry>,
    textures: Vec<Texture>,
    materials: Vec<Material>,
}

impl<'a> Renderer<'a> {
    pub async fn new(window: impl Into<wgpu::SurfaceTarget<'a>> + Copy, size: Vec2) -> Result<Self, wgpu::CreateSurfaceError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;
        let window = window.into();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let camera_bind_group_layout = Camera::bind_group_layout(&device);

        let mut out = Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            camera_bind_group_layout,

            main_camera: 0,

            cameras: Vec::new(),
            geometries: Vec::new(),
            textures: Vec::new(),
            materials: Vec::new(),
        };

        let main_camera = Camera::new(
            &mut out,
            &CameraCreationSettings {
                label: "Main Camera",
                viewport: size,
                projection: CameraProjection::None,
                ..Default::default()
            }
        );

        out.cameras.push(main_camera);

        Ok(out)
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn texture(&self, texture_id: usize) -> &Texture {
        &self.textures[texture_id]
    }

    pub fn camera_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.camera_bind_group_layout
    }

    pub fn resize(&mut self, new_size: Vec2) {
        if new_size.0 > 0. && new_size.0 > 0. {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.cameras[self.main_camera].depth_texture =
                Texture::create_depth_texture(&self.device, Vec2(self.config.width as f32, self.config.height as f32));
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render_geometries_by_material<'r>(
        &self,
        material_id: usize,
        instance_id: usize,
        render_pass: &mut wgpu::RenderPass<'r>,
    ) {
        self.geometries
            .iter()
            .filter(|geometry| geometry.num_instances(material_id, instance_id) > 0)
            .for_each(|geometry| {
                render_pass.set_vertex_buffer(0, geometry.vertex_buffer().slice(..));
                render_pass.set_vertex_buffer(
                    1,
                    geometry
                        .instance_buffer(material_id, instance_id, &self.device)
                        .slice(..),
                );
                render_pass
                    .set_index_buffer(geometry.index_buffer().slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(
                    0..geometry.num_indices(),
                    0,
                    0..geometry.num_instances(material_id, instance_id) as _,
                );
            });
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.cameras
            .iter()
            .enumerate()
            .for_each(|(camera_id, camera)| {
                let view = if camera_id == self.main_camera {
                    &view
                } else {
                    &self.cameras[self.main_camera].depth_texture.view()
                };
                let mut render_pass = camera.begin_render_pass(&mut encoder, view);

                camera.update_buffer(&self.queue);

                self.materials
                    .iter()
                    .enumerate()
                    .for_each(|(material_id, material)| {
                        render_pass.set_pipeline(&material.pipeline);

                        material.instances.iter().enumerate().for_each(|(instance_id, instance)| {
                            render_pass.set_bind_group(1, &instance.bind_group, &[]);
                            self.render_geometries_by_material(material_id, instance_id, &mut render_pass);
                        });
                    })
            });

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }

    pub fn add_texture(&mut self, texture: Texture) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }
    
    pub fn add_geometry(&mut self, geometry: Geometry) -> usize {
        self.geometries.push(geometry);
        self.geometries.len() - 1
    }

    pub fn instantiate_geometry(&mut self, geometry_id: usize, material_id: usize, material_instance_id: usize, translation: isle_math::vector::d3::Vec3, rotation: isle_math::rotation::Rotation, scale: isle_math::vector::d3::Vec3) -> usize {
        self.geometries[geometry_id].instantiate(material_id, material_instance_id, translation, rotation, scale)
    }

    pub fn update_geometry_instance(&mut self, geometry_id: usize, material_id: usize, instance_id: usize, translation: isle_math::vector::d3::Vec3, rotation: isle_math::rotation::Rotation, scale: isle_math::vector::d3::Vec3) {
        self.geometries[geometry_id].update_instance(material_id, instance_id, translation, rotation, scale);
    }

    pub fn add_material(&mut self, material: Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn instantiate_material(&mut self, material_id: usize, label: &'static str, entries: &dyn IntoBindGroup) -> usize {
        self.materials[material_id].instantiate(&self.device, label, entries)
    }
}
