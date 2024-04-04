mod chasing_player;
mod check_level_transition;
mod combat;
mod end_turn;
mod fov;
mod game_over;
mod health_bar_render;
mod inventory_render;
mod map_render;
mod menu_input;
mod menu_render;
mod movement;
mod new_level_map;
mod pickup_item;
mod player_input;
mod random_move;
mod remove_level_entities;
mod sprite_render;
mod start_level;
mod tooltip_render;
mod use_item;

use bevy_ecs::schedule::{apply_deferred, IntoSystemConfigs, Schedule};

use self::chasing_player::chase;
use self::check_level_transition::check_level_transition;
use self::combat::combat;
use self::end_turn::end_turn;
use self::fov::fov;
use self::game_over::check_game_over;
use self::health_bar_render::player_health_bar;
use self::inventory_render::inventory_render;
use self::map_render::map_render;
use self::menu_input::menu_input;
use self::menu_render::main_menu_render;
use self::movement::movement;
use self::new_level_map::new_level_map;
use self::pickup_item::pickup_item;
use self::player_input::player_input;
use self::remove_level_entities::remove_level_entities;
use self::sprite_render::sprite_render;
use self::start_level::start_level;
use self::tooltip_render::tooltip;
use self::use_item::use_item;

pub fn build_input_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(player_input);
    schedule.add_systems(fov.after(player_input));

    schedule.add_systems(map_render.before(sprite_render).after(fov));
    schedule.add_systems(sprite_render);
    schedule.add_systems(player_health_bar);
    schedule.add_systems(tooltip);
    schedule.add_systems(inventory_render);

    schedule
}

pub fn build_player_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(movement);
    schedule.add_systems(combat);
    schedule.add_systems(use_item.after(combat));
    schedule.add_systems(pickup_item.after(movement));
    schedule.add_systems(fov.after(combat).before(map_render));

    schedule.add_systems(apply_deferred.after(combat).after(movement));
    schedule.add_systems(check_game_over.after(combat).after(movement));

    schedule.add_systems(map_render.before(sprite_render).after(apply_deferred));
    schedule.add_systems(sprite_render.after(apply_deferred));
    schedule.add_systems(player_health_bar.after(combat));
    schedule.add_systems(inventory_render.after(apply_deferred));
    schedule.add_systems(check_level_transition.after(movement).before(end_turn));

    schedule.add_systems(end_turn.after(sprite_render));

    // Post process

    schedule
}

pub fn build_enemy_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(chase);
    schedule.add_systems(movement.after(chase));
    schedule.add_systems(combat.after(chase));
    schedule.add_systems(check_game_over.after(combat).after(movement));
    schedule.add_systems(fov.after(combat).before(map_render));

    schedule.add_systems(
        map_render
            .before(sprite_render)
            .after(combat)
            .after(movement),
    );
    schedule.add_systems(sprite_render);
    schedule.add_systems(player_health_bar.after(combat));
    schedule.add_systems(inventory_render.after(apply_deferred));

    schedule.add_systems(end_turn.after(sprite_render));

    schedule
}

pub fn build_menu_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_systems(main_menu_render);
    schedule.add_systems(menu_input);

    schedule
}

pub fn build_level_transition_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_systems(remove_level_entities);
    schedule.add_systems(apply_deferred.after(remove_level_entities));
    schedule.add_systems(new_level_map.after(apply_deferred));
    schedule.add_systems(start_level);

    schedule
}
