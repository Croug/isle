pub struct Material {
    pub(crate) pipeline_layout: wgpu::PipelineLayout,
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) bind_group_layouts: Vec<(usize, wgpu::BindGroupLayout)>,
    pub(crate) bind_groups: Vec<(usize, wgpu::BindGroup)>,
    pub(crate) instances: Vec<MaterialInstance>,
}

impl Material {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        todo!()
    }
}

pub struct MaterialInstance {
    pub(crate) bind_group: wgpu::BindGroup,
}