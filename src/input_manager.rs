use bevy_ecs::system::Resource;
use sdl2::keyboard::Keycode;


#[derive(Resource)]
pub struct InputManager {
    pub keys : Vec<Keycode>
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys : vec![]
        }
    }

    pub fn update_keys(&mut self, new_keys : Vec<Keycode>) {
        self.keys = new_keys;
    }
}