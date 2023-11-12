use bevy::prelude::*;
use std::collections::HashMap;

use crate::{components::Weight, fish::FishVariant};

#[derive(Resource, Default)]
pub struct PlayerFishStored {
    pub fish: Vec<(FishVariant, Weight)>,
}

#[derive(Resource, Default)]
pub struct AliveFish {
    pub count: u32,
}

#[derive(Resource)]
pub struct PortStorage {
    pub weight: f32,
    pub fish: HashMap<FishVariant, u32>,
}

impl Default for PortStorage {
    fn default() -> Self {
        Self {
            weight: 0.,
            fish: FishVariant::iterator()
                .map(|key| {
                    (*key, 0)
                })
                .collect(),
        }
    }
}
