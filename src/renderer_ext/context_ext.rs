use crate::{BitmapFont, Layer, RenderContext, TextAlignment};
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
    ) {
        font.draw_text(self, text, position, layer, alignment, padding);
    }
}
