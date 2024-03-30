use std::collections::HashSet;

use bevy_ecs::component::Component;
use sdl2::rect::Point;

#[derive(Clone, Debug, Component)]
pub struct FieldOfView {
    pub visible_tiles : HashSet<Point>,
    pub radius : i32,
    pub dirty : bool
}

impl FieldOfView {
    pub fn new(radius : i32) -> Self {
        Self {
            visible_tiles : HashSet::new(),
            radius,
            dirty : true
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles : HashSet::new(),
            radius : self.radius,
            dirty : true
        }
    }
}