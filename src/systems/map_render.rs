use bevy_ecs::system::{NonSendMut, Res, ResMut};
use sdl2::rect::Point;

use crate::resources::*;

pub fn map_render(mut map_layer: ResMut<TileMapLayer>, map: Res<Map>, camera: Res<Camera>) {
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            if !map.in_bounds(Point::new(x, y)) {
                continue;
            }

            let map_index = map.map_index(x as usize, y as usize);

            let x_screen = (x - camera.left_x) as usize;
            let y_screen = (y - camera.top_y) as usize;

            map_layer.set(x_screen, y_screen, map.tiles[map_index]);
        }
    }
}
