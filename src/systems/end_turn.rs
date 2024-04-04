use bevy_ecs::system::{Res, ResMut};

use crate::{GameResult, TurnState};

pub fn end_turn(mut turn_state: ResMut<TurnState>, game_result: Res<GameResult>) {
    if *game_result != GameResult::New {
        *turn_state = TurnState::GameEnd;
        return;
    }

    if *turn_state == TurnState::LevelTransition {
        return;
    }

    let new_state = match *turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => unreachable!(),
    };

    *turn_state = new_state;
}
