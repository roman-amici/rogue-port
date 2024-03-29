use std::{collections::BinaryHeap, f32::INFINITY};

use sdl2::rect::Point;

use crate::Map;

struct QueueEntry {
    distance: f32,
    index: usize,
}

impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Eq for QueueEntry {}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.distance.partial_cmp(&other.distance) {
            Some(ord) => ord,
            None => std::cmp::Ordering::Less, // Numbers less than inf, Nan
        }
    }
}

pub struct DijkstraMap {
    priority_queue: BinaryHeap<QueueEntry>,
    rows: usize,
    cols: usize,
    distances: Vec<f32>,
}

impl DijkstraMap {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            priority_queue: BinaryHeap::new(),
            rows,
            cols,
            distances: vec![INFINITY; cols * rows],
        }
    }

    fn reset(&mut self) {
        self.priority_queue.clear();
        self.distances.fill(INFINITY);
    }

    fn exits(point: Point, map: &Map) -> [Point; 4] {
        [
            Point::new(point.x - 1, point.y),
            Point::new(point.x + 1, point.y),
            Point::new(point.x, point.y + 1),
            Point::new(point.x, point.y - 1),
        ]
    }

    fn cartesian_distance(p1: Point, p2: Point) -> f32 {
        let diff = p1 - p2;
        f32::sqrt((diff.x * diff.x + diff.y * diff.y) as f32)
    }

    pub fn fill_all(&mut self, (start_col, start_row): (usize, usize), map: &Map) {
        self.reset();

        let start_idx = map.map_index(start_col, start_row);

        self.distances[start_idx] = 0.0;
        self.priority_queue.push(QueueEntry {
            distance: 0.0,
            index: start_idx,
        });

        while self.priority_queue.len() > 0 {
            let entry = self.priority_queue.pop().unwrap();

            let distance = entry.distance + 1.0;
            let (col, row) = map.map_index_to_coords(entry.index);
            for exit in Self::exits(Point::new(col as i32, row as i32), map) {
                if map.can_enter(exit) {
                    let index = map.map_index(exit.x as usize, exit.y as usize);
                    if self.distances[index] == INFINITY {
                        self.distances[index] = distance;
                        self.priority_queue.push(QueueEntry { distance, index });
                    }
                }
            }
        }
    }

    pub fn next_hop(&self, (start_col, start_row): (usize, usize), map: &Map) -> Option<Point> {
        let index = map.map_index(start_col, start_row);
        if self.distances[index] == INFINITY {
            return None;
        }

        let mut min_entry = QueueEntry {
            distance: self.distances[index],
            index,
        };
        for exit in Self::exits(Point::new(start_col as i32, start_row as i32), map) {
            let index = map.map_index(exit.x as usize, exit.y as usize);
            let distance = self.distances[index];

            if map.can_enter(exit) {
                if  distance <= min_entry.distance {
                    min_entry = QueueEntry { distance, index }
                }
            }
        }

        if min_entry.index == index {
            return None;
        }

        let (col, row) = map.map_index_to_coords(min_entry.index);
        Some(Point::new(col as i32, row as i32))
    }

    // In order for this to map the distances in fill_all, you need to have the destination
    // be the start tile and the source be the end tile
    pub fn fill_astar(
        &mut self,
        (start_col, start_row): (usize, usize),
        (end_col, end_row): (usize, usize),
        map: &Map,
        max_search_distance: f32,
    ) -> bool {
        self.reset();

        let start_idx = map.map_index(start_col, start_row);
        let end_idx = map.map_index(end_col, end_row);
        let end_point = Point::new(end_col as i32, end_row as i32);
        self.distances[start_idx] = 0.0;

        self.priority_queue.push(QueueEntry {
            distance: 0.0,
            index: start_idx,
        });

        let mut grid_distance = 0.0;
        while self.priority_queue.len() > 0 && grid_distance < max_search_distance {
            let entry = self.priority_queue.pop().unwrap();
            grid_distance = self.distances[entry.index] + 1.0;

            if entry.index == end_idx {
                return true;
            }

            let (col, row) = map.map_index_to_coords(entry.index);
            let entry_point = Point::new(col as i32, row as i32);

            for exit in Self::exits(entry_point, map) {
                if map.can_enter(exit) {
                    let index = map.map_index(exit.x as usize, exit.y as usize);
                    if self.distances[index] == INFINITY {
                        self.distances[index] = grid_distance;

                        let distance = grid_distance + Self::cartesian_distance(exit, end_point);
    
                        self.priority_queue.push(QueueEntry { distance, index });
                    }
                }
            }
        }

        false
    }

    pub fn max_distance_tile(&self) -> Point {
        let mut max_distance = 0.0;
        let mut max_index = 0;

        for (index, distance) in self.distances.iter().enumerate() {
            if *distance < INFINITY && *distance > max_distance {
                max_distance = *distance;
                max_index = index;
            }
        }

        let x = max_index % self.cols;
        let y = max_index / self.cols;

        Point::new(x as i32, y as i32)
    }
}
