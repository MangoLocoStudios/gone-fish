use crate::fish::ThingsFishCanCollideWith;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

#[derive(Event, Default)]
pub struct BoatCollisionEvent;

#[derive(Event)]
pub struct FishCollisionEvent {
    pub fish: Entity,
    pub entity: ThingsFishCanCollideWith,
}

#[derive(Event, Default)]
pub struct TrashCollisionEvent;

#[derive(Event)]
pub struct PortCollisionEvent {
    pub collision_direction: Collision
}
