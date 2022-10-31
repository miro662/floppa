use crate::renderer::sprite::Sprite;
use crate::renderer::Renderer;

pub struct Assets {
    pub ball: Sprite,
}

impl Assets {
    pub fn load(renderer: &mut Renderer) -> Assets {
        Assets {
            ball: renderer.load_sprite("sprites/ball.png").unwrap(),
        }
    }
}
