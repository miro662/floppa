use crate::renderer::CameraUniform;
use cgmath::Vector2;
use std::mem;
use wgpu::include_wgsl;
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub(in crate::renderer) struct Pipeline {
    pub(in crate::renderer) pipeline: wgpu::RenderPipeline,
    pub(in crate::renderer) camera_bind_group: wgpu::BindGroup,
    pub(in crate::renderer) texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl Pipeline {
    pub(in crate::renderer) fn create(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        screen_size: Vector2<u32>,
    ) -> Pipeline {
        let shader_descriptor = include_wgsl!("shader.wgsl");
        let shader = device.create_shader_module(shader_descriptor);

        let mut camera_uniforms = [CameraUniform::from_screen_size(screen_size)];
        println!("{:?}", camera_uniforms);
        let camera_buffer_description = wgpu::util::BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytemuck::cast_slice(&camera_uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        };
        let camera_buffer = device.create_buffer_init(&camera_buffer_description);

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some("camera_bind_group_layout"),
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                label: Some("tbgl"),
            });

        let bind_group_layouts = [&camera_bind_group_layout, &texture_bind_group_layout];
        let layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &bind_group_layouts,
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
        Pipeline {
            texture_bind_group_layout,
            pipeline,
            camera_bind_group,
        }
    }
}
