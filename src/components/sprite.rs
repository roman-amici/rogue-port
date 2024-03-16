
use bevy_ecs::component::Component;
use sdl2::pixels::Color;

#[derive(Component)]
pub struct Sprite {
    pub sprite_index : usize,
    pub color : Color
}