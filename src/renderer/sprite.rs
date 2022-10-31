use crate::renderer::TextureRef;
use cgmath::Vector2;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub(in crate::renderer) texture: TextureRef,
    pub(in crate::renderer) size: Vector2<u32>,
    pub(in crate::renderer) offset: Vector2<u32>,
}

impl Sprite {
    pub fn from_whole_texture(texture: &TextureRef) -> Sprite {
        Sprite {
            texture: texture.clone(),
            offset: (0, 0).into(),
            size: texture.size,
        }
    }

    pub fn slice(&self, size: Vector2<u32>, offset: Vector2<u32>) -> Sprite {
        Sprite {
            texture: self.texture.clone(),
            size,
            offset,
        }
    }

    pub fn get_size(&self) -> Vector2<u32> {
        self.size
    }

    pub fn get_offset(&self) -> Vector2<u32> {
        self.offset
    }
}
