use std::path::PathBuf;

use isle_math::vector::Vec2;

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
    file_source: PathBuf,
    state: TextureState,
    size: winit::dpi::PhysicalSize<u32>,
}