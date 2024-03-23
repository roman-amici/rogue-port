use std::cmp::min;

use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};
use sdl2::{keyboard::Keycode, rect::Point};

use crate::{components::prelude::Player, Health, InputManager, TurnState, WorldPosition};
use crate::{resources::*, Enemy};

pub fn player_input(
    query: Query<(Entity, &WorldPosition), With<Player>>,
    enemy_query: Query<(Entity, &Enemy, &WorldPosition)>,
    mut health_query: Query<&mut Health, With<Player>>,
    input_manager: ResMut<InputManager>,
    mut turn_state: ResMut<TurnState>,

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

    if !is_move {
        return;
    }

    *turn_state = TurnState::PlayerTurn;

    if input_manager.keys.iter().any(|k| *k == Keycode::Space) {
        for mut player_health in health_query.iter_mut() {
            player_health.current = min(player_health.current + 1, player_health.max);
        }
        return;
    }

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
