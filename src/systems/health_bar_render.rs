use bevy_ecs::{query::With, system::{Query, ResMut}};

use crate::{Health, HudElement, HudLayer, Player};

pub fn player_health_bar(
    query : Query<&Health, With<Player>>,
    mut hud_layer: ResMut<HudLayer>) {

    if let Some(health) = query.iter().nth(0) {
        let fraction = (health.current as f32) / (health.max as f32);
        let text = format!("{} / {}", health.current, health.max);
        hud_layer.hud_elements.push(HudElement::HealthBar { fraction, text });
    }
}