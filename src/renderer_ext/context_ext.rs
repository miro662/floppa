use crate::renderer::color::Color;
use crate::renderer::{Layer, RenderContext};
use crate::renderer_ext::bitmap_font::{BitmapFont, TextAlignment};
use cgmath::Vector2;

pub trait RenderContextExt {
    fn draw_text(
        &mut self,
        font: &BitmapFont,
        alignment: &TextAlignment,
        padding: Vector2<i32>,
        text: &str,
        position: Vector2<i32>,
        layer: Layer,
        color: Color,
    );
}

impl<'a> RenderContextExt for RenderContext<'a> {
    fn draw_text(
        &mut self,
        font: &BitmapFont,
        alignment: &TextAlignment,
        padding: Vector2<i32>,
        text: &str,
        position: Vector2<i32>,
        layer: Layer,
        color: Color,
    ) {
        font.draw_text(self, text, position, layer, alignment, padding, color);
    }
}
