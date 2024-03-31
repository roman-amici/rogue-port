

use sdl2::{
    pixels::Color, rect::{Point, Rect}, render::{Canvas, TextureCreator}, video::{Window, WindowContext}
};

use crate::resources::*;


use super::{sprite_sheet::SpriteSheet, sprite_sheet_info::SpriteSheetInfo};

pub struct TileRender<'a> {
    screen_tile_size: u32,
    sprite_sheets: Vec<SpriteSheet<'a>>,
}

impl<'a> TileRender<'a> {
    pub fn new(
        screen_tile_size: u32,
        sprite_info: SpriteSheetInfo,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {

        let sprite_sheet = SpriteSheet::new(&sprite_info, texture_creator);

        Self {
            screen_tile_size,
            sprite_sheets: vec![sprite_sheet]
        }
    }

    pub fn draw_tile(
        &mut self,
        canvas: &mut Canvas<Window>,
        col: usize,
        row: usize,
        color : Color,
        tile_index: SpriteIndex,
    ) {
        self.draw_tile_index(canvas, col, row, tile_index, color)
    }

    pub fn draw_sprite(
        &mut self,
        canvas: &mut Canvas<Window>,
        col: usize,
        row: usize,
        sprite_index: SpriteIndex,
    ) {
        self.draw_tile_index(canvas, col, row, sprite_index, Color::RGB(255, 255, 255));
    }

    pub fn tile_to_screen_space(screen_tile_size: u32, row: i32, col: i32) -> Point {
        let x = col * screen_tile_size as i32;
        let y = row * screen_tile_size as i32;

        Point::new(x, y)
    }

    fn draw_tile_index(
        &mut self,
        canvas: &mut Canvas<Window>,
        col: usize,
        row: usize,
        location: SpriteIndex,
        color : Color
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
            color
        );
    }
}
