use crate::{
    geometry::GeometryInstance,
    renderer::{Renderer, Vertex},
    texture::Texture,
};

pub trait IntoBindGroup {
    fn into_bind_group<'a>(
        &'a self,
        data: &'a Renderer,
        bindings: &mut Vec<wgpu::BindGroupEntry<'a>>,
    );
}

impl<T: Fn(&Renderer, &mut Vec<wgpu::BindGroupEntry>)> IntoBindGroup for T {
    fn into_bind_group(&self, data: &Renderer, bindings: &mut Vec<wgpu::BindGroupEntry>) {
        self(data, bindings);
    }
}

pub struct Material {
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) standard_pipeline: wgpu::RenderPipeline,
    pub(crate) main_camera_pipeline: wgpu::RenderPipeline,
    pub(crate) instances: Vec<MaterialInstance>,
}

impl Material {
    pub fn default_shader(renderer: &Renderer) -> Self {
        let bind_group_layout =
            renderer
                .device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Default Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let shader = renderer
            .device()
            .create_shader_module(wgpu::include_wgsl!("../assets/default_shader.wgsl"));
        let pipeline_layout =
            renderer
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Default Pipeline Layout"),
                    bind_group_layouts: &[
                        renderer.lighting_bind_group_layout(),
                        renderer.camera_bind_group_layout(),
                        &bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let standard_pipeline =
            renderer
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Default Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::desc(), GeometryInstance::desc()],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: renderer.config().format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

        let main_camera_pipeline =
            renderer
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Default Render Pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::desc(), GeometryInstance::desc()],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[
                            Some(wgpu::ColorTargetState {
                                format: renderer.config().format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL,
                            }),
                            Some(wgpu::ColorTargetState {
                                format: renderer.config().format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL,
                            }),
                        ],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

        Self {
            standard_pipeline,
            main_camera_pipeline,
            bind_group_layout,
            instances: Vec::new(),
        }
    }

    pub fn instantiate(
        renderer: &mut Renderer,
        material_id: usize,
        entries: &dyn IntoBindGroup,
        label: &str,
    ) -> usize {
        let mut bindings = Vec::new();
        entries.into_bind_group(&renderer, &mut bindings);
        let instance = MaterialInstance::new(
            renderer.device(),
            renderer.material(material_id),
            label,
            bindings.as_slice(),
        );
        let material = renderer.material_mut(material_id);
        material.instances.push(instance);
        material.instances.len() - 1
    }
}

pub struct MaterialInstance {
    pub(crate) bind_group: wgpu::BindGroup,
}

impl MaterialInstance {
    pub(crate) fn new(
        device: &wgpu::Device,
        material: &Material,
        label: &str,
        entries: &[wgpu::BindGroupEntry],
    ) -> MaterialInstance {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &material.bind_group_layout,
            entries,
            label: Some(label),
        });

        MaterialInstance { bind_group }
    }
}
