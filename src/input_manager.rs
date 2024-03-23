use bevy_ecs::system::Resource;
use sdl2::{keyboard::Keycode, mouse::MouseState, rect::Point};

use crate::Viewport;


#[derive(Resource)]
pub struct InputManager {
    pub keys : Vec<Keycode>,
    pub mouse_tile : Option<Point>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys : vec![],
            mouse_tile : None
        }
    }

    pub fn update_keys(&mut self, new_keys : Vec<Keycode>) {
        self.keys = new_keys;
    }

    pub fn update_mouse(&mut self, mouse_x : i32, mouse_y : i32, tile_size : u32) {

        if mouse_x > 0 && mouse_y > 0 {
            let x = mouse_x  / tile_size as i32;
            let y = mouse_y / tile_size as i32;

            self.mouse_tile = Some(Point::new(x,y));
        }

    }
}