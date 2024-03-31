use bevy_ecs::component::Component;


#[derive(Component, Copy, Clone)]
pub struct ProvidesHealing{
    pub hp : i32,
}