use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    events::{BoatCollisionEvent, FishCollisionWithRodEvent, TrashCollisionEvent},
    fish::Fish,
    player::Player,
    resources::RodProperties,
    trash::Trash,
    GameState::Game,
};

#[derive(Component)]
enum RodState {
    Idle,
    Reeling,
}

#[derive(Component)]
pub struct Rod;

pub struct RodPlugin;

impl Plugin for RodPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RodProperties>()
            .add_event::<BoatCollisionEvent>()
            .add_systems(
                Update,
                (
                    cast_rod,
                    rod_movement,
                    check_for_boat_collisions,
                    check_for_fish_collisions,
                    check_for_trash_collisions,
                )
                .run_if(in_state(Game)),
            );
    }
}

const ROD_MOVEMENT_DOWN: f32 = 75.0;

fn cast_rod(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    rod: Query<&Rod>,
    player_position: Query<&Transform, With<Player>>,
) {
    let player = player_position.single();

    // Only spawn a new rod if there isn't already one spawned
    if rod.get_single().is_ok() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        commands.spawn((
            Rod,
            RodState::Idle,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.75, 0.25),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(player.translation.x, -5.0, 0.0),
                    scale: Vec3::new(40.0, 40.0, 0.0),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn check_for_boat_collisions(
    mut commands: Commands,
    rod_query: Query<(Entity, &Transform), (With<Rod>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    mut boat_collision_event: EventWriter<BoatCollisionEvent>,
) {
    let player = player_query.single();
    let (rod_entity, rod) = match rod_query.get_single() {
        Ok((rod_entity, rod)) => (rod_entity, rod),
        Err(_) => return,
    };

    if collide(
        player.translation,
        player.scale.truncate(),
        rod.translation,
        rod.scale.truncate(),
    )
    .is_none()
    {
        return;
    }

    boat_collision_event.send_default();

    // Despawn rod when it's reeled back in
    commands.entity(rod_entity).despawn();
}

fn check_for_fish_collisions(
    mut rod_query: Query<(&Transform, &mut RodState), With<Rod>>,
    fish_query: Query<(Entity, &Transform), With<Fish>>,
    mut collision_events: EventWriter<FishCollisionWithRodEvent>,
) {
    let (rod, mut state) = match rod_query.get_single_mut() {
        Ok((rod, state)) => (rod, state),
        Err(_) => return,
    };

    for (fish, fish_transform) in &fish_query {
        if collide(
            fish_transform.translation,
            fish_transform.scale.truncate(),
            rod.translation,
            rod.scale.truncate(),
        )
        .is_none()
        {
            continue;
        }

        match *state {
            RodState::Idle => {
                collision_events.send(FishCollisionWithRodEvent { fish });
                *state = RodState::Reeling;
            }
            RodState::Reeling => {}
        }
    }
}

fn check_for_trash_collisions(
    mut rod_query: Query<(&Transform, &mut RodState), With<Rod>>,
    trash_query: Query<&Transform, With<Trash>>,
    mut collision_events: EventWriter<TrashCollisionEvent>,
) {
    let (rod, mut state) = match rod_query.get_single_mut() {
        Ok((rod, state)) => (rod, state),
        Err(_) => return,
    };

    for trash_transform in &trash_query {
        if collide(
            trash_transform.translation,
            trash_transform.scale.truncate(),
            rod.translation,
            rod.scale.truncate(),
        )
        .is_none()
        {
            continue;
        }

        match *state {
            RodState::Idle => {}
            RodState::Reeling => {
                collision_events.send_default();
                *state = RodState::Idle;
            }
        }
    }
}

fn rod_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    rod_properties: Res<RodProperties>,
    mut rod_query: Query<&mut Transform, (With<Rod>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player = player_query.single();
    let mut transform = match rod_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    // Move rod up
    if keyboard_input.just_pressed(KeyCode::Space) {
        transform.translation.y += rod_properties.pull;
    }

    // Keep rod x aligned with player
    transform.translation.x = player.translation.x;

    // Constantly move the rod downwards as long as it's above
    // the length of the rod
    if transform.translation.y > (0.0 - rod_properties.length) {
        transform.translation.y -= ROD_MOVEMENT_DOWN * time.delta_seconds();
    }
}
