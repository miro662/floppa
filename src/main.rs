use crate::renderer::{color, Renderer};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod renderer;

fn main() {
    env_logger::init();

    let ev_loop = EventLoop::new();
    let window = Window::new(&ev_loop).unwrap();
    let mut renderer = Renderer::new(&window);
    // let pope = renderer.load_sprite();

    ev_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event, window_id, ..
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::ExitWithCode(0);
            }
            _ => (),
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => renderer.render(|ctx| {
            ctx.set_clear_color(color::BLACK);

            for x in 0..8 {
                for y in 0..8 {
                    if (x + y) % 2 == 0 {
                        let xx = (x as f32 - 4.0) * 0.25;
                        let yy = (y as f32 - 4.0) * 0.25;
                        ctx.draw_rect(xx, yy, 0.25, 0.25);
                        // ctx.draw_sprite(21, 37, &pope);
                    }
                }
            }
        }),
        _ => (),
    });
}
