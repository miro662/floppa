use crate::renderer::pass::PassDescriptor;
use cgmath::Matrix3;

#[derive(Debug, Copy, Clone)]
pub(in crate::renderer) struct Instance {
    pub(in crate::renderer) position: cgmath::Vector2<f32>,
    pub(in crate::renderer) size: cgmath::Vector2<f32>,
    pub(in crate::renderer) texture_id: usize,
}

impl Instance {
    pub(in crate::renderer) fn to_raw(&self) -> InstanceRaw {
        let translation = Matrix3::from_translation(self.position);
        let scale = Matrix3::from_nonuniform_scale(self.size.x, self.size.y);
        let matrix = translation * scale;

        InstanceRaw {
            matrix: matrix.into(),
        }
    }
    pub(in crate::renderer) fn to_pass_descriptor(&self) -> PassDescriptor {
        PassDescriptor {
            texture_id: self.texture_id,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(in crate::renderer) struct InstanceRaw {
    matrix: [[f32; 3]; 3],
}
