use bevy::prelude::*;

#[derive(Component)]
pub struct Speed {
    pub current: f32,
}

#[derive(Component)]
pub struct Weight {
    pub current: f32,
}

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn random_y() -> Self {
        match rand::random::<bool>() {
            true => Self::Left,
            false => Self::Right,
        }
    }

    pub fn random_x() -> Self {
        match rand::random::<bool>() {
            true => Self::Up,
            false => Self::Down,
        }
    }
}
