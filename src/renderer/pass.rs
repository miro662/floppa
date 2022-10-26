use crate::TextureID;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PassDescriptor {
    pub texture_id: TextureID,
}
