use sdl2::{keyboard::Keycode, rect::Point};

use crate::prelude::*;

pub struct Player {
    pub position: Point,
    pub tile_index: usize,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self {
            position,
            tile_index: 64,
        }
    }

    pub fn render(&self, renderer: &mut TileRender, camera : &Camera) {
        let x_screen = (self.position.x - camera.left_x) as usize;
        let y_screen = (self.position.y - camera.top_y) as usize;
        
        renderer.draw_tile_grid(
            x_screen,
            y_screen,
            self.tile_index,
        );
    }

    pub fn update_position(&mut self, keys: &Vec<Keycode>, map: &Map, camera : &mut Camera) -> bool {
        let delta = keys
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

        let new_pos = self.position + delta;
        if map.player_can_enter(new_pos) {
            self.position = new_pos;
            camera.player_move(new_pos);
        }


        Point::new(0, 0) != delta
    }
}
