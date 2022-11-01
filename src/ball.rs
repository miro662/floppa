use crate::assets::Assets;
use crate::game::SCREEN_SIZE;
use crate::renderer::{color, Layer, RenderContext};
use cgmath::Vector2;

const BALL_SIZE: Vector2<i32> = Vector2 { x: 16, y: 16 };
const BALL_LAYER: Layer = Layer(0);
const BALL_INITIAL_POSITION: Vector2<i32> = Vector2 {
    x: 400 - BALL_SIZE.x / 2,
    y: 300 - BALL_SIZE.y / 2,
};
const BALL_INITIAL_VELOCITY: Vector2<i32> = Vector2 { x: 2, y: 2 };

pub struct Ball {
    position: Vector2<i32>,
    velocity: Vector2<i32>,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            position: BALL_INITIAL_POSITION,
            velocity: BALL_INITIAL_VELOCITY,
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        self.bounce_edges();
    }

    pub fn render(&self, ctx: &mut RenderContext, assets: &Assets) {
        ctx.draw_sprite(&assets.ball, self.position, BALL_LAYER, color::WHITE);
    }

    fn bounce_edges(&mut self) {
        // left edge
        let left_edge = 0;
        if self.position.x <= left_edge {
            self.position.x = -self.position.x;
            self.velocity.x = -self.velocity.x;
        }

        // right edge
        let right_edge = SCREEN_SIZE.x - BALL_SIZE.x;
        if self.position.x >= right_edge {
            self.position.x = right_edge - (self.position.x - right_edge);
            self.velocity.x = -self.velocity.x;
        }

        // bottom edge - todo: other behaviour in this case
        let bottom_edge = 0;
        if self.position.y <= bottom_edge {
            self.position.y = -self.position.y;
            self.velocity.y = -self.velocity.y;
        }

        // top edge
        let top_edge = SCREEN_SIZE.y - BALL_SIZE.y;
        if self.position.y >= top_edge {
            self.position.y = top_edge - (self.position.y - top_edge);
            self.velocity.y = -self.velocity.y;
        }
    }
}
