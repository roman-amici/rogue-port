use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut},
};
use rand::{self, Rng};
use sdl2::rect::Point;

use crate::{
    resources::{Messenger, WantsToMove},
    Player, WantsToAttack,
};
use crate::{RandomMover, WorldPosition};

pub fn random_move(
    mut query: Query<(Entity, &WorldPosition, &RandomMover)>,
    player_query: Query<(Entity, &WorldPosition), With<Player>>,
    mut move_messages: ResMut<Messenger<WantsToMove>>,
    mut attack_messages: ResMut<Messenger<WantsToAttack>>,
) {
    let rng = &mut rand::thread_rng();
    for (mover_entity, pos, _) in query.iter_mut() {
        let delta = match rng.gen_range(0..4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };
        let point: Point = (*pos).into();
        let destination = point + delta;

        let mut hit = false;
        for (player_entity, player_pos) in player_query.iter() {
            let player_point: Point = (*player_pos).into();
            if player_point == destination {
                hit = true;
                attack_messages.messages.push(WantsToAttack {
                    entity: mover_entity,
                    victim: player_entity,
                });
            }
        }

        if !hit {
            move_messages.messages.push(WantsToMove {
                destination,
                entity: mover_entity,
            })
        }
    }
}
