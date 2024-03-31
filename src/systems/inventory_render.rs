use bevy_ecs::{query::With, system::{Query, Res, ResMut}};

use crate::{Carrying, HudElement, HudLayer, Item, MapTheme, Sprite, SpriteIndex};

pub fn inventory_render(
    inventory_query : Query<(&Sprite, &Carrying), With<Item>>, 
    mut hud_layer : ResMut<HudLayer>,
    map_theme : Res<MapTheme>,
){
    let mut items : Vec<(Sprite,Carrying)> = inventory_query.iter().map(|(s,c)| (*s,*c)).collect();

    items.sort_by(|(_,c1), (_, c2)| c1.add_order.cmp(&c2.add_order) );
    let items : Vec<SpriteIndex> = items.into_iter()
        .map(|(sprite,_)| map_theme.sprite_map[&sprite.sprite_type])
        .collect();

    hud_layer.hud_elements.push(HudElement::Inventory { items });
}