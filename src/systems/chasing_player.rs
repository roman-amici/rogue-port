use bevy_ecs::{entity::Entity, query::With, system::{Query, Res, ResMut}};
use sdl2::rect::Point;

use crate::{Enemy, FieldOfView, Map, Messenger, Player, PlayerDistanceMap, WantsToAttack, WantsToMove, WorldPosition};

pub fn chase( player_query : Query<(Entity, &WorldPosition), With<Player>>,  
    enemies_query : Query<(Entity, &WorldPosition, &FieldOfView), With<Enemy>>,
    mut move_messages : ResMut<Messenger<WantsToMove>>,
    mut combat_messages : ResMut<Messenger<WantsToAttack>>,
    mut player_map : ResMut<PlayerDistanceMap>,
    map : Res<Map>,
) {
    for (player_entity, pos) in player_query.iter() {
        player_map.fill(*pos, &map);
        let player_point : Point = (*pos).into();

        for (enemy_entity, enemy_pos, fov) in enemies_query.iter(){

            if !fov.visible_tiles.contains(&player_point) {
                continue;
            }

            if let Some(destination) = player_map.next_hop(*enemy_pos, &map) {        
                if player_point == destination {
                    combat_messages.messages.push(WantsToAttack {
                        entity: enemy_entity,
                        victim: player_entity,
                    });
                } else {
                    move_messages.messages.push(WantsToMove {
                        destination,
                        entity: enemy_entity,
                    })
                }
            }
        }
    }
}