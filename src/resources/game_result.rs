use bevy_ecs::system::Resource;


#[derive(Resource)]
pub enum GameResult {
    Win,
    Loss,
    New   
}