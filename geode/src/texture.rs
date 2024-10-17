use std::path::{Path, PathBuf};

use image::{GenericImageView, ImageError};
use isle_math::vector::d2::Vec2;

pub enum TextureSource {
    Disk(PathBuf),
    Internal(&'static str),
    Dynamic(&'static str),
}

pub struct GpuTexture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}
pub struct AtlasedTexture {
    source_id: usize,
    uv: Vec2,
}
pub enum TextureState {
    Disk,
    Memory(image::DynamicImage),
    Gpu(GpuTexture),
    Atlased(AtlasedTexture),
}

pub struct Texture {
    source: TextureSource,
    state: TextureState,
    size: Vec2,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let size = wgpu::Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("__geode_internal__depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        Self {
            source: TextureSource::Internal("Depth Texture"),
            state: TextureState::Gpu(GpuTexture { texture, view, sampler }),
            size: Vec2(size.width as f32, size.height as f32),
        }
    }

    pub fn get_view(&self) -> &wgpu::TextureView {
        if let TextureState::Gpu(GpuTexture { view, .. }) = &self.state {
            &view
        } else {
            panic!("Texture is not in GPU state");
        }
    }
}