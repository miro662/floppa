use crate::assets::Assets;
use crate::collisions::Bounds;
use crate::game::SCREEN_SIZE;
use crate::input::{Input, InputAxis};
use crate::renderer::{color, Layer, RenderContext};
use cgmath::Vector2;

const PALETTE_CELL_SIZE: Vector2<i32> = Vector2 { x: 16, y: 16 };
const PALETTE_SIZE: i32 = 10;
const INITIAL_PALETTE_POSITION: Vector2<i32> = Vector2 {
    x: SCREEN_SIZE.x / 2,
    y: 16,
};
const PALETTE_VELOCITY: Vector2<i32> = Vector2 { x: 3, y: 0 };

pub struct Palette {
    position: Vector2<i32>,
    size: i32,
}

impl Palette {
    pub fn new() -> Palette {
        Palette {
            position: INITIAL_PALETTE_POSITION,
            size: PALETTE_SIZE,
        }
    }

    fn size(&self) -> Vector2<i32> {
        (PALETTE_CELL_SIZE.x * self.size, PALETTE_CELL_SIZE.y).into()
    }

    pub fn update(&mut self, input: &mut Input) {
        self.handle_movement(input);
        self.constrain_position();
    }

    fn handle_movement(&mut self, input: &mut Input) {
        if input.get_axis(&InputAxis::PaletteLeft) {
            self.position -= PALETTE_VELOCITY;
        }
        if input.get_axis(&InputAxis::PaletteRight) {
            self.position += PALETTE_VELOCITY;
        }
    }

    fn constrain_position(&mut self) {
        if self.position.x < 0 {
            self.position.x = 0;
        } else if self.position.x > (SCREEN_SIZE.x - self.size().x) {
            self.position.x = SCREEN_SIZE.x - self.size().x;
        }
    }

    pub fn render(&self, ctx: &mut RenderContext, assets: &Assets) {
        let initial_position: Vector2<i32> =
            (self.position.x, self.position.y).into();
        let tile_offset: Vector2<i32> = (PALETTE_CELL_SIZE.x, 0).into();
        for i in 0..self.size {
            let sprite = match i {
                0 => &assets.palette[0],                         // first tile
                i if i == (self.size - 1) => &assets.palette[2], // last tile
                _ => &assets.palette[1],                         // tile in the middle
            };
            let position = initial_position + i * tile_offset;
            ctx.draw_sprite(sprite, position, Layer(0), color::WHITE)
        }
    }

    pub fn bounds(&self) -> Bounds {
        Bounds {
            position: self.position,
            size: self.size(),
        }
    }
}
