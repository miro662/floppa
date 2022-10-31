use crate::renderer::sprite::Sprite;
use crate::renderer::Renderer;

pub struct Assets {
    pub wall: Sprite,
}

impl Assets {
    pub fn load(renderer: &mut Renderer) -> Assets {
        Assets {
            wall: renderer.load_sprite("sprites/wall.png").unwrap(),
        }
    }
}
