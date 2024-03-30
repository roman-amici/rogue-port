use std::collections::HashSet;

use sdl2::rect::Point;

use crate::Map;

fn distance(p1 : Point, p2 : Point) -> f32 {
    let p = p1 - p2;

    f32::sqrt((p.x * p.x + p.y * p.y) as f32)
}

fn min_max_distance(center : Point, point : Point) -> (Point,Point) 
{
    let mut min_point = point;
    let mut min_distance = distance(center, point);

    let mut max_point = point;
    let mut max_distance = min_distance;


    let px1y = Point::new(point.x + 1, point.y);
    let pxy1 = Point::new(point.x, point.y+1);
    let px1y1 = Point::new(point.x+1, point.y+1);

    for p in [px1y, pxy1, px1y1] {
        let d = distance(center, p);
        if d < min_distance {
            min_distance = d;
            min_point = p;
        }
        if d > max_distance {
            max_distance = d;
            max_point = p;
        }
    }

    (min_point, max_point)

}

fn circle_points(position : Point, radius : i32) -> HashSet<Point> {

    let max_x = position.x + radius;
    let min_x = position.x - radius;
    let max_y = position.y + radius;
    let min_y = position.y - radius;

    let r = radius as f32;

    let mut points = HashSet::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {

            let point = Point::new(x,y);
            let (min_point,max_point) = min_max_distance(position, point);

            let min_distance = distance(min_point, position);
            let max_distance = distance(max_point, position);

            if min_distance <= r && max_distance >= r {
                points.insert(Point::new(x,y));
            }
        }
    }

    points

}

pub fn direction_vec( (x1, y1) : (f32,f32), (x2,y2) : (f32,f32)) -> (f32, f32) {
    
    let (dx,dy) = ((x2 - x1), (y2 - y1));

    let sqrt = f32::sqrt(dx * dx + dy * dy);
    let norm_x = dx  / sqrt;
    let norm_y = dy  / sqrt;

    (norm_x, norm_y)
}

pub fn find_fov_set(position : Point, radius : i32, map : &Map) -> HashSet<Point> {

    let mut set = HashSet::new();

    let max_x = position.x + radius;
    let min_x = position.x - radius;
    let max_y = position.y + radius;
    let min_y = position.y - radius;
    let circle_points = circle_points(position, radius);
    let pos_center =  (position.x as f32 + 0.5, position.y as f32 + 0.5);

    for cp in circle_points {
        let (dir_x, dir_y) = direction_vec(pos_center, (cp.x as f32 + 0.5, cp.y as f32 + 0.5));
        let (mut pos_x, mut pos_y) = pos_center;

        while (pos_x as i32) != cp.x || (pos_y as i32) != cp.y {
            let point_x = pos_x as i32;
            let point_y = pos_y as i32;

            if !map.in_bounds(Point::new(point_x, point_y)){
                break;
            }
            let index = map.map_index(point_x as usize, point_y as usize);
            if map.is_opaque(index) {
                set.insert(Point::new(point_x, point_y));
                break;
            }

            set.insert(Point::new(point_x, point_y));
            pos_x += dir_x*0.1;
            pos_y += dir_y*0.1;
        }
    }

    set
}