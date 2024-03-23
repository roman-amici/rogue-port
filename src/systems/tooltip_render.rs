use bevy_ecs::system::{Query, Res, ResMut};
use sdl2::mouse;

use crate::{render::HudRender, Camera, HudLayer, InputManager, Tooltip, WorldPosition};

pub fn tooltip(query : Query<(&Tooltip, &WorldPosition)>, camera : Res<Camera>, mut hud_layer : ResMut<HudLayer>, input_manager : Res<InputManager> ) {

    if input_manager.mouse_tile.is_none(){
        return;
    }

    let mouse_pos = input_manager.mouse_tile.unwrap();

    for (tooltip, pos) in query.iter() {
        if let Some(screen_pos) = camera.worldspace_to_screenspace((*pos).into()) {
            if screen_pos == mouse_pos {
                hud_layer.hud_elements.push(crate::HudElement::Tooltip { tile_space_index: screen_pos, text: tooltip.text.clone() });
            }
        }
    }
}