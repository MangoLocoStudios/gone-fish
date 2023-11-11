use crate::fish::ThingsFishCanCollideWith;
use bevy::prelude::*;

#[derive(Event, Default)]
pub struct BoatCollisionEvent;

#[derive(Event)]
pub struct FishCollisionEvent {
    pub fish: Entity,
    pub entity: ThingsFishCanCollideWith,
}

#[derive(Event, Default)]
pub struct TrashCollisionEvent;
