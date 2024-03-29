use bevy_ecs::system::{Res, ResMut};
use sdl2::keyboard::Keycode;

use crate::{InputManager, Messenger, SystemMessage, TurnState};

pub fn menu_input(input : Res<InputManager>, mut turn_state : ResMut<TurnState>, mut system_messages : ResMut<Messenger<SystemMessage>> ) {
    if input.keys.iter().any(|x| *x == Keycode::Return) {
        *turn_state = TurnState::PlayerTurn
    } else if input.keys.iter().any(|x| *x == Keycode::Q) {
        system_messages.messages.push(SystemMessage::ShouldQuit);
    }
}