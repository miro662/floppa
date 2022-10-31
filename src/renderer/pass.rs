use crate::renderer::Layer;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PassDescriptor {
    pub texture_id: Option<usize>,
    pub layer: Layer,
}
