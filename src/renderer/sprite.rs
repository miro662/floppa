use crate::TextureRef;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub(crate) texture: TextureRef,
}

impl Sprite {
    pub fn from_whole_texture(texture: &TextureRef) -> Sprite {
        Sprite {
            texture: texture.clone(),
        }
    }
}
