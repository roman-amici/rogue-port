use bevy_ecs::system::{Commands, Query, Res, ResMut};

use crate::{
    random_architect, random_spawn_command, spawn_amulet, Camera, FieldOfView, Map, MapTheme,
    Player, TileType, WorldPosition,
};

pub fn new_level_map(
    mut player: Query<(&Player, &mut WorldPosition, &mut FieldOfView)>,
    mut map: ResMut<Map>,
    mut map_theme: ResMut<MapTheme>,
    camera: Res<Camera>,
    mut commands: Commands,
) {
    let rng = &mut rand::thread_rng();

    let mut map_arch = random_architect(rng);
    let map_builder = map_arch.new(
        (camera.viewport.width_tiles * 4) as usize,
        (camera.viewport.height_tiles * 4) as usize,
        rng,
    );

    map_builder
        .spawn_points
        .iter()
        .for_each(|pos| random_spawn_command(&mut commands, rng, (*pos).into()));

    *map = map_builder.map;

    let level = if let Some((player, mut pos, mut fov)) = player.iter_mut().nth(0) {
        *pos = map_builder.player_start.into();
        fov.dirty = true;
        player.level
    } else {
        0
    };

    if level == 3 {
        commands.spawn(spawn_amulet(map_builder.amulet_start.into()));
    } else {
        let index = map.map_index(
            map_builder.amulet_start.x as usize,
            map_builder.amulet_start.y as usize,
        );
        map.tiles[index] = TileType::Stairs;
    }

    *map_theme = MapTheme::random_theme(rng);
}
