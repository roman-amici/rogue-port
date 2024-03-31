use sdl2::rect::Point;

pub fn cartesian_distance(p1: Point, p2: Point) -> f32 {
    let diff = p1 - p2;
    f32::sqrt((diff.x * diff.x + diff.y * diff.y) as f32)
}
