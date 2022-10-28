use crate::renderer::{RenderContext, Renderer, TextureID};
use cgmath::Vector2;
use rand::prelude::*;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod renderer;

const BALL_SIZE: i32 = 32;
const STEPS_PER_SECOND: i32 = 60;
const PALETTE_VELOCITY: i32 = 4;
const BALL_VELOCITY: i32 = 3;
const PALETTE_SIZE: i32 = 4 * BALL_SIZE;
const POINTS_SIZE: i32 = 12;
const POINTS_MARGIN: i32 = 6;
const TOLERANCE: i32 = BALL_VELOCITY;

#[derive(Debug, Copy, Clone)]
struct Textures {
    ball: TextureID,
    palette: TextureID,
    point: TextureID,
}

#[derive(Debug, Copy, Clone)]
struct Bounds {
    origin: Vector2<i32>,
    size: Vector2<i32>,
}

#[derive(Debug, Copy, Clone)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
struct Palette {
    position: Vector2<i32>,
    side: Side,

    up_button: VirtualKeyCode,
    down_button: VirtualKeyCode,

    up_button_pressed: bool,
    down_button_pressed: bool,

    texture: TextureID,
}

impl Palette {
    fn update(&mut self) {
        let translation = match (self.up_button_pressed, self.down_button_pressed) {
            (true, _) => (0, PALETTE_VELOCITY).into(),
            (_, true) => (0, -PALETTE_VELOCITY).into(),
            (false, false) => (0, 0).into(),
        };
        self.position += translation;
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    virtual_keycode,
                    state,
                    ..
                } => {
                    if virtual_keycode == &Some(self.up_button) {
                        self.up_button_pressed = match state {
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        };
                        true
                    } else if virtual_keycode == &Some(self.down_button) {
                        self.down_button_pressed = match state {
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        };
                        true
                    } else {
                        false
                    }
                }
            },
            _ => false,
        }
    }

    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_sprite(self.texture, self.position);
    }

    fn overlap(&self, bounds: Bounds) -> bool {
        let horizontal_overlap = match self.side {
            Side::Left => {
                let collision_line = self.position.x + BALL_SIZE;
                let origin = bounds.origin.x;
                origin <= collision_line && origin >= collision_line - TOLERANCE
            }
            Side::Right => {
                let collision_line = self.position.x;
                let origin = bounds.origin.x + bounds.size.x;
                origin >= collision_line && origin <= collision_line + TOLERANCE
            }
        };
        let vertical_overlap = bounds.origin.y >= self.position.y - BALL_SIZE
            && bounds.origin.y <= self.position.y + PALETTE_SIZE;
        horizontal_overlap && vertical_overlap
    }

    fn restart(&mut self) {
        self.position.y = (600 - PALETTE_SIZE) / 2
    }
}

#[derive(Debug)]
struct Player {
    palette: Palette,
    score: u32,
    side: Side,
    points_texture: TextureID,
}

impl Player {
    fn new(side: Side, textures: Textures) -> Player {
        let x_palette_position = match side {
            Side::Left => BALL_SIZE,
            Side::Right => 800 - BALL_SIZE * 2,
        };
        let up_button = match side {
            Side::Left => VirtualKeyCode::Q,
            Side::Right => VirtualKeyCode::O,
        };
        let down_button = match side {
            Side::Left => VirtualKeyCode::A,
            Side::Right => VirtualKeyCode::L,
        };

        Player {
            side,
            score: 0,
            palette: Palette {
                position: (x_palette_position, (600 - PALETTE_SIZE) / 2).into(),
                side,
                up_button,
                down_button,
                up_button_pressed: false,
                down_button_pressed: false,
                texture: textures.palette,
            },
            points_texture: textures.point,
        }
    }

    fn render(&self, ctx: &mut RenderContext) {
        self.palette.render(ctx);

        for i in 0..self.score {
            let x = match self.side {
                Side::Left => POINTS_SIZE + (i as i32) * (POINTS_SIZE + POINTS_MARGIN),
                Side::Right => 800 - (2 * POINTS_SIZE + (i as i32) * (POINTS_SIZE + POINTS_MARGIN)),
            };
            ctx.draw_sprite(self.points_texture, (x, 600 - 2 * POINTS_SIZE).into());
        }
    }

    fn should_score(&self, bounds: Bounds) -> bool {
        match self.side {
            Side::Left => bounds.origin.x > 800,
            Side::Right => bounds.origin.x < -bounds.size.x,
        }
    }

