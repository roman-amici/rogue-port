use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query},
};

use crate::{Enemy, Player, WorldPosition};

pub fn collisions(
    players: Query<(&Player, &WorldPosition)>,
    enemies: Query<(Entity, &Enemy, &WorldPosition)>,
    mut commands: Commands,
) {
    for (_, player_position) in players.iter() {
        for (enemy_entity, _, enemy_position) in enemies.iter() {
            if *player_position == *enemy_position {
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
