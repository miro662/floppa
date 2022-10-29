use crate::TextureRef;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub(crate) texture: TextureRef,
    pub(crate) offset: cgmath::Vector2<u32>,
    pub(crate) size: cgmath::Vector2<u32>,
}

impl Sprite {
    pub fn from_whole_texture(texture: &TextureRef) -> Sprite {
        Sprite {
            texture: texture.clone(),
            offset: (0, 0).into(),
            size: texture.size,
        }
    }
}
