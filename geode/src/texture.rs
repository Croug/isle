use std::path::{Path, PathBuf};

use image::{GenericImageView, ImageError};
use isle_math::vector::d2::Vec2;

use crate::material::IntoBindGroup;

pub enum TextureSource {
    Disk(PathBuf),
    Internal(&'static str),
    Dynamic(&'static str),
}

pub struct GpuTexture {
    #[allow(dead_code)]
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

    pub fn new(path: &Path) -> Self {
        Texture {
            source: TextureSource::Disk(path.to_path_buf()),
            state: TextureState::Disk,
            size: Vec2(0.0, 0.0),
        }
    }

    pub fn name(&self) -> &str {
        match &self.source {
            TextureSource::Disk(path) => path.file_name().unwrap().to_str().unwrap(),
            TextureSource::Internal(name) => name,
            TextureSource::Dynamic(name) => name,
        }
    }

    pub fn load_to_mem(&mut self) -> Result<(), ImageError> {
        let path = if let TextureSource::Disk(path) = &self.source {
            path
        } else {
            log::warn!(
                "Attempted to load non-disk '{}' texture to memory",
                self.name()
            );
            return Ok(());
        };

        let image = image::open(path)?;
        self.size = image.dimensions().into();
        self.state = TextureState::Memory(image);

        Ok(())
    }

    pub fn load_to_gpu(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let image = match &self.state {
            TextureState::Memory(image) => image,
            TextureState::Gpu(_) => {
                log::warn!(
                    "Attempted to load already loaded '{}' texture to GPU",
                    self.name()
                );
                return;
            }
            TextureState::Atlased(_) => {
                log::warn!("Attempted to load atlased '{}' texture to GPU", self.name());
                return;
            }
            TextureState::Disk => {
                log::error!(
                    "Attempted to load unloaded '{}' texture to GPU",
                    self.name()
                );
                panic!("Attempted to load unloaded texture to GPU");
            }
        };

        let rgba = image.to_rgba8();
        let size = wgpu::Extent3d {
            width: self.size.0 as u32,
            height: self.size.1 as u32,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(self.name()),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width),
                rows_per_image: Some(size.height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        self.state = TextureState::Gpu(GpuTexture {
            texture,
            view,
            sampler,
        });
    }

    pub fn create_depth_texture(
        device: &wgpu::Device,
        size: Vec2,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: size.0.max(1.0) as u32,
            height: size.1.max(1.0) as u32,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("__geode_internal__depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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
        });

        Self {
            source: TextureSource::Internal("Depth Texture"),
            state: TextureState::Gpu(GpuTexture {
                texture,
                view,
                sampler,
            }),
            size: Vec2(size.width as f32, size.height as f32),
        }
    }

    pub fn create_camera_texture(
        size: Vec2,
        device: &wgpu::Device,
        label: &'static str,
    ) -> Self {
        let size_wgpu = wgpu::Extent3d {
            width: size.0 as u32,
            height: size.1 as u32,
            depth_or_array_layers: 1, 
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size: size_wgpu,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            source: TextureSource::Internal(label),
            state: TextureState::Gpu(GpuTexture {
                texture,
                view,
                sampler,
            }),
            size,
        }
    }

    pub fn view(&self) -> &wgpu::TextureView {
        if let TextureState::Gpu(GpuTexture { view, .. }) = &self.state {
            &view
        } else {
            panic!("Texture is not in GPU state");
        }
    }
    pub fn sampler(&self) -> &wgpu::Sampler {
        if let TextureState::Gpu(GpuTexture { sampler, .. }) = &self.state {
            &sampler
        } else {
            panic!("Texture is not in GPU state");
        }
    }
}

impl IntoBindGroup for Texture {
    fn into_bind_group<'a>(&'a self, bindings: &mut Vec<wgpu::BindGroupEntry<'a>>) {
        let next_index = bindings.len() as u32;
        bindings.extend(vec![
            wgpu::BindGroupEntry {
                binding: next_index,
                resource: wgpu::BindingResource::TextureView(&self.view()),
            },
            wgpu::BindGroupEntry {
                binding: next_index + 1,
                resource: wgpu::BindingResource::Sampler(&self.sampler()),
            }
        ]);
    }
}
