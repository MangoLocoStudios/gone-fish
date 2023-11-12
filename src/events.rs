use bevy::prelude::*;

#[derive(Event, Default)]
pub struct BoatCollisionEvent;

#[derive(Event)]
pub struct FishCollisionWithRodEvent {
    pub fish: Entity,
}

#[derive(Event, Default)]
pub struct TrashCollisionEvent;
