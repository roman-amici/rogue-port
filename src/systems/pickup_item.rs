use bevy_ecs::{entity::Entity, query::With, system::{Commands, Query, ResMut}};

use crate::{AddOrder, Carrying, Item, Player, WorldPosition};

pub fn pickup_item(
    player : Query<&WorldPosition, With<Player>>,
    item : Query<(Entity, &WorldPosition), With<Item>>,
    mut add_order : ResMut<AddOrder>,
    mut commands : Commands
) {
    for player_pos in player.iter() {
        for (item_entity, item_pos) in item.iter() {
            if *player_pos == *item_pos {
                commands.entity(item_entity)
                    .insert(Carrying {
                        add_order : add_order.next()
                    })
                    .remove::<WorldPosition>();
            }
        }
    }
}