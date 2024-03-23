use bevy_ecs::world::World;
use rand::{Rng, RngCore};
use sdl2::pixels::Color;

use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: WorldPosition) {
    ecs.spawn((
        crate::components::prelude::Player {},
        pos,
        Sprite {
            sprite_type: SpriteType::Knight,
            color: Color::RGB(255, 255, 255),
        },
        Health{ current : 12, max : 20}
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut dyn RngCore, point: WorldPosition) {
    let sprite_type = match rng.gen_range(0..4) {
        0 => SpriteType::Ogre,   // Ogre,
        1 => SpriteType::Entin,  // Entin
        2 => SpriteType::Daemon, // Demon?
        _ => SpriteType::Goblin, // Goblin
    };

    let name = match sprite_type {
        SpriteType::Knight => "Knight",
        SpriteType::Ogre => "Ogre",
        SpriteType::Entin => "Entin",
        SpriteType::Goblin => "Goblin",
        SpriteType::Daemon => "Daemon",
    };

    ecs.spawn((
        Enemy,
        point,
        Sprite {
            color: Color::RGB(0, 0, 0),
            sprite_type,
        },
        RandomMover,
        Tooltip{ text : name.to_string()}
    ));
}
