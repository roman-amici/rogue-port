mod collisions;
mod end_turn;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod sprite_render;
mod health_bar_render;
mod tooltip_render;

use bevy_ecs::schedule::{apply_deferred, IntoSystemConfigs, Schedule};

use self::collisions::collisions;
use self::end_turn::end_turn;
use self::map_render::map_render;
use self::movement::movement;
use self::player_input::player_input;
use self::random_move::random_move;
use self::sprite_render::sprite_render;
use self::health_bar_render::player_health_bar;
use self::tooltip_render::tooltip;

pub fn build_input_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(player_input);

    schedule.add_systems(map_render.before(sprite_render).after(player_input));
    schedule.add_systems(sprite_render);
    schedule.add_systems(player_health_bar);
    schedule.add_systems(tooltip);

    schedule
}

pub fn build_player_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(movement);
    schedule.add_systems(collisions.after(movement));

    schedule.add_systems(apply_deferred.after(collisions));

    schedule.add_systems(map_render.before(sprite_render).after(collisions));
    schedule.add_systems(sprite_render);
    schedule.add_systems(player_health_bar.after(collisions));

    schedule.add_systems(end_turn.after(sprite_render));

    // Post process

    schedule
}

pub fn build_enemy_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(random_move);
    schedule.add_systems(movement.after(random_move));
    schedule.add_systems(collisions.after(movement));

    schedule.add_systems(map_render.before(sprite_render).after(collisions));
    schedule.add_systems(sprite_render);
    schedule.add_systems(player_health_bar.after(collisions));

    schedule.add_systems(end_turn.after(sprite_render));

    schedule
}

