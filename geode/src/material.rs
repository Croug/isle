pub struct Material {
    pipeline_layout: wgpu::PipelineLayout,
    pipeline: wgpu::RenderPipeline,
    bind_group_layouts: Vec<(usize, wgpu::BindGroupLayout)>,
    bind_groups: Vec<(usize, wgpu::BindGroup)>,
}