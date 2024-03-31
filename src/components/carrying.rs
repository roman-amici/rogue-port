use bevy_ecs::component::Component;

#[derive(Component, Clone, Copy)]
pub struct Carrying {
    pub add_order : i32
}