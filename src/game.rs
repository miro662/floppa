use crate::assets::Assets;
use crate::ball::Ball;
use crate::input::Input;
use crate::main_loop::Game;
use crate::palette::Palette;
use crate::renderer::color::Color;
use crate::renderer::{color, Layer, Renderer};
use cgmath::Vector2;
use winit::event::WindowEvent;
use winit::window::Window;
use crate::block::Blocks;

const STEPS_PER_SECOND: u32 = 60;
pub const SCREEN_SIZE: Vector2<i32> = Vector2 { x: 800, y: 600 };

pub struct Arkanoid {
    renderer: Renderer,
    assets: Assets,
    input: Input,

    ball: Ball,
    palette: Palette,
    blocks: Blocks,
}

impl Game for Arkanoid {
    fn create(window: &Window) -> Self {
        let mut renderer = Renderer::new(&window).unwrap();
        let assets = Assets::load(&mut renderer);
        let input = Input::new();

        let ball = Ball::new();
        let palette = Palette::new();
        let blocks = Blocks::new();
        Arkanoid {
            renderer,
            assets,
            input,
            ball,
            palette,
            blocks,
        }
    }

    fn steps_per_second(&self) -> u32 {
        STEPS_PER_SECOND
    }

    fn handle_event(&mut self, event: &WindowEvent) -> bool {
        self.input.handle_event(event)
    }

    fn update(&mut self) {
        self.ball.update();
        self.palette.update(&mut self.input);

        let ball_palette_collision = self.ball.bounds().overlap(&self.palette.bounds());
        if let Some(bounds) = ball_palette_collision {
            self.ball.handle_collision(&bounds);
        }

        'a: while let Some(bounds) = self.blocks.handle_collisions(self.ball.bounds()) {
            if !self.ball.handle_collision(&bounds) {
                break 'a
            }
        }
    }

    fn render(&mut self) {
        self.renderer
            .render(|ctx| {
                ctx.set_clear_color(Color {
                    r: 0.386,
                    g: 0.605,
                    b: 1.0,
                    a: 1.0,
                });
                self.ball.render(ctx, &self.assets);
                self.palette.render(ctx, &self.assets);
                self.blocks.render(ctx, &self.assets);
            })
            .unwrap()
    }
}
