use std::mem;
use wgpu::include_wgsl;

#[derive(Debug)]
pub(in crate::renderer) struct Pipeline {
    pub(in crate::renderer) pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub(in crate::renderer) fn create(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
    ) -> Pipeline {
        let shader_descriptor = include_wgsl!("shader.wgsl");
        let shader = device.create_shader_module(shader_descriptor);

        let layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };
        let layout = device.create_pipeline_layout(&layout_descriptor);

        let vertex_buf_desc = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
        };

        let instances_buf_desc = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![2 => Float32x3, 3 => Float32x3, 4 => Float32x3],
        };

        let fragment_targets = [Some(wgpu::ColorTargetState {
            format: target_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
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
        Pipeline { pipeline }
    }
}
