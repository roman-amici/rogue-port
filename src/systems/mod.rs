mod collisions;
mod combat;
mod end_turn;
mod health_bar_render;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod sprite_render;
mod tooltip_render;

use bevy_ecs::schedule::{apply_deferred, IntoSystemConfigs, Schedule};

use self::collisions::collisions;
use self::combat::combat;
use self::end_turn::end_turn;
use self::health_bar_render::player_health_bar;
use self::map_render::map_render;
use self::movement::movement;
use self::player_input::player_input;
use self::random_move::random_move;
use self::sprite_render::sprite_render;
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
    schedule.add_systems(combat);

    schedule.add_systems(apply_deferred.after(combat).after(movement));

    schedule.add_systems(map_render.before(sprite_render).after(apply_deferred));
    schedule.add_systems(sprite_render.after(apply_deferred));
    schedule.add_systems(player_health_bar.after(combat));

    schedule.add_systems(end_turn.after(sprite_render));

    // Post process

    schedule
}

pub fn build_enemy_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(random_move);
    schedule.add_systems(movement.after(random_move));
    schedule.add_systems(combat.after(random_move));

    schedule.add_systems(
        map_render
            .before(sprite_render)
            .after(combat)
            .after(movement),
    );
    schedule.add_systems(sprite_render);
    schedule.add_systems(player_health_bar.after(combat));

    schedule.add_systems(end_turn.after(sprite_render));

    schedule
}
