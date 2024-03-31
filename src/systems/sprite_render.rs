use bevy_ecs::query::With;
use bevy_ecs::system::{Query, Res, ResMut};
use sdl2::rect::Point;

use crate::{resources::*, FieldOfView, Player};
use crate::{Sprite, WorldPosition};

pub fn sprite_render(
    player_fov : Query<&FieldOfView, With<Player>>,
    query: Query<(&Sprite, &WorldPosition)>,
    mut sprite_layer: ResMut<SpriteLayer>,
    camera: Res<Camera>,
    theme : Res<MapTheme>,
) {
    let top_left = Point::new(camera.left_x, camera.top_y);

    for fov in player_fov.iter() {
        for (sprite, pos) in query.iter() {
            let point: Point = (*pos).into();
            let point_screen = point - top_left;
    
            if point_screen.x >= 0
                && point_screen.y >= 0
                && point_screen.x < camera.viewport.width_tiles as i32
                && point_screen.y < camera.viewport.height_tiles as i32
                && fov.visible_tiles.contains(&point)
            {
                let sprite_index = theme.sprite_map[&sprite.sprite_type];

                sprite_layer.sprites.push(SpriteRender {
                    col: point_screen.x as usize,
                    row: point_screen.y as usize,
                    color: sprite.color,
                    sprite_index,
                })
            }
        }
    }
}
