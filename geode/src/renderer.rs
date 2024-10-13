use crate::{geometry::{Geometry, GeometryInstance}, material::Material, texture::Texture};

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    depth_texture: Texture,

    geometries: Vec<Geometry>,
    geometry_instances: Vec<GeometryInstance>,
    textures: Vec<Texture>,
    materials: Vec<Material>,
}