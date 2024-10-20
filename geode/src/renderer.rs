use wgpu::VertexBufferLayout;
use winit::window::Window;

use crate::{camera::Camera, geometry::Geometry, material::Material, texture::Texture};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) normal: [f32; 3],
    pub(crate) uv: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];

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
    size: winit::dpi::PhysicalSize<u32>,
    camera_bind_group_layout: wgpu::BindGroupLayout,

    #[allow(dead_code)]
    window: &'a Window, // must be last to drop last

    default_camera: usize,

    cameras: Vec<Camera>,
    geometries: Vec<Geometry>,
    textures: Vec<Texture>,
    materials: Vec<Material>,
}

impl<'a> Renderer<'a>{
    pub async fn new(window: &'a Window) -> Result<Self, wgpu::CreateSurfaceError> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let camera_bind_group_layout = Camera::bind_group_layout(&device);

        Ok(Self{
            surface,
            device,
            queue,
            config,
            size,
            window,
            camera_bind_group_layout,

            default_camera: 0,

            cameras: Vec::new(),
            geometries: Vec::new(),
            textures: Vec::new(),
            materials: Vec::new(),
        })
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn camera_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.camera_bind_group_layout
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.cameras[self.default_camera].depth_texture = Texture::create_depth_texture(&self.device, &self.config);
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render_geometries_by_material<'r>(&self, material_id: usize, render_pass: &mut wgpu::RenderPass<'r>) {
        self.geometries.iter()
            .filter(|geometry| geometry.instances[material_id].is_some())
            .for_each(|geometry| {
                render_pass.set_vertex_buffer(0, geometry.vertex_buffer().slice(..));
                render_pass.set_vertex_buffer(1, geometry.instance_buffer(material_id, &self.device).slice(..));
                render_pass.set_index_buffer(geometry.index_buffer().slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..geometry.num_indices(), 0, 0..geometry.num_instances(material_id) as _);
            });
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.cameras.iter()
            .enumerate()
            .for_each(|(camera_id, camera)| {
                let surface_texture = if camera_id == self.default_camera {
                    Some(&view)
                } else {
                    None
                };

                let view = self.textures[camera.texture_id].get_view();
                let mut render_pass = camera.begin_render_pass(&mut encoder, view, surface_texture);

                self.materials.iter()
                    .enumerate()
                    .for_each(|(material_id, material)|{
                        render_pass.set_pipeline(&material.pipeline);

                        material.instances.iter()
                            .for_each(|instance| {
                                render_pass.set_bind_group(1, &instance.bind_group, &[]);
                                self.render_geometries_by_material(material_id, &mut render_pass);
                            });
                    })
            });

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}