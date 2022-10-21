pub mod color;
mod instances;
mod pipeline;
mod sprite_buffers;

use cgmath::SquareMatrix;
use pollster::FutureExt;
use std::iter;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::renderer::color::Color;
use crate::renderer::instances::{Instance, InstanceRaw};
use crate::renderer::pipeline::Pipeline;
use crate::renderer::sprite_buffers::SpriteBuffers;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj_matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    fn from_screen_size(screen_size: cgmath::Vector2<u32>) -> CameraUniform {
        let translation_matrix = cgmath::Matrix4::from_translation((-1.0, -1.0, 0.0).into());
        let two_scale_matrix = cgmath::Matrix4::from_scale(2.0);
        let res_scale_matrix = cgmath::Matrix4::from_nonuniform_scale(
            1.0 / (screen_size.x as f32),
            1.0 / (screen_size.y as f32),
            1.0,
        );
        let matrix = translation_matrix * two_scale_matrix * res_scale_matrix;
        CameraUniform {
            view_proj_matrix: matrix.into(),
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    target_surface: wgpu::Surface,
    sprite_buffers: SpriteBuffers,
    pipeline: Pipeline,
}

impl Renderer {
    pub fn new(window: &Window) -> Renderer {
        let backends = wgpu::Backends::all();
        let instance = wgpu::Instance::new(backends);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("GPU"),
                    features: wgpu::Features::empty(),
                    limits: Default::default(),
                },
                None,
            )
            .block_on()
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: Default::default(),
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let sprite_buffers = SpriteBuffers::create(&device, Some("Sprite"));
        let screen_size = (config.width, config.height).into();
        let pipeline = Pipeline::create(&device, config.format, screen_size);

        Renderer {
            device,
            queue,
            target_surface: surface,
            sprite_buffers,
            pipeline,
        }
    }

    pub fn render(&mut self, render_closure: impl FnOnce(&mut RenderContext) -> ()) {
        let mut ctx = RenderContext {
            renderer: self,
            clear_color: Color::default(),
            instances: vec![],
        };
        render_closure(&mut ctx);
        ctx.render();
    }
}

#[derive(Debug)]
pub struct RenderContext<'a> {
    renderer: &'a Renderer,
    clear_color: Color,
    instances: Vec<Instance>,
}

impl<'a> RenderContext<'a> {
    fn render(&self) {
        let output = self.renderer.target_surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.renderer
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command encoder"),
                });

        self.render_pass(&view, &mut encoder);
        self.renderer.queue.submit(iter::once(encoder.finish()));
        output.present();
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.instances.push(Instance {
            position: (x as f32, y as f32).into(),
            size: (w as f32, h as f32).into(),
        })
    }

    fn get_instances_buffer(&self) -> wgpu::Buffer {
        let raw_instances = self
            .instances
            .iter()
            .map(|i| i.to_raw())
            .collect::<Vec<_>>();
        let instances_buffer_desc = wgpu::util::BufferInitDescriptor {
            label: Some("Instances Buffer"),
            contents: bytemuck::cast_slice(&raw_instances),
            usage: wgpu::BufferUsages::VERTEX,
        };
        self.renderer
            .device
            .create_buffer_init(&instances_buffer_desc)
    }

    fn render_pass(&self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let instances_buffer = self.get_instances_buffer();
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Example render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color.into()),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.renderer.pipeline.pipeline);
        render_pass.set_bind_group(0, &self.renderer.pipeline.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.renderer.sprite_buffers.vertex.slice(..));
        render_pass.set_vertex_buffer(1, instances_buffer.slice(..));
        render_pass.set_index_buffer(
            self.renderer.sprite_buffers.index.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..6, 0, 0..self.instances.len() as _)
    }
}
