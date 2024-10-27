use std::{f32::consts::PI, sync::atomic::{AtomicBool, Ordering}};

use isle_math::{
    matrix::Mat4,
    rotation::{quaternion::Quaternion, Angle, Rotation},
    vector::{d2::Vec2, d3::Vec3},
};
use wgpu::{util::DeviceExt, BindGroupDescriptor};

use crate::{renderer::Renderer, texture::Texture};

pub struct Camera {
    pub(crate) texture_id: usize,
    pub(crate) label: &'static str,
    pub(crate) clear_color: wgpu::Color,
    pub(crate) depth_texture: Texture,
    pub(crate) buffer: wgpu::Buffer,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) viewport: Vec2,
    pub(crate) position: Vec3,
    pub(crate) projection: CameraProjection,
    pub(crate) view_mat: Mat4,
    pub(crate) projection_mat: Mat4,

    dirty: AtomicBool,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    position: [f32; 3],
    _padding: f32,
}

#[derive(Clone, Copy)]
pub enum CameraProjection {
    Perspective {
        fovy: f32,
        znear: f32,
        zfar: f32,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        znear: f32,
        zfar: f32,
    },
    None,
}

pub struct CameraCreationSettings {
    pub label: &'static str,
    pub clear_color: wgpu::Color,
    pub viewport: Vec2,
    pub position: Vec3,
    pub orientation: Rotation,
    pub scale: Vec3,
    pub projection: CameraProjection,
}

impl Default for CameraCreationSettings {
    fn default() -> Self {
        let position = Vec3(0.0, 500., -500.0);
        Self {
            label: "Camera",
            clear_color: wgpu::Color::BLACK,
            viewport: Vec2(800.0, 600.0),
            position,
            orientation: Quaternion::look_at(&position, &Vec3::ZERO).into(),
            scale: Vec3::IDENTITY,
            projection: CameraProjection::Perspective {
                fovy: 60.0,
                znear: 10.0,
                zfar: 100000.0,
            },
        }
    }
}

impl Camera {
    pub fn new(renderer: &mut Renderer, settings: &CameraCreationSettings) -> Self {
        let texture = Texture::create_camera_texture(settings.viewport, renderer.device(), settings.label);
        let texture_id = renderer.add_texture(texture);
        let depth_texture = Texture::create_depth_texture(renderer.device(), settings.viewport);

        let view = Mat4::inverse_transform(settings.scale, &settings.orientation, settings.position);
        let projection_mat = match settings.projection {
            CameraProjection::Perspective { fovy, znear, zfar } =>
                Mat4::perspective_projection(settings.viewport.0 / settings.viewport.1, Angle::Degrees(fovy), znear, zfar),

            CameraProjection::Orthographic { left, right, bottom, top, znear, zfar } =>
                Mat4::orthographic_projection(left, right, bottom, top, znear, zfar),

            CameraProjection::None => Mat4::identity(),
        };

        let camera_uniform = CameraUniform {
            view_proj: (projection_mat * view).0,
            position: settings.position.into(),
            _padding: 0.0,
        };

        let buffer = renderer.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(format!("{} Buffer", settings.label).as_str()),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group = renderer.device().create_bind_group(
            &BindGroupDescriptor {
                layout: renderer.camera_bind_group_layout(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    },
                ],
                label: Some(format!("{} Bind Group", settings.label).as_str()),
            }
        );

        Self {
            texture_id,
            label: settings.label,
            clear_color: settings.clear_color,
            viewport: settings.viewport,
            projection: settings.projection,
            buffer,
            bind_group,
            depth_texture,
            view_mat: view,
            projection_mat,
            position: settings.position,

            dirty: AtomicBool::new(false),
        }
    }

    pub fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Camera Bind Group Layout"),
        })
    }

    pub fn begin_render_pass<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) -> wgpu::RenderPass {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(format!("Render Pass: {}", self.label).as_str()),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view(),
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

    pub fn set_dirty(&self) {
        self.dirty.store(true, Ordering::SeqCst);
    }

    pub fn update_view(&mut self, position: &Vec3, orientation: &Rotation, scale: &Vec3) {
        self.view_mat = Mat4::inverse_transform(*scale, orientation, *position);
        self.position = *position;
        self.dirty.store(true, Ordering::SeqCst);
    }

    pub fn update_projection(&mut self, projection: CameraProjection) {
        self.projection_mat = match projection {
            CameraProjection::Perspective { fovy, znear, zfar } =>
                Mat4::perspective_projection(self.viewport.0 / self.viewport.1, Angle::Degrees(fovy), znear, zfar),

            CameraProjection::Orthographic { left, right, bottom, top, znear, zfar } =>
                Mat4::orthographic_projection(left, right, bottom, top, znear, zfar),

            CameraProjection::None => Mat4::identity(),
        };

        self.dirty.store(true, Ordering::SeqCst);
    }

    pub fn update_buffer(&self, queue: &wgpu::Queue) {
        if self.dirty.swap(false, Ordering::SeqCst) {
            let camera_buffer = CameraUniform {
                view_proj: (self.projection_mat * self.view_mat).0,
                position: self.position.into(),
                _padding: 0.0,
            };
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[camera_buffer]));
        }
    }
}
