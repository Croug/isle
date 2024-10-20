use isle_math::{matrix::Mat4, rotation::Rotation, vector::{d2::Vec2, d3::Vec3}};

use crate::texture::Texture;

pub struct Camera {
    pub(crate) texture_id: usize,
    pub(crate) label: &'static str,
    pub(crate) clear_color: wgpu::Color,
    pub(crate) depth_texture: Texture,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) viewport: Vec2,
    pub(crate) view: Mat4,
    pub(crate) projection: Mat4,
}

impl Camera {
    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some("Camera Bind Group Layout"),
            }
        )
    }
    pub fn begin_render_pass<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder, view: &wgpu::TextureView, surface_view: Option<&wgpu::TextureView>) -> wgpu::RenderPass {
        let surface_attachment = surface_view.map(|view| wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(self.clear_color),
                store: wgpu::StoreOp::Store,
            }
        });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(format!("Render Pass: {}", self.label).as_str()),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: wgpu::StoreOp::Store,
                }
            }), surface_attachment],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.get_view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_bind_group(0, &self.bind_group, &[]);

        render_pass
    }

    pub fn update_view(&mut self, scale: Vec3, rotation: &Rotation, translation: Vec3) {
        self.view = Mat4::transform(scale, rotation, translation);
    }

    pub fn update_projection_perspective(&mut self, fovy: f32, znear: f32, zfar: f32) {
        self.projection = Mat4::perspective_projection(self.viewport.0 / self.viewport.1, fovy, znear, zfar);
    }
}