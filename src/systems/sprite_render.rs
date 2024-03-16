use bevy_ecs::system::{NonSendMut, Query, Res};
use sdl2::rect::Point;

use crate::{tile_render, Camera, Sprite, TileRender, WorldPosition};

pub fn sprite_render(query : Query<(&Sprite, &WorldPosition)>, mut render : NonSendMut<TileRender>, camera : Res<Camera>){
    let top_left = Point::new(camera.left_x, camera.top_y);
    
    
    for (sprite, pos) in query.iter() {
        let point : Point = (*pos).into();
        let point_screen = point - top_left;
        render.draw_tile_grid(
            point_screen.x as usize,
            point_screen.y as usize,
            sprite.sprite_index,
        );
    }
}