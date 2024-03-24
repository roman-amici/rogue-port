use bevy_ecs::{query::With, system::{Query, ResMut}};

use crate::{Health, Player, TurnState};

pub fn end_turn(
    player_health_query : Query<&Health, With<Player>>,
    mut turn_state: ResMut<TurnState>) {

        for health in player_health_query.iter() {

            if health.current <= 0 {
                *turn_state = TurnState::GameEnd;
            }
        }

    let new_state = match *turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => unreachable!()
    };

    *turn_state = new_state;
}
