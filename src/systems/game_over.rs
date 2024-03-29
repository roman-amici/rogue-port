use bevy_ecs::{query::With, system::{Query, ResMut}};

use crate::{GameResult, Health, Item, ItemType, Player, WorldPosition};

pub fn check_game_over(
    player : Query<(&Health, &WorldPosition), With<Player>>,
    amulet : Query<(&WorldPosition,&Item)>,
    mut game_result : ResMut<GameResult>)
{

    if player.iter().len() == 0 {
        *game_result = GameResult::Loss;
        return;
    }

    for (health, player_pos) in player.iter() {
        if health.current <= 0 {
            *game_result = GameResult::Loss;
            return;
        }

        for (item_pos, item) in amulet.iter() {
            if item.item_type == ItemType::Amulet && item_pos == player_pos {
                *game_result = GameResult::Win;
                return;
            }
        }
    }
}