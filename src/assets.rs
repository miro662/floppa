use crate::renderer::sprite::Sprite;
use crate::renderer::Renderer;
use crate::renderer_ext::sprite::{GridMode, SpriteExt};

pub struct Assets {
    pub ball: Sprite,
    pub palette: Vec<Sprite>,
}

impl Assets {
    pub fn load(renderer: &mut Renderer) -> Assets {
        Assets {
            ball: renderer.load_sprite("sprites/ball.png").unwrap(),
            palette: renderer
                .load_sprite("sprites/palette.png")
                .unwrap()
                .uniform_grid(GridMode::CellSize((16, 16).into())),
        }
    }
}
