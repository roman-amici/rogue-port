use std::time::Duration;

use prelude::*;
use sdl2::{event::Event, image::InitFlag, keyboard::Keycode, rect::Point};

mod map;
mod map_builder;
mod player;
mod tile_render;
mod camera;

mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::camera::*;
    pub use crate::tile_render::*;
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video()?;

    let info = SpriteSheetInfo {
        path: "dungeonfont.png".to_string(),
        tile_size_pixels: 32,
    };

    let rows = 20;
    let cols = 40;

    let viewport = Viewport {
        height_tiles : rows,
        width_tiles : cols
    };

    let mut renderer = TileRender::new(info, &video_subsystem, 16)?;

    let rng = &mut rand::thread_rng();
    let map_builder = MapBuilder::new((cols*4) as usize, (rows*4) as usize, 20, vec![35, 46], rng);

    let mut camera = Camera::new(viewport, map_builder.player_start);
    let map = map_builder.map;
    let mut player = Player::new(map_builder.player_start);

    let mut event_pump = sdl_context.event_pump()?;
    let timers = sdl_context.timer()?;

    let mut last_player_update = timers.ticks();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let keys: Vec<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let now = timers.ticks();
        if (now - last_player_update) > 80 && player.update_position(&keys, &map, &mut camera) {
            last_player_update = now;
        }

        renderer.start_batch();
        map.render(&mut renderer, &camera);
        player.render(&mut renderer, &camera);
        renderer.end_batch();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
