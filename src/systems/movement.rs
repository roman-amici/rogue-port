use crate::{resources::*, Player, WorldPosition};
use bevy_ecs::system::{Query, Res, ResMut};

pub fn movement(
    mut query: Query<(&mut WorldPosition, Option<&Player>)>,
    mut move_events: ResMut<Messenger<WantsToMove>>,
    map: Res<Map>,
    mut camera: ResMut<Camera>,
) {
    for event in move_events.messages.iter() {
        if !map.can_enter(event.destination) {
            continue;
        }

        if let Ok((mut world_position, maybe_player)) = query.get_mut(event.entity) {
            *world_position = event.destination.into();

            if maybe_player.is_some() {
                camera.player_move(event.destination);
            }
        }
    }

    move_events.messages.clear();
}
