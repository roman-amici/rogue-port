use bevy_ecs::system::Resource;


#[derive(Resource, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    Win,
    Loss,
    New   
}