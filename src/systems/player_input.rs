use bevy_ecs::system::{Query, Res, ResMut};
use sdl2::{keyboard::Keycode, rect::Point};

use crate::{components::prelude::Player, Camera, InputManager, Map, WorldPosition};

pub fn player_input(
    mut query: Query<(&Player, &mut WorldPosition)>,
    input_manager: ResMut<InputManager>,
    map: Res<Map>,
    mut camera: ResMut<Camera>,
) {
    let delta = input_manager
        .keys
        .iter()
        .filter_map(|k| match *k {
            Keycode::Left => Some(Point::new(-1, 0)),
            Keycode::Right => Some(Point::new(1, 0)),
            Keycode::Up => Some(Point::new(0, -1)),
            Keycode::Down => Some(Point::new(0, 1)),
            _ => None,
        })
        .nth(0)
        .unwrap_or(Point::new(0, 0));

    if delta == Point::new(0, 0) {
        return;
    }

    for (_, mut pos) in query.iter_mut() {
        let position: Point = (*pos).into();
        let destination = position + delta;

        if map.can_enter(destination) {
            *pos = destination.into();
            camera.player_move(destination);
        }
    }
}