    fn score(&mut self) {
        self.score += 1
    }
}

#[derive(Debug)]
struct Ball {
    position: Vector2<i32>,
    velocity: Vector2<i32>,
    rng: ThreadRng,
    texture: TextureID,
}

impl Ball {
    fn new(texture: TextureID) -> Ball {
        let mut rng = rand::thread_rng();
        let velocity = match rng.gen_range(0..=3) {
            0 => (BALL_VELOCITY, BALL_VELOCITY).into(),
            1 => (-BALL_VELOCITY, BALL_VELOCITY).into(),
            2 => (-BALL_VELOCITY, -BALL_VELOCITY).into(),
            3 => (BALL_VELOCITY, -BALL_VELOCITY).into(),
            _ => unreachable!(),
        };
        Ball {
            position: (400 - BALL_SIZE / 2, 300 - BALL_SIZE / 2).into(),
            velocity,
            rng,
            texture,
        }
    }

    fn bounds(&self) -> Bounds {
        Bounds {
            origin: self.position,
            size: (BALL_SIZE, BALL_SIZE).into(),
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;
        if self.position.y <= 0 || self.position.y >= (600 - BALL_SIZE) {
            self.velocity.y = -self.velocity.y
        }
    }

    fn bounce_horizontally(&mut self) {
        self.velocity.x = -self.velocity.x;
        match self.rng.gen_range(0..=1) {
            0 => self.velocity.x += self.velocity.x.signum(),
            1 => self.velocity.y += self.velocity.y.signum(),
            _ => unreachable!(),
        };
    }

    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_sprite(self.texture, self.position);
    }

    fn restart(&mut self) {
        self.position = (400 - BALL_SIZE / 2, 300 - BALL_SIZE / 2).into();
        self.velocity = match self.rng.gen_range(0..=3) {
            0 => (BALL_VELOCITY, BALL_VELOCITY).into(),
            1 => (-BALL_VELOCITY, BALL_VELOCITY).into(),
            2 => (-BALL_VELOCITY, -BALL_VELOCITY).into(),
            3 => (BALL_VELOCITY, -BALL_VELOCITY).into(),
            _ => unreachable!(),
        };
    }
}

#[derive(Debug)]
struct State {
    players: [Player; 2],
    ball: Ball,
}

impl State {
    fn new(textures: Textures) -> State {
        State {
            players: [
                Player::new(Side::Left, textures),
                Player::new(Side::Right, textures),
            ],
            ball: Ball::new(textures.ball),
        }
    }

    fn update(&mut self) {
        self.ball.update();

        let mut should_restart = false;
        for player in &mut self.players {
            player.palette.update();

            if player.palette.overlap(self.ball.bounds()) {
                self.ball.bounce_horizontally();
            }

            if player.should_score(self.ball.bounds()) {
                player.score();
                should_restart = true;
            }
        }
        if should_restart {
            self.restart();
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        self.players.iter_mut().any(|p| p.palette.input(&event))
    }

    fn render(&self, ctx: &mut RenderContext) {
        self.ball.render(ctx);

        for player in &self.players {
            player.render(ctx);
        }
    }

    fn restart(&mut self) {
        self.ball.restart();

        for player in &mut self.players {
            player.palette.restart()
        }
    }
}

fn main() {
    env_logger::init();

    let ev_loop = EventLoop::new();
    let window = Window::new(&ev_loop).unwrap();
    window.set_inner_size(LogicalSize::new(800, 600));
    let mut renderer = Renderer::new(&window);
    let textures = Textures {
        ball: renderer.load_texture("sprites/ball.png", 0),
        palette: renderer.load_texture("sprites/palette.png", 1),
        point: renderer.load_texture("sprites/point.png", 2),
    };
    let mut state = State::new(textures);

    let mut last_frame_finished = chrono::Utc::now();
    let step_duration = chrono::Duration::seconds(1) / STEPS_PER_SECOND;
    let mut duration = chrono::Duration::zero();

    ev_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event, window_id, ..
        } if window_id == window.id() && !state.input(&event) => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::ExitWithCode(0);
            }
            _ => (),
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let time_now = chrono::Utc::now();
            duration = duration + (time_now - last_frame_finished);
            while duration >= step_duration {
                state.update();
                duration = duration - step_duration;
            }
            renderer.render(|ctx| {
                state.render(ctx);
            });
            last_frame_finished = time_now;
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => (),
    });
}
