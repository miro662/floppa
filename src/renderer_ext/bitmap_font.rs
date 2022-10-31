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

    fn draw_char(
        &self,
        ctx: &mut RenderContext,
        character: char,
        position: Vector2<i32>,
        layer: Layer,
    ) -> i32 {
        use MissingCharacterBehaviour::*;
        use SpaceBehaviour::*;

        let (space_replaced_ch, space_skip_draw) = if character == ' ' {
            match self.settings.space_behaviour {
                TreatAsCharacter => (' ', None),
                SizedEmptySpace(space) => (' ', Some(space)),
                SpaceBehaviour::DrawOtherCharacter(ch) => (ch, None),
            }
        } else {
            (character, None)
        };
        if let Some(space) = space_skip_draw {
            return space;
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
            self.draw_char_raw(ctx, ch, position, layer)
        } else {
            0
        }
    }

    fn draw_char_raw(
        &self,
        ctx: &mut RenderContext,
        character: char,
        position: Vector2<i32>,
        layer: Layer,
    ) -> i32 {
        let sprite = &self.characters_map[&character];
        ctx.draw_sprite(sprite, position, layer);
        sprite.get_size().x as i32
    }

    pub fn draw_text(
        &self,
        ctx: &mut RenderContext,
        text: &str,
        position: Vector2<i32>,
        layer: Layer,
    ) {
        let mut current_position = position;
        for ch in text.chars() {
            let offset = self.draw_char(ctx, ch, current_position, layer);
            current_position.x += offset;
        }
    }
}
