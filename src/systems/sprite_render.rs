use bevy_ecs::system::{Query, Res, ResMut};
use sdl2::rect::Point;

use crate::resources::*;
use crate::{Sprite, WorldPosition};

pub fn sprite_render(
    query: Query<(&Sprite, &WorldPosition)>,
    mut sprite_layer: ResMut<SpriteLayer>,
    camera: Res<Camera>,
) {
    let top_left = Point::new(camera.left_x, camera.top_y);

    for (sprite, pos) in query.iter() {
        let point: Point = (*pos).into();
        let point_screen = point - top_left;

        if point_screen.x >= 0
            && point_screen.y >= 0
            && point_screen.x < camera.viewport.width_tiles as i32
            && point_screen.y < camera.viewport.height_tiles as i32
        {
            sprite_layer.sprites.push(SpriteRender {
                col: point_screen.x as usize,
                row: point_screen.y as usize,
                color: sprite.color,
                sprite_type: sprite.sprite_type,
            })
        }
    }
}
