use bevy_ecs::component::Component;

#[derive(Copy,Clone,Debug, PartialEq, Eq)]
pub enum ItemType {
    Amulet,
    Potion,
    Map,
}

#[derive(Copy,Clone,Component)]
pub struct Item {
    pub item_type : ItemType
}