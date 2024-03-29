use bevy_ecs::system::Resource;
use sdl2::rect::Point;

#[derive(Clone, Copy)]
pub struct Viewport {
    pub width_tiles: u32,
    pub height_tiles: u32,
}

#[derive(Resource)]
pub struct Camera {
    pub viewport: Viewport,
    // Mapspace coordinates
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(viewport: Viewport, player_position: Point) -> Self {
        let mut n = Self {
            left_x: 0,
            right_x: 0,
            top_y: 0,
            bottom_y: 0,
            viewport,
        };

        n.player_move(player_position);

        n
    }

    pub fn player_move(&mut self, player_position: Point) {
        let half_width = (self.viewport.width_tiles / 2) as i32;
        let half_height = (self.viewport.height_tiles / 2) as i32;

        self.left_x = player_position.x - half_width;
        self.right_x = player_position.x + half_width;
        self.top_y = player_position.y - half_height;
        self.bottom_y = player_position.y + half_height;
    }

    pub fn worldspace_to_screenspace(&self, pos_world : Point) -> Option<Point> {

        let x_screen = pos_world.x - self.left_x;
        let y_screen = pos_world.y - self.top_y;

        if x_screen > 0 && x_screen <= self.right_x &&
            y_screen > 0 && y_screen <= self.bottom_y {
                Some(Point::new(x_screen, y_screen))
            }
            else {
                None
            }
    }
}
