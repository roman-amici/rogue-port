use bevy_ecs::{
    entity::Entity,
    system::{Query, ResMut},
};
use rand::{self, Rng};
use sdl2::rect::Point;

use crate::resources::{Messenger, WantsToMove};
use crate::{RandomMover, WorldPosition};

pub fn random_move(
    mut query: Query<(Entity, &WorldPosition, &RandomMover)>,
    mut messenger: ResMut<Messenger<WantsToMove>>,
) {
    let rng = &mut rand::thread_rng();
    for (entity, pos, _) in query.iter_mut() {
        let delta = match rng.gen_range(0..4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };
        let point: Point = (*pos).into();
        let destination = point + delta;

        messenger.messages.push(WantsToMove {
            destination,
            entity,
        })
    }
}
