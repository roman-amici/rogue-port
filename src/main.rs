use std::time::Duration;

use bevy_ecs::{schedule::Schedule, system::Resource, world::World};
use prelude::*;
use render::{
    new_canvas, render_map_layer, render_sprite_layer, sprite_sheet_info::SpriteSheetInfo,
    HudRender, MainMenuRender, TextRender,
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
    menu_schedule: Schedule,
}

#[derive(Resource, Copy, Clone, Debug, PartialEq)]
enum TurnState {
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
    GameEnd,
}

fn build_world(viewport: Viewport) -> State {
    let mut ecs = World::new();

    ecs.insert_resource(TurnState::AwaitingInput);

    let rng = &mut rand::thread_rng();
    let mut map_arch = random_architect(rng);
    let map_builder = map_arch.new(
        (viewport.width_tiles * 4) as usize,
        (viewport.height_tiles * 4) as usize,
        rng,
    );

    let camera = Camera::new(viewport, map_builder.player_start);

    map_builder
        .monster_spawn
        .iter()
        .for_each(|pos| spawn_monster(&mut ecs, rng, (*pos).into()));

    let map = map_builder.map;

    spawn_amulet(&mut ecs, map_builder.amulet_start.into());

    ecs.insert_resource(TileMapLayer::new(
        camera.viewport.width_tiles as usize,
        camera.viewport.height_tiles as usize,
    ));

    ecs.insert_resource(MapTheme::random_theme(rng));
    ecs.insert_resource(SpriteLayer::new());
    ecs.insert_resource(camera);
    ecs.insert_resource(InputManager::new());
    ecs.insert_resource(PlayerDistanceMap::new(&map));
    ecs.insert_resource(map);
    ecs.insert_resource(Messenger::<WantsToMove>::new());
    ecs.insert_resource(Messenger::<WantsToAttack>::new());
    ecs.insert_resource(Messenger::<SystemMessage>::new());
    ecs.insert_resource(MainMenuLayer::new());
    ecs.insert_resource(HudLayer::new());
    ecs.insert_resource(GameResult::New);

    spawn_player(&mut ecs, map_builder.player_start.into());

    State {
        ecs,
        awaiting_input: build_input_schedule(),
        player_turn: build_player_schedule(),
        enemy_turn: build_enemy_schedule(),
        menu_schedule: build_menu_schedule(),
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
    let mut turn_state = state.ecs.resource_mut::<TurnState>();
    *turn_state = TurnState::GameEnd;

    let screen_tile_size = 32;
    let mut canvas = new_canvas(&video_subsystem, viewport, screen_tile_size)?;
    let texture_creator = canvas.texture_creator();

    let info = SpriteSheetInfo {
        path: "dungeonfont.png".to_string(),
        tile_size_pixels: 32,
    };

    let mut tile_render = render::TileRender::new(screen_tile_size, info, &texture_creator);

    let mut font = ttf_context.load_font("FreeMono.ttf", screen_tile_size as u16)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let menu_render = MainMenuRender::new(
        (viewport.width_tiles * screen_tile_size) as i32,
        (viewport.height_tiles * screen_tile_size) as i32,
    );
    let mut text_render = TextRender::new();
    let hud_render = HudRender::new(screen_tile_size, viewport);
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

            let turn = *state.ecs.resource::<TurnState>();

            match turn {
                TurnState::AwaitingInput => state.awaiting_input.run(&mut state.ecs),
                TurnState::PlayerTurn => state.player_turn.run(&mut state.ecs),
                TurnState::EnemyTurn => state.enemy_turn.run(&mut state.ecs),
                TurnState::GameEnd => state.menu_schedule.run(&mut state.ecs),
            }

            let mut system_messages = state.ecs.resource_mut::<Messenger<SystemMessage>>();
            let messages: Vec<SystemMessage> = system_messages.messages.drain(..).collect();
            for message in messages {
                match message {
                    SystemMessage::ShouldQuit => {
                        break 'running;
                    }
                    SystemMessage::NewGame => {
                        state = build_world(viewport);
                        continue 'running;
                    }
                }
            }

            canvas.set_draw_color((0, 0, 0));
            canvas.clear();

            if turn == TurnState::GameEnd {
                let menu_layer = state.ecs.resource::<MainMenuLayer>();

                text_render.add_to_cache(
                    &menu_layer.title,
                    Color::RGB(255, 255, 255),
                    &texture_creator,
                    &font,
                );
                for option in menu_layer.options.iter() {
                    text_render.add_to_cache(
                        option,
                        Color::RGB(255, 255, 255),
                        &texture_creator,
                        &font,
                    );
                }
                menu_render.render_menu(&mut canvas, menu_layer, &text_render);
            } else {
                let map_layer = state.ecs.resource::<TileMapLayer>();
                render_map_layer(&map_layer, &mut canvas, &mut tile_render);

                let sprite_layer = state.ecs.resource::<SpriteLayer>();
                render_sprite_layer(&sprite_layer, &mut canvas, &mut tile_render);

                let hud_layer = state.ecs.resource::<HudLayer>();
                for element in hud_layer.hud_elements.iter() {
                    match element {
                        HudElement::HealthBar { text, .. } => text_render.add_to_cache(
                            text,
                            Color::RGB(255, 255, 255),
                            &texture_creator,
                            &font,
                        ),
                        HudElement::Tooltip { text, .. } => text_render.add_to_cache(
                            text,
                            Color::RGB(0, 0, 0),
                            &texture_creator,
                            &font,
                        ),
                    }
                }
                hud_render.render_hud_layer(&hud_layer, &mut canvas, &mut text_render);
            }

            canvas.present();

            let mut sprite_layer = state.ecs.resource_mut::<SpriteLayer>();
            sprite_layer.sprites.clear();

            let mut hud_layer = state.ecs.resource_mut::<HudLayer>();
            hud_layer.hud_elements.clear();

            let mut menu_layer = state.ecs.resource_mut::<MainMenuLayer>();
            menu_layer.options.clear();

            last_frame = now;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
