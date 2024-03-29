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
        Health {
            current: 12,
            max: 20,
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut dyn RngCore, point: WorldPosition) {
    let (sprite_type, name, hp) = match rng.gen_range(1..=10) {
        1..=8 => (SpriteType::Goblin, "Goblin", 1),
        _ => (SpriteType::Orc, "Orc", 2),
    };
    ecs.spawn((
        Enemy,
        point,
        Sprite {
            color: Color::RGB(0, 0, 0),
            sprite_type,
        },
        ChasingPlayer{},
        Tooltip {
            text: name.to_string(),
        },
        Health {
            current: hp,
            max: hp,
        },
    ));
}

pub fn spawn_amulet(ecs : &mut World, pos : WorldPosition) {
    ecs.spawn((
        Item{
            item_type : ItemType::Amulet
        },
        Tooltip {
            text : "Amulet!".to_string(),
        },
        Sprite {
            color : Color::RGB(0, 0, 0),
            sprite_type : SpriteType::Amulet,
        },
        pos
    ));
}
