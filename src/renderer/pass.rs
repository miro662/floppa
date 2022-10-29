use crate::renderer::Layer;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PassDescriptor {
    pub texture_id: usize,
    pub layer: Layer,
}
