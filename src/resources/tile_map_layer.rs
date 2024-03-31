use bevy_ecs::system::Resource;
use sdl2::pixels::Color;

use crate::SpriteIndex;

#[derive(Resource)]
pub struct TileMapLayer {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<(Color,SpriteIndex)>,
}

impl TileMapLayer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![ (Color::RGB(0,0,0),SpriteIndex{
                sprite_sheet_index : 0,
                tile_index : 0,
            }); width * height],
        }
    }

    pub fn set(&mut self, col: usize, row: usize, tile_index: SpriteIndex, color : Color ) {
        let index = row * self.width + col;

        self.tiles[index] = (color,tile_index);
    }

    pub fn get_index(&self, col: usize, row: usize) -> usize {
        row * self.width + col
    }
}
