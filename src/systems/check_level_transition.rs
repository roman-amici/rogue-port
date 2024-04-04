use bevy_ecs::system::{Query, Res, ResMut};

use crate::{Map, Player, TileType, TurnState, WorldPosition};

pub fn check_level_transition(
    mut player: Query<(&WorldPosition, &mut Player)>,
    mut turn_state: ResMut<TurnState>,
    map: Res<Map>,
) {
    for (pos, mut player) in player.iter_mut() {
        let index = map.map_index(pos.x as usize, pos.y as usize);
        if map.tiles[index] == TileType::Stairs {
            *turn_state = TurnState::LevelTransition;
            player.level += 1;
        }
    }
}
