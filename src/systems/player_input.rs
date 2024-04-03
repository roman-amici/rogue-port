use std::cmp::min;

use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};
use sdl2::{keyboard::Keycode, rect::Point, sys::KeyCode};

use crate::{
    components::prelude::Player, Carrying, Health, InputManager, Item, TurnState, WorldPosition,
};
use crate::{resources::*, Enemy};

pub fn player_input(
    query: Query<(Entity, &WorldPosition), With<Player>>,
    enemy_query: Query<(Entity, &Enemy, &WorldPosition)>,
    item_query: Query<(Entity, &Carrying), With<Item>>,
    mut health_query: Query<&mut Health, With<Player>>,
    input_manager: ResMut<InputManager>,
    mut turn_state: ResMut<TurnState>,
    mut use_item: ResMut<Messenger<UseItem>>,
    mut move_events: ResMut<Messenger<WantsToMove>>,
    mut attack_events: ResMut<Messenger<WantsToAttack>>,
) {
    let is_move = input_manager.keys.iter().any(|k| match *k {
        Keycode::Left => true,
        Keycode::Right => true,
        Keycode::Up => true,
        Keycode::Down => true,
        Keycode::Space => true,
        _ => false,
    });

    let item_select_index = input_manager
        .keys
        .iter()
        .filter_map(|k| match *k {
            Keycode::Num0 | Keycode::Kp0 => Some(0),
            Keycode::Num1 | Keycode::Kp1 => Some(1),
            Keycode::Num2 | Keycode::Kp2 => Some(2),
            Keycode::Num3 | Keycode::Kp3 => Some(3),
            Keycode::Num4 | Keycode::Kp4 => Some(4),
            Keycode::Num5 | Keycode::Kp5 => Some(5),
            Keycode::Num6 | Keycode::Kp6 => Some(6),
            Keycode::Num7 | Keycode::Kp7 => Some(7),
            Keycode::Num8 | Keycode::Kp8 => Some(8),
            Keycode::Num9 | Keycode::Kp9 => Some(9),
            _ => None,
        })
        .nth(0);

    if let Some(index) = item_select_index {
        let mut carrying_item: Vec<(Entity, i32)> = item_query
            .iter()
            .map(|(entity, carrying)| (entity, carrying.add_order))
            .collect();
        carrying_item.sort_by(|(_, i1), (_, i2)| i1.cmp(i2));

        if let Some((entity, _)) = carrying_item.get(index) {
            use_item.messages.push(UseItem { entity: *entity });
        }

        *turn_state = TurnState::PlayerTurn;
    }

    if !is_move {
        return;
    }

    *turn_state = TurnState::PlayerTurn;

    let delta = input_manager
        .keys
        .iter()
        .filter_map(|k| match *k {
            Keycode::Left => Some(Point::new(-1, 0)),
            Keycode::Right => Some(Point::new(1, 0)),
            Keycode::Up => Some(Point::new(0, -1)),
            Keycode::Down => Some(Point::new(0, 1)),
            _ => None,
        })
        .nth(0)
        .unwrap_or(Point::new(0, 0));

    for (player_entity, pos) in query.iter() {
        let position: Point = (*pos).into();
        let destination = position + delta;

        let mut hit = false;
        for (enemy_entity, _, enemy_pos) in enemy_query.iter() {
            let enemy_point: Point = (*enemy_pos).into();
            if destination == enemy_point {
                hit = true;
                attack_events.messages.push(WantsToAttack {
                    entity: player_entity,
                    victim: enemy_entity,
                });
            }
        }

        if !hit {
            move_events.messages.push(WantsToMove {
                destination,
                entity: player_entity,
            });
        }
    }
}
