use bevy_ecs::{query::With, system::{Query, Res, ResMut}};
use sdl2::mouse;

use crate::{Camera, FieldOfView, Health, HudLayer, InputManager, Player, Tooltip, WorldPosition};

pub fn tooltip(
    player_fov : Query<&FieldOfView, With<Player>>,
    query: Query<(&Tooltip, &WorldPosition, Option<&Health>)>,
    camera: Res<Camera>,
    mut hud_layer: ResMut<HudLayer>,
    input_manager: Res<InputManager>,
) {
    if input_manager.mouse_tile.is_none() {
        return;
    }

    let mouse_pos = input_manager.mouse_tile.unwrap();

    for fov in player_fov.iter() {
        for (tooltip, pos, maybe_health) in query.iter() {

            if !fov.visible_tiles.contains(&((*pos).into())) {
                continue;
            }

            if let Some(screen_pos) = camera.worldspace_to_screenspace((*pos).into()) {
                if screen_pos == mouse_pos {
                    let text = if let Some(health) = maybe_health {
                        format!("{} : {} hp", tooltip.text, health.current)
                    } else {
                        tooltip.text.clone()
                    };
    
                    hud_layer.hud_elements.push(crate::HudElement::Tooltip {
                        tile_space_index: screen_pos,
                        text,
                    });
                }
            }
        }
    }

}
