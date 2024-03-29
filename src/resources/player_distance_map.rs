use bevy_ecs::system::Resource;
use sdl2::rect::Point;

use crate::{utilities::dijkstra_map::DijkstraMap, Map, WorldPosition};

#[derive(Resource)]
pub struct PlayerDistanceMap {
    dijkstra_map: DijkstraMap,
}

impl PlayerDistanceMap {
    pub fn new(map: &Map) -> Self {
        Self {
            dijkstra_map: DijkstraMap::new(map.width_tiles, map.height_tiles),
        }
    }

    pub fn fill(&mut self, player_position: WorldPosition, map: &Map) {
        self.dijkstra_map.fill_all(
            (player_position.x as usize, player_position.y as usize),
            map,
        );
    }

    pub fn next_hop(&self, search_position: WorldPosition, map: &Map) -> Option<Point> {
        self.dijkstra_map.next_hop(
            (search_position.x as usize, search_position.y as usize),
            map,
        )
    }

    pub fn max_distance_tile(&self) -> WorldPosition {
        self.dijkstra_map.max_distance_tile().into()
    }
}
