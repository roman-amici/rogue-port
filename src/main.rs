use std::time::Duration;

use bevy_ecs::{schedule::Schedule, system::Resource, world::World};
use prelude::*;
use render::{
    new_canvas, render_map_layer, render_sprite_layer, sprite_sheet_info::SpriteSheetInfo,
    HudRender, TextRender,
};
use sdl2::{event::Event, image::InitFlag, keyboard::Keycode, pixels::Color, rect::Rect};

mod components;

mod input_manager;
mod map_builder;
mod render;
mod resources;
mod spawner;
mod systems;
mod utilities;

mod prelude {
    pub use crate::components::prelude::*;
    pub use crate::input_manager::*;
    pub use crate::map_builder::*;
    pub use crate::resources::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

struct State {
    ecs: World,
    awaiting_input: Schedule,
    player_turn: Schedule,
    enemy_turn: Schedule,
}

#[derive(Resource, Copy, Clone, Debug, PartialEq)]
enum TurnState {
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
    GameEnd,
}

fn build_world(viewport : Viewport) -> State {

    let mut ecs = World::new();

    ecs.insert_resource(TurnState::AwaitingInput);

    let rng = &mut rand::thread_rng();
    let map_builder = MapBuilder::new(
        (viewport.width_tiles * 4) as usize,
        (viewport.height_tiles * 4) as usize,
        20,
        vec![35, 46],
        rng,
    );

    let camera = Camera::new(viewport, map_builder.player_start);
    
    map_builder
    .rooms
    .iter()
    .skip(1)
    .map(|r| r.center())
    .for_each(|pos| spawn_monster(&mut ecs, rng, pos.into()));
    
    let map = map_builder.map;

    ecs.insert_resource(TileMapLayer::new(
        camera.viewport.width_tiles as usize,
        camera.viewport.height_tiles as usize,
    ));

    ecs.insert_resource(SpriteLayer::new());
    ecs.insert_resource(camera);
    ecs.insert_resource(InputManager::new());
    ecs.insert_resource(PlayerDistanceMap::new(&map));
    ecs.insert_resource(map);
    ecs.insert_resource(Messenger::<WantsToMove>::new());
    ecs.insert_resource(Messenger::<WantsToAttack>::new());
    ecs.insert_resource(HudLayer::new());

    spawn_player(&mut ecs, map_builder.player_start.into());


    State {
        ecs,
        awaiting_input: build_input_schedule(),
        player_turn: build_player_schedule(),
        enemy_turn: build_enemy_schedule(),
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let rows = 20;
    let cols = 40;

    let viewport = Viewport {
        height_tiles: rows,
        width_tiles: cols,
    };

    let mut state = build_world(viewport);

    let screen_tile_size = 32;
    let mut canvas = new_canvas(&video_subsystem, viewport, screen_tile_size)?;
    let texture_creator = canvas.texture_creator();

    let info = SpriteSheetInfo {
        path: "dungeonfont.png".to_string(),
        tile_size_pixels: 32,
    };

    let tile_render = render::TileRender::new(screen_tile_size, info, &texture_creator);

    let mut font = ttf_context.load_font("FreeMono.ttf", screen_tile_size as u16)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut text_render = TextRender::new();
    let mut hud_render = HudRender::new(screen_tile_size, viewport);
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
        if (now - last_frame) > 30 {
            let keys: Vec<Keycode> = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            let mouse_state = event_pump.mouse_state();

            let mut input_manager = state.ecs.resource_mut::<InputManager>();
            input_manager.update_keys(keys);
            input_manager.update_mouse(mouse_state.x(), mouse_state.y(), screen_tile_size);

            let turn = state.ecs.resource::<TurnState>();

            match *turn {
                TurnState::AwaitingInput => state.awaiting_input.run(&mut state.ecs),
                TurnState::PlayerTurn => state.player_turn.run(&mut state.ecs),
                TurnState::EnemyTurn => state.enemy_turn.run(&mut state.ecs),
                TurnState::GameEnd => {}
            }

            canvas.set_draw_color((0, 0, 0));
            canvas.clear();

            let map_layer = state.ecs.resource::<TileMapLayer>();
            render_map_layer(&map_layer, &mut canvas, &tile_render);

            let mut sprite_layer = state.ecs.resource_mut::<SpriteLayer>();
            render_sprite_layer(&sprite_layer, &mut canvas, &tile_render);

            sprite_layer.sprites.clear();

            let mut hud_layer = state.ecs.resource_mut::<HudLayer>();
            for element in hud_layer.hud_elements.iter() {
                match element {
                    HudElement::HealthBar { text, .. } => text_render.add_to_cache(
                        text,
                        Color::RGB(255, 255, 255),
                        &texture_creator,
                        &font,
                    ),
                    HudElement::Tooltip { text, .. } => {
                        text_render.add_to_cache(text, Color::RGB(0, 0, 0), &texture_creator, &font)
                    }
                }
            }

            hud_render.render_hud_layer(&hud_layer, &mut canvas, &mut text_render);
            hud_layer.hud_elements.clear();

            canvas.present();

            last_frame = now;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
