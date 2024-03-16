mod collisions;
mod map_render;
mod player_input;
mod random_move;
mod sprite_render;

use bevy_ecs::schedule::{apply_deferred, IntoSystemConfigs, Schedule};

pub fn build_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_systems(player_input::player_input.before(collisions::collisions));
    schedule.add_systems(collisions::collisions);

    schedule.add_systems(apply_deferred.after(collisions::collisions));

    schedule.add_systems(
        map_render::map_render
            .before(sprite_render::sprite_render)
            .after(collisions::collisions),
    );
    schedule.add_systems(sprite_render::sprite_render);

    // Post process
    schedule.add_systems(random_move::random_move.after(sprite_render::sprite_render));

    schedule
}

pub mod prelude {
    pub use super::build_schedule;
}
