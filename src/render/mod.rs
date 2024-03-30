use sdl2::{render::Canvas, video::Window, VideoSubsystem};

use crate::resources::*;

pub use self::tile_render::TileRender;
pub use self::text_render::TextRender;
pub use self::hud_render::HudRender;
pub use self::main_menu_render::MainMenuRender;

mod sprite_sheet;
pub mod sprite_sheet_info;
mod tile_render;
mod text_render;
mod hud_render;
mod main_menu_render;

pub fn new_canvas(
    video_subsystem: &VideoSubsystem,
    viewport: Viewport,
    screen_tile_size: u32,
) -> Result<Canvas<Window>, String> {
    let window_width = viewport.width_tiles * screen_tile_size;
    let window_height = viewport.height_tiles * screen_tile_size;
    let window = video_subsystem
        .window("dungeon", window_width, window_height)
        .position_centered()
        .build()
        .or_else(|x| Err(x.to_string()))?;

    let canvas = window
        .into_canvas()
        .build()
        .or_else(|e| Err(e.to_string()))?;

    Ok(canvas)
}

pub fn render_map_layer(
    screen_tiles: &TileMapLayer,
    canvas: &mut Canvas<Window>,
    tile_render: &mut TileRender,
) {
    for row in 0..screen_tiles.height {
        for col in 0..screen_tiles.width {
            let index = screen_tiles.get_index(col, row);
            let (color, tile_type) = screen_tiles.tiles[index];
            tile_render.draw_tile(canvas, col, row, color, tile_type)
        }
    }
}

pub fn render_sprite_layer(
    sprite_layer: &SpriteLayer,
    canvas: &mut Canvas<Window>,
    tile_render: &mut TileRender,
) {
    for s in sprite_layer.sprites.iter() {
        tile_render.draw_sprite(canvas, s.col, s.row, s.sprite_type);
    }
}