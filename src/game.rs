use crate::assets::Assets;
use crate::ball::Ball;
use crate::main_loop::Game;
use crate::palette::Palette;
use crate::renderer::color::Color;
use crate::renderer::{color, Layer, Renderer};
use cgmath::Vector2;
use winit::event::WindowEvent;
use winit::window::Window;

const STEPS_PER_SECOND: u32 = 60;
pub const SCREEN_SIZE: Vector2<i32> = Vector2 { x: 800, y: 600 };

pub struct Arkanoid {
    renderer: Renderer,
    assets: Assets,

    ball: Ball,
    palette: Palette,
}

impl Game for Arkanoid {
    fn create(window: &Window) -> Self {
        let mut renderer = Renderer::new(&window).unwrap();
        let assets = Assets::load(&mut renderer);

        let ball = Ball::new();
        let palette = Palette::new();
        Arkanoid {
            renderer,
            assets,
            ball,
            palette,
        }
    }

    fn steps_per_second(&self) -> u32 {
        STEPS_PER_SECOND
    }

    fn handle_event(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        self.ball.update();
    }

    fn render(&mut self) {
        self.renderer
            .render(|ctx| {
                ctx.set_clear_color(Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                });
                self.ball.render(ctx, &self.assets);
                self.palette.render(ctx, &self.assets);
            })
            .unwrap()
    }
}
