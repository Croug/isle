use std::path::PathBuf;

use isle_math::{matrix::Mat4, vector::{Vec2, Vec3}};

pub enum GeometryType {
    Lines(Vec<usize>),
    Tris(Vec<usize>),
    Quads(Vec<usize>),
    Points
}

pub struct Mesh {
    geometry_type: GeometryType,
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
}

pub struct GpuMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

pub enum GeometryState {
    Disk,
    Memory(Mesh),
    Gpu(GpuMesh),
}

pub struct Geometry {
    file_source: PathBuf,
    state: GeometryState,
}

pub struct GeometryInstance {
    geometry_id: usize,
    material_id: usize,
    texture_id: usize,
    transform: Mat4<f32>,
}