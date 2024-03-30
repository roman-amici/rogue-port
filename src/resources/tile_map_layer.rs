use bevy_ecs::system::Resource;
use sdl2::{pixels::Color, render::Canvas, video::Window};

use crate::resources::map::TileType;

#[derive(Resource)]
pub struct TileMapLayer {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<(Color,TileType)>,
}

impl TileMapLayer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![ (Color::RGB(0,0,0),TileType::Wall); width * height],
        }
    }

    pub fn set(&mut self, col: usize, row: usize, tile: TileType, color : Color ) {
        let index = row * self.width + col;

        self.tiles[index] = (color,tile);
    }

    pub fn get_index(&self, col: usize, row: usize) -> usize {
        row * self.width + col
    }
}
