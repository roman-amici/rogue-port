use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    system::{Query, ResMut},
};
use sdl2::{keyboard::Keycode, rect::Point};

use crate::{
    components::prelude::Player, InputManager, Messenger, TurnState, WantsToMove, WorldPosition,
};

pub fn player_input(
    mut query: Query<(Entity, &Player, &WorldPosition)>,
    input_manager: ResMut<InputManager>,
    mut turn_state: ResMut<TurnState>,

    mut move_events: ResMut<Messenger<WantsToMove>>,
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

    for (entity, _, pos) in query.iter_mut() {
        let position: Point = (*pos).into();
        let destination = position + delta;

        move_events.messages.push(WantsToMove {
            destination,
            entity,
        });
    }
}
