use bevy_ecs::{entity::Entity, system::Resource};
use sdl2::rect::Point;

#[derive(Resource)]
pub struct Messenger<T> {
    pub messages: Vec<T>,
}

impl<T> Messenger<T> {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }
}

pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

pub struct WantsToAttack {
    pub entity: Entity,
    pub victim: Entity,
}

pub enum SystemMessage {
    ShouldQuit,
    NewGame,
}