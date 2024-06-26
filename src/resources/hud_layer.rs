use bevy_ecs::system::Resource;
use sdl2::rect::Point;

use crate::SpriteIndex;

#[derive(Clone,Debug)]
pub enum HudElement
{
    Tooltip{ tile_space_index : Point, text : String},
    HealthBar { fraction : f32, text : String },
    Inventory { items : Vec<SpriteIndex> }
}

#[derive(Resource)]
pub struct HudLayer 
{
    pub hud_elements : Vec<HudElement>
}

impl HudLayer {
    pub fn new() -> HudLayer {
        Self {
            hud_elements : vec![]
        }
    }
}

