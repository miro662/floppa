mod camera;
pub mod color;
mod instances;
mod pass;
mod pipeline;
pub mod sprite;
mod sprite_buffers;
mod texture;

use crate::renderer::camera::Camera;
use pollster::FutureExt;
use std::collections::{HashMap, HashSet};
use std::iter;
use std::rc::Rc;
use wgpu::util::DeviceExt;
use wgpu::{CommandEncoder, TextureView};
use winit::window::Window;

use crate::renderer::color::Color;
use crate::renderer::instances::Instance;
use crate::renderer::pass::PassDescriptor;
use crate::renderer::pipeline::Pipeline;
use crate::renderer::sprite::Sprite;
use crate::renderer::sprite_buffers::SpriteBuffers;
pub use crate::renderer::texture::Texture;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Layer(pub isize);

pub type TextureRef = Rc<Texture>;

#[derive(Debug)]
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    target_surface: wgpu::Surface,
    sprite_buffers: SpriteBuffers,
    pipeline: Pipeline,
    camera: Camera,
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
            camera,
        }
    }

    pub fn render(&mut self, render_closure: impl Fn(&mut RenderContext) -> ()) {
        let mut ctx = RenderContext {
            renderer: self,
            clear_color: Color::default(),
            instances: vec![],
            textures: HashMap::new(),
        };
        render_closure(&mut ctx);
        ctx.render();
    }

    pub fn load_texture(&mut self, file_path: &str, id: usize) -> TextureRef {
        let texture = Texture::load_from_file(
            file_path,
            &self.device,
            &self.queue,
            &self.pipeline.bind_group_layouts.texture,
            id,
        );
        Rc::new(texture)
    }
}

#[derive(Debug)]
pub struct RenderContext<'a> {
    renderer: &'a Renderer,
    clear_color: Color,
    instances: Vec<Instance>,
    textures: HashMap<usize, TextureRef>,
}

impl<'a> RenderContext<'a> {
    #[allow(dead_code)]
    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, position: cgmath::Vector2<i32>, layer: Layer) {
        let texture = &sprite.texture;
        if !self.textures.contains_key(&texture.id) {
            self.textures.insert(texture.id, texture.clone());
        }

        self.instances.push(Instance {
            position: (position.x as f32, position.y as f32).into(),
            size: (texture.size.x as f32, texture.size.y as f32).into(),
            texture_id: texture.id,
            layer,
        })
    }

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

        let pass_descriptors: HashSet<_> = self
            .instances
            .iter()
            .map(|i| i.to_pass_descriptor())
            .collect();
        let mut sorted_descriptors: Vec<_> = pass_descriptors.iter().collect();
        sorted_descriptors.sort_by_key(|pd| pd.layer);

        for (id, pass_descriptor) in sorted_descriptors.iter().enumerate() {
            self.encode_pass(&view, &mut encoder, &pass_descriptor, id);
        }

        self.renderer.queue.submit(iter::once(encoder.finish()));
        output.present();
    }

    fn encode_pass(
        &self,
        view: &TextureView,
        encoder: &mut CommandEncoder,
        pass_descriptor: &PassDescriptor,
        id: usize,
    ) {
        let (instances_buffer, instances_count) = self.get_raw_instances_for_pass(pass_descriptor);
        let load_op: wgpu::LoadOp<wgpu::Color> = if id == 0 {
            wgpu::LoadOp::Clear(self.clear_color.into())
        } else {
            wgpu::LoadOp::Load
        };
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(&format!("Render pass {}", id)),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: load_op,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.renderer.pipeline.pipeline);
        render_pass.set_bind_group(0, &self.renderer.camera.bind_group, &[]);
        render_pass.set_bind_group(
            1,
            &self.textures[&pass_descriptor.texture_id].bind_group,
            &[],
        );
        render_pass.set_vertex_buffer(0, self.renderer.sprite_buffers.vertex.slice(..));
        render_pass.set_vertex_buffer(1, instances_buffer.slice(..));
        render_pass.set_index_buffer(
            self.renderer.sprite_buffers.index.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..6, 0, 0..instances_count as _);
    }

    fn get_raw_instances_for_pass(
        &self,
        pass_descriptor: &PassDescriptor,
    ) -> (wgpu::Buffer, usize) {
        let raw_instances = self
            .instances
            .iter()
            .filter(|i| i.to_pass_descriptor() == *pass_descriptor)
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
}
