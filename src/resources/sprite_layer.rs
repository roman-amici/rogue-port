use bevy_ecs::system::Resource;
use sdl2::pixels::Color;

use crate::SpriteType;

pub struct SpriteRender {
    pub col: usize,
    pub row: usize,
    pub sprite_type: SpriteType,
    pub color: Color,
}

#[derive(Resource)]
pub struct SpriteLayer {
    pub sprites: Vec<SpriteRender>,
}

impl SpriteLayer {
    pub fn new() -> Self {
        Self { sprites: vec![] }
    }
}
