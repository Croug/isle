use std::path::PathBuf;

use isle_math::{
    matrix::Mat4, rotation::Rotation, vector::{d2::Vec2, d3::Vec3}
};
use rustc_hash::FxHashMap;
use tobj::{load_obj, LoadError, LoadOptions};
use wgpu::util::DeviceExt;

use crate::renderer::Vertex;

pub enum GeometryType {
    Lines(Vec<u32>),
    Tris(Vec<u32>),
    Quads(Vec<u32>),
    Points,
}

pub enum GeometrySource {
    Disk(PathBuf),
    Internal(&'static str),
    Dynamic(&'static str),
}

pub struct Mesh {
    pub(crate) geometry_type: GeometryType,
    pub(crate) vertices: Vec<Vec3>,
    // pub(crate) normals: Option<Vec<Vec3>>,
    pub(crate) uvs: Vec<Vec2>,
}

impl From<tobj::Mesh> for Mesh {
    fn from(value: tobj::Mesh) -> Self {
        let num_positions = value.positions.len() / 3;
        let num_uvs = value.texcoords.len() / 2;

        assert_eq!(num_positions, num_uvs, "Number of positions and uvs must match\nVertices: {num_positions}\nUVs: {num_uvs}");

        Self {
            geometry_type: GeometryType::Tris(value.indices),
            vertices: value.positions.chunks_exact(3).map(|v| Vec3(v[0], v[1], v[2])).collect(),
            uvs: value.texcoords.chunks_exact(2).map(|v| Vec2(v[0], v[1])).collect(),
        }
    }
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
    pub(crate) source: GeometrySource,
    pub(crate) state: GeometryState,
    pub(crate) instances: FxHashMap<usize, Vec<GeometryInstance>>,
}

impl Geometry {
    pub(crate) fn vertices(&self) -> Vec<Vertex> {
        let mesh = match &self.state {
            GeometryState::Memory(mesh) => mesh,
            _ => panic!("Geometry not loaded into memory"),
        };

        mesh.vertices
            .iter()
            .enumerate()
            .map(|(i, vertex)| Vertex {
                position: vertex.clone().into(),
                // normal: mesh.normals.as_ref().unwrap()[i].clone().into(),
                uv: mesh.uvs[i].clone().into(),
            })
            .collect()
    }
    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        if let GeometryState::Gpu(mesh) = &self.state {
            &mesh.vertex_buffer
        } else {
            panic!("Geometry not loaded into GPU memory")
        }
    }
    pub fn instance_buffer(&self, material_id: usize, instance_id: usize, device: &wgpu::Device) -> wgpu::Buffer {
        let data = self.instances.get(&material_id)
            .unwrap()
            .iter()
            .filter(|instance| instance.instance_id == instance_id)
            .map(|instance| instance.to_raw())
            .collect::<Vec<_>>();

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} Instance Buffer", self.name()).as_str()),
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
    pub fn indices(&self) -> &[u32] {
        if let GeometryState::Memory(mesh) = &self.state {
            match &mesh.geometry_type {
                GeometryType::Lines(indices) => indices,
                GeometryType::Tris(indices) => indices,
                GeometryType::Quads(indices) => indices,
                GeometryType::Points => panic!("Cannot get indices for point geometry"),
            }
        } else {
            panic!("Geometry not loaded into memory")
        }
    }
    pub fn index_buffer(&self) -> &wgpu::Buffer {
        if let GeometryState::Gpu(mesh) = &self.state {
            &mesh.index_buffer
        } else {
            panic!("Geometry not loaded into GPU memory")
        }
    }
    pub fn num_instances(&self, material_id: usize, instance_id: usize) -> usize {
        let empty = Vec::new();
        self.instances.get(&material_id).unwrap_or(&empty)
            .iter()
            .filter(|instance| instance.instance_id == instance_id)
            .count()
    }
    pub fn num_indices(&self) -> u32 {
        if let GeometryState::Gpu(mesh) = &self.state {
            mesh.num_indices
        } else {
            panic!("Geometry not loaded into GPU memory")
        }
    }
    pub fn name(&self) -> &str {
        match &self.source {
            GeometrySource::Disk(path) => path.file_name().unwrap().to_str().unwrap(),
            GeometrySource::Internal(name) => name,
            GeometrySource::Dynamic(name) => name,
        }
    }

    pub fn load_to_mem(&mut self) -> Result<(), LoadError> {
        let path = if let GeometrySource::Disk(path) = &self.source {
            path
        } else {
            log::warn!(
                "Attempted to load non-disk '{}' geometry to memory",
                self.name()
            );
            return Ok(());
        };

        let (mut models, _) = load_obj(path, &LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_lines: true,
            ignore_points: true,
        })?;

        let model = models.remove(0);

        self.state = GeometryState::Memory(model.mesh.into());

        Ok(())
    }

