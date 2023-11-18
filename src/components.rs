use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Speed {
    pub current: f32,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Weight {
    pub current: f32,
}

#[derive(Component, Clone, Copy)]
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

    pub fn opposite(self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
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

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub(crate) Timer);

#[derive(Component)]
pub struct CanDie {
    pub dying: bool,
}

#[derive(Component)]
pub struct DecayTimer {
    pub timer: Timer,
}

impl Default for DecayTimer {
    fn default() -> Self {
        let time_to_death = rand::thread_rng().gen_range(30. ..120.);
        DecayTimer {
            timer: Timer::from_seconds(time_to_death, TimerMode::Once),
        }
    }
}
