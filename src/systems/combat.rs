use bevy_ecs::system::{Commands, Query, ResMut};

use crate::{Health, Messenger, WantsToAttack};

pub fn combat(
    mut query: Query<&mut Health>,
    mut attack_events: ResMut<Messenger<WantsToAttack>>,
    mut commands: Commands,
) {
    for event in attack_events.messages.iter() {
        if let Ok(mut health) = query.get_mut(event.victim) {
            health.current -= 1;

            if health.current <= 0 {
                commands.entity(event.victim).despawn();
            }
        }
    }

    attack_events.messages.clear();
}
