use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct MainMenuLayer 
{
    pub title : String,
    pub options : Vec<String>
}

impl MainMenuLayer {
    pub fn new() -> Self {
        Self{
            title : "".to_string(),
            options : vec![]
        }
    }
}

