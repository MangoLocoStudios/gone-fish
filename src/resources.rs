use bevy::prelude::*;

use crate::{components::Weight, fish::FishVariant};

#[derive(Resource, Default)]
pub struct FishStored {
    pub fish: Vec<(FishVariant, Weight)>,
}
