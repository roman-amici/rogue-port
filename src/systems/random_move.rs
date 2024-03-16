use bevy_ecs::system::{Query, Res};
use rand::{self, Rng};
use sdl2::rect::Point;

use crate::{Map, RandomMover, WorldPosition};

pub fn random_move(mut query: Query<(&mut WorldPosition, &RandomMover)>, map: Res<Map>) {
    let rng = &mut rand::thread_rng();
    for (mut pos, _) in query.iter_mut() {
        let delta = match rng.gen_range(0..4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        };
        let point: Point = (*pos).into();
        let destination = point + delta;

        if map.can_enter(destination) {
            *pos = destination.into();
        }
    }
}
