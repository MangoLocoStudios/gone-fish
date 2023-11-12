use bevy::prelude::*;

#[derive(Component)]
pub struct Speed {
    pub current: f32,
}

#[derive(Component, Clone, Copy, Debug)]
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

#[derive(Component)]
pub struct FishStorage {
    pub current: f32,
    pub max: f32,
}

impl FishStorage {
    pub fn update_storage(new_current: f32, new_max: Option<f32>, storage: &mut Mut<Self>) {
        storage.current = new_current;

        if let Some(new_max) = new_max {
            storage.max = new_max
        }
    }
}

#[derive(Component)]
pub struct Invincibility {
    pub invincibility_timer: Timer,
}
