use chrono::{DateTime, Duration, Utc};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub trait Game {
    fn create(window: &Window) -> Self;
    fn steps_per_frame(&self) -> u32;
    fn handle_event(&mut self, event: &WindowEvent) -> bool;
    fn update(&mut self);
    fn render(&mut self);
}

struct FixedUpdater {
    last_frame_finished: DateTime<Utc>,
    step_duration: Duration,
    non_evaluated_duration: Duration,
}

pub fn run<T: Game + 'static>() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_inner_size(LogicalSize::new(800, 600));
    let mut game = <T as Game>::create(&window);

    let steps_per_frame = game.steps_per_frame();
    let mut last_frame_finished = Utc::now();
    let step_duration = Duration::seconds(1) / steps_per_frame as i32;
    let mut non_evaluated_duration = Duration::zero();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event, window_id, ..
        } if window_id == window.id() && !game.handle_event(&event) => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::ExitWithCode(0);
            }
            _ => (),
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let frame_start = Utc::now();
            let last_frame_duration = frame_start - last_frame_finished;
            non_evaluated_duration = non_evaluated_duration + last_frame_duration;
            while step_duration <= non_evaluated_duration {
                non_evaluated_duration = non_evaluated_duration - step_duration;
                game.update();
            }
            game.render();
            last_frame_finished = frame_start;
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => (),
    });
}
