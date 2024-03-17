use bevy_ecs::component::Component;
use sdl2::pixels::Color;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpriteType {
    Knight,
    Ogre,
    Entin,
    Goblin,
    Daemon,
}

#[derive(Component)]
pub struct Sprite {
    pub sprite_type: SpriteType,
    pub color: Color,
}