    pub fn load_to_gpu(&mut self, device: &wgpu::Device) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} Vertex Buffer", self.name()).as_str()),
            contents: bytemuck::cast_slice(self.vertices().as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let indices = self.indices();
        let num_indices = indices.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} Index Buffer", self.name()).as_str()),
            contents: bytemuck::cast_slice(self.indices()),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.state = GeometryState::Gpu(GpuMesh {
            vertex_buffer,
            index_buffer,
            num_indices,
        });
    }

    pub fn plane(size: Vec2) -> Self {
        let half_size = size / 2.0;
        let vertices = vec![
            Vec3(-half_size.0, -half_size.1, 0.0),
            Vec3(half_size.0, -half_size.1, 0.0),
            Vec3(half_size.0, half_size.1, 0.0),
            Vec3(-half_size.0, half_size.1, 0.0),
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        let uvs = vec![
            Vec2(0.0, 1.0),
            Vec2(1.0, 1.0),
            Vec2(1.0, 0.0),
            Vec2(0.0, 0.0),
        ];

        Self {
            source: GeometrySource::Internal("Plane"),
            state: GeometryState::Memory(Mesh {
                geometry_type: GeometryType::Tris(indices),
                vertices,
                // normals: None,
                uvs,
            }),
            instances: FxHashMap::default(),
        }
    }

    pub fn cube(size: Vec3) -> Self {
        let half_size = size / 2.0;
        let vertices = vec![
            Vec3(-half_size.0, -half_size.1, -half_size.2),
            Vec3(half_size.0, -half_size.1, -half_size.2),
            Vec3(half_size.0, half_size.1, -half_size.2),
            Vec3(-half_size.0, half_size.1, -half_size.2),
            Vec3(-half_size.0, -half_size.1, half_size.2),
            Vec3(half_size.0, -half_size.1, half_size.2),
            Vec3(half_size.0, half_size.1, half_size.2),
            Vec3(-half_size.0, half_size.1, half_size.2),
        ];
        let indices = vec![
            0, 1, 2, 2, 3, 0, // Front
            1, 5, 6, 6, 2, 1, // Right
            5, 4, 7, 7, 6, 5, // Back
            4, 0, 3, 3, 7, 4, // Left
            3, 2, 6, 6, 7, 3, // Top
            4, 5, 1, 1, 0, 4, // Bottom
        ];
        let uvs = vec![
            Vec2(0.0, 1.0),
            Vec2(1.0, 1.0),
            Vec2(1.0, 0.0),
            Vec2(0.0, 0.0),
            Vec2(1.0, 1.0),
            Vec2(0.0, 1.0),
            Vec2(0.0, 0.0),
            Vec2(1.0, 0.0),
        ];

        Self {
            source: GeometrySource::Internal("Cube"),
            state: GeometryState::Memory(Mesh {
                geometry_type: GeometryType::Tris(indices),
                vertices,
                // normals: None,
                uvs,
            }),
            instances: FxHashMap::default(),
        }
    }

    pub fn instantiate(&mut self, material_id: usize, material_instance_id: usize, translation: Vec3, rotation: Rotation, scale: Vec3) -> usize {
        let transform = Mat4::transform(scale, &rotation, translation);
        self.instances
            .entry(material_id)
            .or_insert_with(Vec::new)
            .push(GeometryInstance {
                instance_id: material_instance_id,
                transform,
            });

        self.instances.get(&material_id).unwrap().len() - 1
    }

    pub fn update_instance(&mut self, material_id: usize, instance_id: usize, translation: Vec3, rotation: Rotation, scale: Vec3) {
        let transform = Mat4::transform(scale, &rotation, translation);
        self.instances.get_mut(&material_id)
            .unwrap()
            .get_mut(instance_id)
            .unwrap()
            .transform = transform;
    }
}

pub struct GeometryInstance {
    pub(crate) instance_id: usize,
    pub(crate) transform: Mat4,
}

impl GeometryInstance {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &GeometryInstance::ATTRIBS,
        }
    }

    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        self.transform.0
    }
}
