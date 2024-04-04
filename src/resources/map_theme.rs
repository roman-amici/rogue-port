use std::collections::HashMap;

use bevy_ecs::system::Resource;
use rand::{Rng, RngCore};
use sdl2::pixels::Color;

use crate::{SpriteType, TileType};

#[derive(Copy, Clone, Debug)]
pub struct SpriteIndex {
    pub tile_index: usize,
    pub sprite_sheet_index: usize,
}

impl SpriteIndex {
    fn new(sprite_sheet_index: usize, tile_index: usize) -> Self {
        Self {
            tile_index,
            sprite_sheet_index,
        }
    }
}

#[derive(Resource)]
pub struct MapTheme {
    pub tint: Color,
    pub tile_map: HashMap<TileType, SpriteIndex>,
    pub sprite_map: HashMap<SpriteType, SpriteIndex>,
}

impl MapTheme {
    pub fn random_theme(rng: &mut dyn RngCore) -> Self {
        match rng.gen_range(0..6) {
            0..=2 => Self::forrest_theme(),
            _ => Self::dungeon_theme(),
        }
    }

    pub fn new(
        tile_map: HashMap<TileType, SpriteIndex>,
        sprite_map: HashMap<SpriteType, SpriteIndex>,
        tint: Color,
    ) -> Self {
        Self {
            tint,
            tile_map,
            sprite_map,
        }
    }

    fn default_sprite_map() -> HashMap<SpriteType, SpriteIndex> {
        let mut sprite_map = HashMap::new();
        sprite_map.insert(SpriteType::Knight, SpriteIndex::new(0, 64));
        sprite_map.insert(SpriteType::Ogre, SpriteIndex::new(0, 69));
        sprite_map.insert(SpriteType::Orc, SpriteIndex::new(0, 79));
        sprite_map.insert(SpriteType::Daemon, SpriteIndex::new(0, 103));
        sprite_map.insert(SpriteType::Goblin, SpriteIndex::new(0, 111));
        sprite_map.insert(SpriteType::Amulet, SpriteIndex::new(0, 124));
        sprite_map.insert(SpriteType::Potion, SpriteIndex::new(0, 33));
        sprite_map.insert(SpriteType::Map, SpriteIndex::new(0, 123));

        sprite_map
    }

    pub fn dungeon_theme() -> Self {
        let mut tile_map = HashMap::new();
        tile_map.insert(TileType::Wall, SpriteIndex::new(0, 35));
        tile_map.insert(TileType::Floor, SpriteIndex::new(0, 46));
        tile_map.insert(TileType::Stairs, SpriteIndex::new(0, 62));

        let tint = Color::RGB(255, 255, 255);
        Self::new(tile_map, Self::default_sprite_map(), tint)
    }

    pub fn forrest_theme() -> Self {
        let mut tile_map = HashMap::new();
        tile_map.insert(TileType::Wall, SpriteIndex::new(0, 34));
        tile_map.insert(TileType::Floor, SpriteIndex::new(0, 59));
        tile_map.insert(TileType::Stairs, SpriteIndex::new(0, 62));

        let tint = Color::RGB(255, 255, 255);
        Self::new(tile_map, Self::default_sprite_map(), tint)
    }
}
