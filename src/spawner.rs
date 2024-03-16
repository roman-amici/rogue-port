use bevy_ecs::world::World;
use sdl2::{pixels::Color, rect::Point};

use crate::prelude::*;

pub fn spawn_player(ecs : &mut World, pos : WorldPosition) {
    ecs.spawn((crate::components::prelude::Player{},
    pos,
    Sprite {
        sprite_index : 64,
        color : Color::RGB(255,255, 255)
    }));
}