use bevy_ecs::component::Component;
use sdl2::pixels::Color;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum SpriteType {
    Knight,
    Ogre,
    Orc,
    Goblin,
    Daemon,
    Amulet
}

#[derive(Component)]
pub struct Sprite {
    pub sprite_type: SpriteType,
    pub color: Color,
}
