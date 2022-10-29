use std::mem;
use wgpu::include_wgsl;

#[derive(Debug)]
pub(in crate::renderer) struct BindGroupLayouts {
    pub(in crate::renderer) camera: wgpu::BindGroupLayout,
    pub(in crate::renderer) texture: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    fn create(device: &wgpu::Device) -> BindGroupLayouts {
        let camera = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Bind group layout 0 - Camera"),
        });

        let texture = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            label: Some("Bind group layout 1 - Texture"),
        });

        BindGroupLayouts { camera, texture }
    }

    fn as_array(&self) -> [&wgpu::BindGroupLayout; 2] {
        [&self.camera, &self.texture]
    }
}

#[derive(Debug)]
pub(in crate::renderer) struct Pipeline {
    pub(in crate::renderer) pipeline: wgpu::RenderPipeline,
    pub(in crate::renderer) bind_group_layouts: BindGroupLayouts,
}

impl Pipeline {
    pub(in crate::renderer) fn create(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
    ) -> Pipeline {
        let shader_descriptor = include_wgsl!("shader.wgsl");
        let shader = device.create_shader_module(shader_descriptor);

        let bind_group_layouts = BindGroupLayouts::create(device);
        let layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &bind_group_layouts.as_array(),
            push_constant_ranges: &[],
        };
        let layout = device.create_pipeline_layout(&layout_descriptor);

        let vertex_buf_desc = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
        };

        let instances_buf_desc = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<[f32; 13]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![2 => Float32x3, 3 => Float32x3, 4 => Float32x3, 5 => Float32x2, 6 => Float32x2],
        };

        let blend_state = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent::OVER,
        };

        let fragment_targets = [Some(wgpu::ColorTargetState {
            format: target_format,
            blend: Some(blend_state),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex",
                buffers: &[vertex_buf_desc, instances_buf_desc],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment",
                targets: &fragment_targets,
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        };
        let pipeline = device.create_render_pipeline(&pipeline_descriptor);
        Pipeline {
            pipeline,
            bind_group_layouts,
        }
    }
}
