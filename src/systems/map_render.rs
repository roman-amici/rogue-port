use bevy_ecs::system::{NonSendMut, Res};
use sdl2::rect::Point;

use crate::{Camera, Map, TileRender, TileType};

pub fn map_render(mut render : NonSendMut<TileRender>, map : Res<Map>, camera: Res<Camera>) {
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            if !map.in_bounds(Point::new(x,y)){
                continue;
            }

            let map_index = map.map_index(x as usize, y as usize );

            let x_screen = (x - camera.left_x) as usize;
            let y_screen = (y - camera.top_y) as usize;
            match map.tiles[map_index] {
                TileType::Floor => {
                    render.draw_tile_grid(x_screen, y_screen, map.tile_map[TileType::Floor as usize])
                }
                TileType::Wall => {
                    render.draw_tile_grid(x_screen, y_screen, map.tile_map[TileType::Wall as usize])
                }
            }
        }
    }
}