use crate::{resources::*, FieldOfView, Player, WorldPosition};
use bevy_ecs::system::{Query, Res, ResMut};

pub fn movement(
    mut query: Query<(&mut WorldPosition, &mut FieldOfView, Option<&Player>)>,
    mut move_events: ResMut<Messenger<WantsToMove>>,
    mut map: ResMut<Map>,
    mut camera: ResMut<Camera>,
) {
    for event in move_events.messages.iter() {
        if !map.can_enter(event.destination) {
            continue;
        }

        if let Ok((mut world_position, mut fov, maybe_player)) = query.get_mut(event.entity) {
            *world_position = event.destination.into();
            fov.dirty = true;

            if maybe_player.is_some() {
                for point in fov.visible_tiles.iter() {
                    let index = map.map_index(point.x as usize, point.y as usize);
                    map.revealed_tiles[index] = true;
                }

                camera.player_move(event.destination);

            }
        }
    }

    move_events.messages.clear();
}
