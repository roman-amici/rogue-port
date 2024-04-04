use bevy_ecs::system::ResMut;

use crate::TurnState;

pub fn start_level(mut turn_state: ResMut<TurnState>) {
    *turn_state = TurnState::AwaitingInput;
}
