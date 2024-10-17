use std::path::PathBuf;

use isle_math::{matrix::Mat4, vector::{Vec2, Vec3}};

pub enum GeometryType {
    Lines(Vec<usize>),
    Tris(Vec<usize>),
    Quads(Vec<usize>),
    Points
}

pub struct Mesh {
    pub(crate) geometry_type: GeometryType,
    pub(crate) vertices: Vec<Vec3>,
    pub(crate) normals: Vec<Vec3>,
    pub(crate) uvs: Vec<Vec2>,
}

pub struct GpuMesh {
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

pub enum GeometryState {
    Disk,
    Memory(Mesh),
    Gpu(GpuMesh),
}

pub struct Geometry {
    pub(crate) file_source: PathBuf,
    pub(crate) state: GeometryState,
    pub(crate) instances: Vec<Option<(wgpu::Buffer, Vec<GeometryInstance>)>>,
}

impl Geometry {
    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        if let GeometryState::Gpu(mesh) = &self.state {
            &mesh.vertex_buffer
        } else {
            panic!("Geometry not loaded into GPU memory")
        }
    }
    pub fn instance_buffer(&self, material_id: usize) -> &wgpu::Buffer {
        &self.instances[material_id].as_ref().unwrap().0
    }
    pub fn index_buffer(&self) -> &wgpu::Buffer {
        if let GeometryState::Gpu(mesh) = &self.state {
            &mesh.index_buffer
        } else {
            panic!("Geometry not loaded into GPU memory")
        }
    }
    pub fn num_instances(&self, material_id: usize) -> usize {
        self.instances[material_id].as_ref().unwrap().1.len()
    }
    pub fn num_indices(&self) -> u32 {
        if let GeometryState::Gpu(mesh) = &self.state {
            mesh.num_indices
        } else {
            panic!("Geometry not loaded into GPU memory")
        }
    }
}

pub struct GeometryInstance {
    pub(crate) geometry_id: usize,
    pub(crate) material_id: usize,
    pub(crate) texture_id: usize,
    pub(crate) transform: Mat4,
}