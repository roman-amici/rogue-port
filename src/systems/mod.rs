mod player_input;
mod map_render;
mod sprite_render;

use bevy_ecs::schedule::Schedule;

pub fn build_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_systems(player_input::player_input);
    schedule.add_systems(map_render::map_render);
    schedule.add_systems(sprite_render::sprite_render);

    schedule
}


pub mod prelude{
    pub use super::build_schedule;
}