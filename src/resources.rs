use crate::{components::Weight, fish::FishVariant};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct FishStored {
    pub fish: Vec<(FishVariant, Weight)>,
}
