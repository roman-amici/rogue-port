use bevy_ecs::world::World;
use rand::{Rng, RngCore};
use sdl2::{pixels::Color, rect::Point};

use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: WorldPosition) {
    ecs.spawn((
        crate::components::prelude::Player {},
        pos,
        Sprite {
            sprite_index: 64,
            color: Color::RGB(255, 255, 255),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut dyn RngCore, point: WorldPosition) {
    let sprite_index = match rng.gen_range(0..4) {
        0 => 69,  // Ogre,
        1 => 79,  // Entin
        2 => 103, // Demon?
        _ => 111, // Goblin
    };

    ecs.spawn((
        Enemy,
        point,
        Sprite {
            color: Color::RGB(0, 0, 0),
            sprite_index,
        },
        RandomMover,
    ));
}
