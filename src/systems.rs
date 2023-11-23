use bevy::prelude::*;

use crate::components::{AnimationIndices, AnimationTimer, DecayTimer};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index >= indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn tick_decay_timers(mut can_die_query: Query<&mut DecayTimer>, timer: ResMut<Time>) {
    for mut decay_timer in can_die_query.iter_mut() {
        decay_timer.timer.tick(timer.delta());
    }
}
