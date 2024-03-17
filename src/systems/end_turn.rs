use bevy_ecs::system::ResMut;

use crate::TurnState;

pub fn end_turn(mut turn_state: ResMut<TurnState>) {
    let new_state = match *turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
    };

    *turn_state = new_state;
}
