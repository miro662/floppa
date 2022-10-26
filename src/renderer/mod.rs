mod camera;
pub mod color;
mod instances;
mod pipeline;
mod sprite_buffers;
mod texture;

use crate::renderer::camera::Camera;
use pollster::FutureExt;
use std::iter;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::renderer::color::Color;
use crate::renderer::instances::Instance;
use crate::renderer::pipeline::Pipeline;
use crate::renderer::sprite_buffers::SpriteBuffers;
use crate::renderer::texture::Texture;

#[derive(Debug)]
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    target_surface: wgpu::Surface,
    sprite_buffers: SpriteBuffers,
    pipeline: Pipeline,
    camera: Camera,
    textures: Vec<Texture>,
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
        let pipeline = Pipeline::create(&device, config.format);
        let camera = Camera::new(&device, screen_size, &pipeline.bind_group_layouts.camera);

        Renderer {
            device,
            queue,
            target_surface: surface,
            sprite_buffers,
            pipeline,
            textures: vec![],
            camera,
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

    pub fn load_texture(&mut self, file_path: &str) -> usize {
        let texture = Texture::load_from_file(
            file_path,
            &self.device,
            &self.queue,
            &self.pipeline.bind_group_layouts.texture,
        );
        self.textures.push(texture);
        self.textures.len() - 1
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

    pub fn draw_sprite(&mut self, texture_id: usize, x: i32, y: i32) {
        let texture = &self.renderer.textures[texture_id];
        self.instances.push(Instance {
            position: (x as f32, y as f32).into(),
            size: (texture.size.x as f32, texture.size.y as f32).into(),
            texture: texture_id,
        })
    }

    fn get_instances_buffer(&self, texture_id: usize) -> (wgpu::Buffer, usize) {
        let raw_instances = self
            .instances
            .iter()
            .filter(|i| i.texture == texture_id)
            .map(|i| i.to_raw())
            .collect::<Vec<_>>();
        let instances_buffer_desc = wgpu::util::BufferInitDescriptor {
            label: Some("Instances Buffer"),
            contents: bytemuck::cast_slice(&raw_instances),
            usage: wgpu::BufferUsages::VERTEX,
        };
        (
            self.renderer
                .device
                .create_buffer_init(&instances_buffer_desc),
            raw_instances.len(),
        )
    }

    fn render_pass(&self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        for texture_id in 0..self.renderer.textures.len() {
            let (instances_buffer, no_of_instances) = self.get_instances_buffer(texture_id);
            if no_of_instances > 0 {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Example render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: if texture_id == 0 {
                                wgpu::LoadOp::Clear(self.clear_color.into())
                            } else {
                                wgpu::LoadOp::Load
                            },
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&self.renderer.pipeline.pipeline);
                render_pass.set_bind_group(0, &self.renderer.camera.bind_group, &[]);
                render_pass.set_bind_group(1, &self.renderer.textures[texture_id].bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.renderer.sprite_buffers.vertex.slice(..));
                render_pass.set_vertex_buffer(1, instances_buffer.slice(..));
                render_pass.set_index_buffer(
                    self.renderer.sprite_buffers.index.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.draw_indexed(0..6, 0, 0..no_of_instances as _);
            }
        }
    }
}
