use cgmath::Vector2;
use crate::assets::Assets;
use crate::collisions::Bounds;
use crate::game::SCREEN_SIZE;
use crate::renderer::{color, Layer, RenderContext};

const BLOCK_SIZE: Vector2<i32> = Vector2 {x: 48, y: 24};
const BLOCKS_COUNT: Vector2<i32> = Vector2 {x: 15, y: 10};
const BLOCKS_ORIGIN: Vector2<i32> = Vector2 {x: (SCREEN_SIZE.x - BLOCK_SIZE.x * BLOCKS_COUNT.x) / 2, y: 320};

#[derive(Debug, Clone)]
pub struct Block {
    position: Vector2<i32>
}

impl Block {
    pub fn new(position: Vector2<i32>) -> Block {
        Block {
            position
        }
    }

    pub fn render(&self, ctx: &mut RenderContext, assets: &Assets) {
        ctx.draw_sprite(&assets.block, self.position, Layer(0), color::WHITE);
    }

    pub fn bounds(&self) -> Bounds {
        Bounds {
            position: self.position,
            size: BLOCK_SIZE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blocks {
    blocks: Vec<Block>
}

impl Blocks {
    pub fn new() -> Blocks {
        let mut blocks = vec![];
        for x in 0..BLOCKS_COUNT.x {
            for y in 0..BLOCKS_COUNT.y {
                let offset: Vector2<i32> = (x * BLOCK_SIZE.x, y * BLOCK_SIZE.y).into();
                let position = BLOCKS_ORIGIN + offset;
                let block = Block::new(position);
                blocks.push(block);
            }
        }
        Blocks {
            blocks
        }
    }

    pub fn render(&self, ctx: &mut RenderContext, assets: &Assets) {
        for block in &self.blocks {
            block.render(ctx, assets)
        }
    }

    pub fn handle_collisions(&mut self, ball: Bounds) -> Option<Bounds> {
        let mut collision = None;
        for (i, block) in self.blocks.iter().enumerate() {
            if let Some(b) = block.bounds().overlap(&ball) {
                collision = Some((i, b));
                break;
            }
        };
        if let Some((i, _)) = collision {
            self.blocks.remove(i);
        }

        collision.map(|it| it.1)
    }
}