mod combat;
mod end_turn;
mod health_bar_render;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod sprite_render;
mod tooltip_render;
mod chasing_player;
mod menu_input;
mod menu_render;
mod game_over;
mod fov;
mod pickup_item;
mod inventory_render;

use bevy_ecs::schedule::{apply_deferred, IntoSystemConfigs, Schedule};

use self::combat::combat;
use self::end_turn::end_turn;
use self::health_bar_render::player_health_bar;
use self::map_render::map_render;
use self::menu_render::main_menu_render;
use self::movement::movement;
use self::player_input::player_input;
use self::sprite_render::sprite_render;
use self::tooltip_render::tooltip;
use self::chasing_player::chase;
use self::menu_input::menu_input;
use self::game_over::check_game_over;
use self::fov::fov;
use self::pickup_item::pickup_item;
use self::inventory_render::inventory_render;

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
    schedule.add_systems(pickup_item.after(movement));
    schedule.add_systems(fov.after(combat).before(map_render));

    schedule.add_systems(apply_deferred.after(combat).after(movement));
    schedule.add_systems(check_game_over.after(combat).after(movement));

    schedule.add_systems(map_render.before(sprite_render).after(apply_deferred));
    schedule.add_systems(sprite_render.after(apply_deferred));
    schedule.add_systems(player_health_bar.after(combat));
    schedule.add_systems(inventory_render.after(apply_deferred));

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
