use crate::assets::Assets;
use crate::main_loop::Game;
use crate::renderer::{color, Layer, Renderer};
use winit::event::WindowEvent;
use winit::window::Window;

pub struct Arkanoid {
    renderer: Renderer,
    assets: Assets,
}

impl Game for Arkanoid {
    fn create(window: &Window) -> Self {
        let mut renderer = Renderer::new(&window).unwrap();
        let assets = Assets::load(&mut renderer);
        Arkanoid { renderer, assets }
    }

    fn steps_per_frame(&self) -> u32 {
        60
    }

    fn handle_event(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        // todo!()
    }

    fn render(&mut self) {
        self.renderer
            .render(|ctx| {
                ctx.set_clear_color(color::YELLOW);
                ctx.draw_sprite(&self.assets.wall, (0, 0).into(), Layer(0), color::WHITE);
            })
            .unwrap()
    }
}
