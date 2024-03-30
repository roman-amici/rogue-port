use bevy_ecs::system::{Query, Res};

use crate::{utilities::field_of_view_set::find_fov_set, FieldOfView, Map, WorldPosition};

pub fn fov(mut query : Query<(&WorldPosition, &mut FieldOfView)>, map : Res<Map> ){
    for (pos, mut view) in query.iter_mut() {
        if view.dirty {
            view.visible_tiles = find_fov_set((*pos).into(), view.radius, &map);
            view.dirty = false;
        }
    }
}