use bevy_ecs::{
    query::With,
    system::{Commands, Query, ResMut},
};

use crate::{Health, Map, Messenger, Player, ProvidesDungeonMap, ProvidesHealing, UseItem};

pub fn use_item(
    mut player: Query<&mut Health, With<Player>>,
    mut map: ResMut<Map>,
    item_properties: Query<(Option<&ProvidesHealing>, Option<&ProvidesDungeonMap>)>,
    mut use_item_messages: ResMut<Messenger<UseItem>>,
    mut commands: Commands,
) {
    for message in use_item_messages.messages.iter() {
        if let Ok((healing, reveal_map)) = item_properties.get(message.entity) {
            if let Some(healing) = healing {
                for mut player_health in player.iter_mut() {
                    player_health.current =
                        i32::min(player_health.current + healing.hp, player_health.max);
                }
            }

            if reveal_map.is_some() {
                map.revealed_tiles
                    .iter_mut()
                    .for_each(|revealed| *revealed = true);
            }
        }

        commands.entity(message.entity).despawn();
    }

    use_item_messages.messages.clear();
}
