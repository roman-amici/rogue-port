use bevy_ecs::component::Component;
use sdl2::rect::Point;


#[derive(Component,Copy,Clone)]
pub struct WorldPosition {
    pub x : i32,
    pub y : i32,
}

impl Into<Point> for WorldPosition {
    fn into(self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl From<Point> for WorldPosition {
    fn from(value: Point) -> Self {
        Self {
            x : value.x,
            y : value.y
        }
    }
}