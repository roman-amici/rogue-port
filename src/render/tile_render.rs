use std::collections::BTreeMap;

use sdl2::{
    rect::Rect,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use crate::resources::*;
use crate::SpriteType;

use super::{sprite_sheet::SpriteSheet, sprite_sheet_info::SpriteSheetInfo};

#[derive(Copy, Clone)]
struct SpriteIndex {
    tile_index: usize,
    sprite_sheet_index: usize,
}

impl SpriteIndex {
    fn new(sprite_sheet_index: usize, tile_index: usize) -> Self {
        Self {
            tile_index,
            sprite_sheet_index,
        }
    }
}

pub struct TileRender<'a> {
    screen_tile_size: u32,
    tile_map: BTreeMap<TileType, SpriteIndex>, //TileMap -> tile_index,
    sprite_map: BTreeMap<SpriteType, SpriteIndex>,
    sprite_sheets: Vec<SpriteSheet<'a>>,
}

impl<'a> TileRender<'a> {
    pub fn new(
        screen_tile_size: u32,
        sprite_info: SpriteSheetInfo,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        let mut tile_map = BTreeMap::new();
        tile_map.insert(TileType::Wall, SpriteIndex::new(0, 35));
        tile_map.insert(TileType::Floor, SpriteIndex::new(0, 46));

        let mut sprite_map = BTreeMap::new();
        sprite_map.insert(SpriteType::Knight, SpriteIndex::new(0, 64));
        sprite_map.insert(SpriteType::Ogre, SpriteIndex::new(0, 69));
        sprite_map.insert(SpriteType::Daemon, SpriteIndex::new(0, 79));
        sprite_map.insert(SpriteType::Ogre, SpriteIndex::new(0, 111));

        let sprite_sheet = SpriteSheet::new(&sprite_info, texture_creator);

        Self {
            screen_tile_size,
            sprite_map,
            tile_map,
            sprite_sheets: vec![sprite_sheet],
        }
    }

    pub fn draw_tile(
        &self,
        canvas: &mut Canvas<Window>,
        col: usize,
        row: usize,
        tile_type: TileType,
    ) {
        let tile_loc = self
            .tile_map
            .get(&tile_type)
            .as_deref()
            .map(|opt| *opt)
            .unwrap_or(SpriteIndex::new(0, 0));

        self.draw_tile_index(canvas, col, row, tile_loc)
    }

    pub fn draw_sprite(
        &self,
        canvas: &mut Canvas<Window>,
        col: usize,
        row: usize,
        sprite_type: SpriteType,
    ) {
        let tile_loc = self
            .sprite_map
            .get(&sprite_type)
            .as_deref()
            .map(|opt| *opt)
            .unwrap_or(SpriteIndex::new(0, 0));

        self.draw_tile_index(canvas, col, row, tile_loc);
    }

    fn draw_tile_index(
        &self,
        canvas: &mut Canvas<Window>,
        col: usize,
        row: usize,
        location: SpriteIndex,
    ) {
        let x = col as u32 * self.screen_tile_size;
        let y = row as u32 * self.screen_tile_size;

        self.sprite_sheets[location.sprite_sheet_index].render_index(
            canvas,
            location.tile_index,
            Rect::new(
                x as i32,
                y as i32,
                self.screen_tile_size,
                self.screen_tile_size,
            ),
        );
    }
}
