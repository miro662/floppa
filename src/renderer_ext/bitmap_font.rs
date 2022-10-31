use crate::renderer::color;
use crate::renderer::color::Color;
use crate::renderer_ext::bitmap_font::MissingCharacterBehaviour::{Panic, Skip};
use crate::renderer_ext::bitmap_font::SpaceBehaviour::TreatAsCharacter;
use crate::{Layer, RenderContext, Renderer, Sprite};
use cgmath::Vector2;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub enum SpaceBehaviour {
    #[default]
    TreatAsCharacter,
    SizedEmptySpace(i32),
    DrawOtherCharacter(char),
}

#[derive(Debug, Clone, Default)]
pub enum MissingCharacterBehaviour {
    #[default]
    Panic,
    Skip,
    DrawOtherCharacter(char),
}

#[derive(Debug, Clone, Default)]
pub struct BitmapFontSettings {
    space_behaviour: SpaceBehaviour,
    missing_character_behaviour: MissingCharacterBehaviour,
}

#[derive(Debug, Clone, Default)]
pub enum TextAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct BitmapFont {
    characters_map: HashMap<char, Sprite>,
    settings: BitmapFontSettings,
}

impl BitmapFont {
    pub fn new(
        sprites: &[Sprite],
        characters_list: impl Iterator<Item = char>,
        settings: BitmapFontSettings,
    ) -> BitmapFont {
        let mut characters_map = HashMap::new();
        for (character, sprite) in characters_list.zip(sprites.iter()) {
            characters_map.insert(character, sprite.clone());
        }
        BitmapFont {
            characters_map,
            settings,
        }
    }

    fn get_real_char(&self, character: char) -> (Option<char>, i32) {
        // handle space behaviour
        let (space_replaced_ch, space_skip_draw) = if character == ' ' {
            match self.settings.space_behaviour {
                SpaceBehaviour::TreatAsCharacter => (' ', None),
                SpaceBehaviour::SizedEmptySpace(space) => (' ', Some(space)),
                SpaceBehaviour::DrawOtherCharacter(ch) => (ch, None),
            }
        } else {
            (character, None)
        };
        if let Some(space) = space_skip_draw {
            return (None, space);
        }

        let real_ch = if self.characters_map.contains_key(&space_replaced_ch) {
            Some(space_replaced_ch)
        } else {
            match self.settings.missing_character_behaviour {
                Panic => panic!("Missing character {}", space_replaced_ch),
                Skip => None,
                MissingCharacterBehaviour::DrawOtherCharacter(ch) => {
                    if self.characters_map.contains_key(&ch) {
                        Some(ch)
                    } else {
                        panic!("Missing replacement character {}", ch)
                    }
                }
            }
        };

        if let Some(ch) = real_ch {
            let size = self.characters_map[&ch].get_size().x as i32;
            (Some(ch), size)
        } else {
            (None, 0)
        }
    }

    fn draw_char(
        &self,
        ctx: &mut RenderContext,
        character: char,
        position: Vector2<i32>,
        layer: Layer,
        color: Color,
    ) -> i32 {
        let (real_ch, space) = self.get_real_char(character);
        if let Some(ch) = real_ch {
            let sprite = &self.characters_map[&ch];
            ctx.draw_sprite(sprite, position, layer, color);
        }
        space
    }

    fn draw_line(
        &self,
        ctx: &mut RenderContext,
        line: &str,
        position: Vector2<i32>,
        layer: Layer,
        alignment: &TextAlignment,
        padding: i32,
        color: Color,
    ) {
        use TextAlignment::*;
        let string_size: i32 = line.chars().map(|ch| self.get_real_char(ch).1).sum();
        let x_position = position.x
            - match alignment {
                Left => 0,
                Center => string_size / 2,
                Right => string_size,
            };
        let mut current_position = (x_position, position.y).into();

        for ch in line.chars() {
            let offset = self.draw_char(ctx, ch, current_position, layer, color);
            current_position.x += offset + padding * 2;
        }
    }

    pub(in crate::renderer_ext) fn draw_text(
        &self,
        ctx: &mut RenderContext,
        text: &str,
        position: Vector2<i32>,
        layer: Layer,
        alignment: &TextAlignment,
        padding: Vector2<i32>,
        color: Color,
    ) {
        let mut current_position = position;
        let lines = text.lines();
        let line_height = text
            .chars()
            .next()
            .and_then(|ch| self.characters_map.get(&ch))
            .map_or(0, |ch| ch.get_size().y) as i32;
        for line in lines {
            self.draw_line(
                ctx,
                line,
                current_position,
                layer,
                alignment,
                padding.x,
                color,
            );
            current_position.y -= line_height + padding.y * 2;
        }
    }
}
