use std::time::Duration;

use bevy_ecs::{schedule::Schedule, world::World};
use prelude::*;
use sdl2::{event::Event, image::InitFlag, keyboard::Keycode};

mod components;

mod camera;
mod input_manager;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod tile_render;

mod prelude {
    pub use crate::camera::*;
    pub use crate::components::prelude::*;
    pub use crate::input_manager::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::tile_render::*;
}

struct State {
    ecs: World,
    systems: Schedule,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video()?;

    let info = SpriteSheetInfo {
        path: "dungeonfont.png".to_string(),
        tile_size_pixels: 32,
    };

    let mut ecs = World::new();

    let rows = 20;
    let cols = 40;

    let viewport = Viewport {
        height_tiles: rows,
        width_tiles: cols,
    };

    let renderer = TileRender::new(info, &video_subsystem, 64)?;

    ecs.insert_non_send_resource(renderer);

    let rng = &mut rand::thread_rng();
    let map_builder = MapBuilder::new(
        (cols * 4) as usize,
        (rows * 4) as usize,
        20,
        vec![35, 46],
        rng,
    );

    let camera = Camera::new(viewport, map_builder.player_start);
    let map = map_builder.map;

    ecs.insert_resource(camera);
    ecs.insert_resource(InputManager::new());
    ecs.insert_resource(map);

    let mut state = State {
        ecs,
        systems: build_schedule(),
    };

    spawn_player(&mut state.ecs, map_builder.player_start.into());
    map_builder
        .rooms
        .iter()
        .skip(1)
        .map(|r| r.center())
        .for_each(|pos| spawn_monster(&mut state.ecs, rng, pos.into()));

    let mut event_pump = sdl_context.event_pump()?;
    let timers = sdl_context.timer()?;

    let mut last_frame = timers.ticks();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let now = timers.ticks();
        if (now - last_frame) > 80 {
            let keys: Vec<Keycode> = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            let mut input_manager = state.ecs.resource_mut::<InputManager>();
            input_manager.update_keys(keys);

            let mut renderer = state.ecs.non_send_resource_mut::<TileRender>();
            renderer.start_batch();

            state.systems.run(&mut state.ecs);

            let mut renderer = state.ecs.non_send_resource_mut::<TileRender>();
            renderer.end_batch();

            last_frame = now;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
