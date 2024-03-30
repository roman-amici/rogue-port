use bevy_ecs::{query::With, system::{NonSendMut, Query, Res, ResMut}};
use sdl2::{pixels::Color, rect::Point};

use crate::{resources::*, FieldOfView, Player};

pub fn map_render(player_fov : Query<&FieldOfView, With<Player>>, mut map_layer: ResMut<TileMapLayer>, map: Res<Map>, camera: Res<Camera>) {

    for fov in player_fov.iter() {
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if !map.in_bounds(Point::new(x, y))  {
                    continue;
                }

                let point = Point::new(x,y);
                let color = if fov.visible_tiles.contains(&point) {
                    Color::RGB(255, 255,255)
                } else if map.revealed_tiles[ map.map_index(point.x as usize, point.y as usize)] {
                    Color::RGBA(255, 255, 255, 128)
                } else {
                    Color::RGBA(255,255,255,0)
                };
    
                let map_index = map.map_index(x as usize, y as usize);
    
                let x_screen = (x - camera.left_x) as usize;
                let y_screen = (y - camera.top_y) as usize;
    
                map_layer.set(x_screen, y_screen, map.tiles[map_index], color);
            }
        }
    }

}
