use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Health {
    pub current : i32,
    pub max : i32,
}