use crate::renderer::pass::PassDescriptor;
use crate::renderer::Layer;
use cgmath::{Matrix3, Vector2};

#[derive(Debug, Copy, Clone)]
pub(in crate::renderer) struct Instance {
    pub(in crate::renderer) position: cgmath::Vector2<f32>,
    pub(in crate::renderer) texture_id: usize,
    pub(in crate::renderer) layer: Layer,

    pub(in crate::renderer) tex_size: cgmath::Vector2<u32>,
    pub(in crate::renderer) sprite_size: cgmath::Vector2<u32>,
    pub(in crate::renderer) sprite_offset: cgmath::Vector2<u32>,
}

impl Instance {
    pub(in crate::renderer) fn to_raw(&self) -> InstanceRaw {
        let tex_size: Vector2<f32> = (self.tex_size.x as f32, self.tex_size.y as f32).into();
        let sprite_offset: Vector2<f32> =
            (self.sprite_offset.x as f32, self.sprite_offset.y as f32).into();
        let sprite_size: Vector2<f32> =
            (self.sprite_size.x as f32, self.sprite_size.y as f32).into();
        let sprite_higher_bounds = sprite_offset + sprite_size;

        let translation = Matrix3::from_translation(self.position);
        let scale = Matrix3::from_nonuniform_scale(tex_size.x, tex_size.y);
        let matrix = translation * scale;

        InstanceRaw {
            matrix: matrix.into(),
            tex_lower_bounds: [sprite_offset.x / tex_size.x, sprite_offset.y / tex_size.y],
            tex_higher_bounds: [
                sprite_higher_bounds.x / tex_size.x,
                sprite_higher_bounds.y / tex_size.y,
            ],
        }
    }
    pub(in crate::renderer) fn to_pass_descriptor(&self) -> PassDescriptor {
        PassDescriptor {
            texture_id: self.texture_id,
            layer: self.layer,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(in crate::renderer) struct InstanceRaw {
    matrix: [[f32; 3]; 3],
    tex_lower_bounds: [f32; 2],
    tex_higher_bounds: [f32; 2],
}
