use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Player {
    pub level: usize,
}
