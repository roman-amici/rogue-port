use bevy_ecs::{
    query::With,
    system::{Query, Res, ResMut},
};
use sdl2::{pixels::Color, rect::Point};

use crate::{resources::*, FieldOfView, Player};

pub fn map_render(
    player_fov: Query<&FieldOfView, With<Player>>,
    mut map_layer: ResMut<TileMapLayer>,
    map: Res<Map>,
    camera: Res<Camera>,
    theme: Res<MapTheme>,
) {
    for fov in player_fov.iter() {
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                let x_screen = (x - camera.left_x) as usize;
                let y_screen = (y - camera.top_y) as usize;

                if !map.in_bounds(Point::new(x, y)) {
                    // Render out of bounds as black
                    map_layer.set(
                        x_screen,
                        y_screen,
                        theme.tile_map[&TileType::Wall],
                        Color::RGBA(255, 255, 255, 0),
                    );
                    continue;
                }

                let point = Point::new(x, y);
                let color = if fov.visible_tiles.contains(&point) {
                    Color::RGB(255, 255, 255)
                } else if map.revealed_tiles[map.map_index(point.x as usize, point.y as usize)] {
                    Color::RGBA(255, 255, 255, 128)
                } else {
                    Color::RGBA(255, 255, 255, 0)
                };

                let map_index = map.map_index(x as usize, y as usize);

                let tile_type = map.tiles[map_index];
                let sprite_index = theme.tile_map[&tile_type];

                map_layer.set(x_screen, y_screen, sprite_index, color);
            }
        }
    }
}
