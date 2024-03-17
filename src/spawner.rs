use bevy_ecs::world::World;
use rand::{Rng, RngCore};
use sdl2::{pixels::Color, rect::Point};

use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: WorldPosition) {
    ecs.spawn((
        crate::components::prelude::Player {},
        pos,
        Sprite {
            sprite_type: SpriteType::Knight,
            color: Color::RGB(255, 255, 255),
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut dyn RngCore, point: WorldPosition) {
    let sprite_type = match rng.gen_range(0..4) {
        0 => SpriteType::Ogre,   // Ogre,
        1 => SpriteType::Entin,  // Entin
        2 => SpriteType::Daemon, // Demon?
        _ => SpriteType::Goblin, // Goblin
    };

    ecs.spawn((
        Enemy,
        point,
        Sprite {
            color: Color::RGB(0, 0, 0),
            sprite_type,
        },
        RandomMover,
    ));
}
