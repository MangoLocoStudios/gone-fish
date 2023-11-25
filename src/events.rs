use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
use crate::components::Weight;
use crate::fish::FishVariant;

#[derive(Event, Default)]
pub struct BoatCollisionEvent;

#[derive(Event)]
pub struct FishCollisionWithRodEvent {
    pub fish: Entity,
}

#[derive(Event, Default)]
pub struct TrashCollisionEvent;

#[derive(Event)]
pub struct PortCollisionEvent {
    pub collision_direction: Collision,
}

#[derive(Event)]
pub struct DepositFishEvent;

#[derive(Event)]
pub struct CatchFishEvent {
    pub weight: Weight,
    pub fish_variant: FishVariant
}
