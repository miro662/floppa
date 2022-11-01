use crate::input::InputAxis::{PaletteLeft, PaletteRight};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum InputAxis {
    PaletteLeft,
    PaletteRight,
}

impl InputAxis {
    fn axes_list() -> impl Iterator<Item = InputAxis> {
        vec![PaletteLeft, PaletteRight].into_iter()
    }
    fn map_keycode(keycode: &VirtualKeyCode) -> Option<InputAxis> {
        match keycode {
            VirtualKeyCode::A => Some(PaletteLeft),
            VirtualKeyCode::D => Some(PaletteRight),
            _ => None,
        }
    }
}

pub struct Input {
    axes: HashMap<InputAxis, bool>,
}

impl Input {
    pub fn new() -> Input {
        let axes: HashMap<_, _> = InputAxis::axes_list().map(|axis| (axis, false)).collect();
        Input { axes }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        if let WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(key_code),
                    state,
                    ..
                },
            ..
        } = event
        {
            if let Some(axis) = InputAxis::map_keycode(key_code) {
                let state = match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
                *self.axes.get_mut(&axis).unwrap() = state;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn get_axis(&self, axis: &InputAxis) -> bool {
        self.axes[&axis]
    }
}
