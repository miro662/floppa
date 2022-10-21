use crate::renderer::{color, Renderer};
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

#[derive(Debug)]
struct State {
    palette_1_position: cgmath::Vector2<i32>,
    q_pressed: bool,
    a_pressed: bool,
    palette_1_points: u32,

    palette_2_position: cgmath::Vector2<i32>,
    o_pressed: bool,
    l_pressed: bool,
    palette_2_points: u32,

    ball_position: cgmath::Vector2<i32>,
    ball_velocity: cgmath::Vector2<i32>,
}

impl State {
    fn new() -> State {
        State {
            palette_1_position: (BALL_SIZE, (600 - PALETTE_SIZE) / 2).into(),
            q_pressed: false,
            a_pressed: false,
            palette_1_points: 0,

            palette_2_position: (800 - BALL_SIZE * 2, (600 - PALETTE_SIZE) / 2).into(),
            o_pressed: false,
            l_pressed: false,
            palette_2_points: 0,

            ball_position: (400 - BALL_SIZE / 2, 300 - BALL_SIZE / 2).into(),
            ball_velocity: (BALL_VELOCITY, BALL_VELOCITY).into(),
        }
    }

    fn update(&mut self) {
        let palette_1_translation = match (self.q_pressed, self.a_pressed) {
            (true, _) => (0, PALETTE_VELOCITY).into(),
            (_, true) => (0, -PALETTE_VELOCITY).into(),
            (false, false) => (0, 0).into(),
        };
        self.palette_1_position += palette_1_translation;

        let palette_2_translation = match (self.o_pressed, self.l_pressed) {
            (true, _) => (0, PALETTE_VELOCITY).into(),
            (_, true) => (0, -PALETTE_VELOCITY).into(),
            (false, false) => (0, 0).into(),
        };
        self.palette_2_position += palette_2_translation;

        self.ball_position += self.ball_velocity;
        if self.ball_position.y <= 0 || self.ball_position.y >= (600 - BALL_SIZE) {
            self.ball_velocity.y = -self.ball_velocity.y
        }

        let left_boud = 2 * BALL_SIZE;
        if self.ball_position.x <= left_boud
            && self.ball_position.y >= left_boud - BALL_VELOCITY
            && self.ball_position.y >= self.palette_1_position.y - BALL_SIZE
            && self.ball_position.y <= self.palette_1_position.y + PALETTE_SIZE
        {
            self.ball_velocity.x = -self.ball_velocity.x
        }

        let right_bound = 800 - 3 * BALL_SIZE;
        if self.ball_position.x >= right_bound
            && self.ball_position.x <= right_bound + BALL_VELOCITY
            && self.ball_position.y >= self.palette_2_position.y - BALL_SIZE
            && self.ball_position.y <= self.palette_2_position.y + PALETTE_SIZE
        {
            self.ball_velocity.x = -self.ball_velocity.x
        }

        if self.ball_position.x < -BALL_SIZE {
            self.ball_position = (400 - BALL_SIZE / 2, 300 - BALL_SIZE / 2).into();
            self.palette_1_position.y = (600 - PALETTE_SIZE) / 2;
            self.palette_2_position.y = (600 - PALETTE_SIZE) / 2;
            self.palette_2_points += 1;
        }

        if self.ball_position.x > 800 {
            self.ball_position = (400 - BALL_SIZE / 2, 300 - BALL_SIZE / 2).into();
            self.palette_1_position.y = (600 - PALETTE_SIZE) / 2;
            self.palette_2_position.y = (600 - PALETTE_SIZE) / 2;
            self.palette_1_points += 1;
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Q),
                    ..
                } => {
                    self.q_pressed = true;
                    true
                }
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Q),
                    ..
                } => {
                    self.q_pressed = false;
                    true
                }
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::A),
                    ..
                } => {
                    self.a_pressed = true;
                    true
                }
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::A),
                    ..
                } => {
                    self.a_pressed = false;
                    true
                }
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::O),
                    ..
                } => {
                    self.o_pressed = true;
                    true
                }
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::O),
                    ..
                } => {
                    self.o_pressed = false;
                    true
                }
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::L),
                    ..
                } => {
                    self.l_pressed = true;
                    true
                }
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::L),
                    ..
                } => {
                    self.l_pressed = false;
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn render(&self, ctx: &mut renderer::RenderContext) {
        // ball
        ctx.draw_rect(
            self.ball_position.x,
            self.ball_position.y,
            BALL_SIZE as u32,
            BALL_SIZE as u32,
        );

        // palettes
        ctx.draw_rect(
            self.palette_1_position.x,
            self.palette_1_position.y,
            BALL_SIZE as u32,
            PALETTE_SIZE as u32,
        );
        ctx.draw_rect(
            self.palette_2_position.x,
            self.palette_2_position.y,
            BALL_SIZE as u32,
            PALETTE_SIZE as u32,
        );

        for i in 0..self.palette_1_points {
            let x = POINTS_SIZE + (i as i32) * (POINTS_SIZE + POINTS_MARGIN);
            ctx.draw_rect(
                x,
                600 - 2 * POINTS_SIZE,
                POINTS_SIZE as u32,
                POINTS_SIZE as u32,
            );
        }

        for i in 0..self.palette_2_points {
            let x = 800 - (2 * POINTS_SIZE + (i as i32) * (POINTS_SIZE + POINTS_MARGIN));
            ctx.draw_rect(
                x,
                600 - 2 * POINTS_SIZE,
                POINTS_SIZE as u32,
                POINTS_SIZE as u32,
            );
        }
    }
}

fn main() {
    env_logger::init();

    let ev_loop = EventLoop::new();
    let window = Window::new(&ev_loop).unwrap();
    window.set_inner_size(LogicalSize::new(800, 600));
    let mut renderer = Renderer::new(&window);
    let mut state = State::new();

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
            while (duration >= step_duration) {
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
