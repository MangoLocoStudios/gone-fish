use bevy::prelude::*;

use crate::directions::Directions;

#[derive(Component)]
pub struct Direction {
    pub direction: Directions,
}

#[derive(Component)]
pub struct Speed {
    pub current: f32,
}

#[derive(Component)]
pub struct Weight {
    pub current: f32,
}
