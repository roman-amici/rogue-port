use bevy_ecs::{bundle::Bundle, system::Commands, world::World};
use rand::{Rng, RngCore};
use sdl2::pixels::Color;

use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: WorldPosition) {
    ecs.spawn((
        Player { level: 0 },
        pos,
        Sprite {
            sprite_type: SpriteType::Knight,
            color: Color::RGB(255, 255, 255),
        },
        Health {
            current: 20,
            max: 20,
        },
        FieldOfView::new(8),
        CrossLevel,
    ));
}

pub fn random_spawn(ecs: &mut World, rng: &mut dyn RngCore, pos: WorldPosition) {
    match rng.gen_range(1..=20) {
        1 => ecs.spawn(spawn_map(pos)),
        2 => ecs.spawn(spawn_potion(pos, 10)),
        _ => ecs.spawn(spawn_monster(rng, pos)),
    };
}

pub fn random_spawn_command(commands: &mut Commands, rng: &mut dyn RngCore, pos: WorldPosition) {
    match rng.gen_range(1..=20) {
        1 => commands.spawn(spawn_map(pos)),
        2 => commands.spawn(spawn_potion(pos, 10)),
        _ => commands.spawn(spawn_monster(rng, pos)),
    };
}

pub fn spawn_monster(rng: &mut dyn RngCore, point: WorldPosition) -> impl Bundle {
    let (sprite_type, name, hp) = match rng.gen_range(1..=10) {
        1..=8 => (SpriteType::Goblin, "Goblin", 1),
        _ => (SpriteType::Orc, "Orc", 2),
    };
    (
        Enemy,
        point,
        Sprite {
            color: Color::RGB(0, 0, 0),
            sprite_type,
        },
        ChasingPlayer {},
        Tooltip {
            text: name.to_string(),
        },
        Health {
            current: hp,
            max: hp,
        },
        FieldOfView::new(6),
    )
}

pub fn spawn_amulet(pos: WorldPosition) -> impl Bundle {
    (
        Item {
            item_type: ItemType::Amulet,
        },
        Tooltip {
            text: "Amulet!".to_string(),
        },
        Sprite {
            color: Color::RGB(0, 0, 0),
            sprite_type: SpriteType::Amulet,
        },
        pos,
    )
}

pub fn spawn_potion(pos: WorldPosition, hp: i32) -> impl Bundle {
    (
        Item {
            item_type: ItemType::Potion,
        },
        Tooltip {
            text: "Potion".to_string(),
        },
        ProvidesHealing { hp },
        pos,
        Sprite {
            color: Color::RGB(255, 255, 255),
            sprite_type: SpriteType::Potion,
        },
    )
}

pub fn spawn_map(pos: WorldPosition) -> impl Bundle {
    (
        Item {
            item_type: ItemType::Map,
        },
        Tooltip {
            text: "Map".to_string(),
        },
        ProvidesDungeonMap,
        pos,
        Sprite {
            color: Color::RGB(255, 255, 255),
            sprite_type: SpriteType::Map,
        },
    )
}
