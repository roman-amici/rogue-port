use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query},
};

use crate::CrossLevel;

pub fn remove_level_entities(query: Query<(Entity, Option<&CrossLevel>)>, mut commands: Commands) {
    query
        .iter()
        .filter_map(
            |(entity, cross)| {
                if cross.is_some() {
                    None
                } else {
                    Some(entity)
                }
            },
        )
        .for_each(|entity| commands.entity(entity).despawn());
}
