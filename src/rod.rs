use crate::player::Player;
use bevy::{prelude::*, sprite::collide_aabb::collide};

// We may not need these events but i'll keep them here for now.
#[derive(Event, Default)]
struct BoatCollisionEvent;

#[derive(Event, Default)]
struct FishCollisionEvent;

#[derive(Event, Default)]
struct TrashCollisionEvent;

#[derive(Component)]
pub struct Rod;

pub struct RodPlugin;

impl Plugin for RodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoatCollisionEvent>()
            .add_systems(Update, (cast_rod, rod_movement, check_for_collisions));
    }
}

const ROD_LENGTH: f32 = 200.0;
const ROD_MOVEMENT_UP: f32 = 20.0;

fn cast_rod(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    rod: Query<&Rod>,
    player_position: Query<&Transform, With<Player>>,
) {
    let player = player_position.single();

    // Only spawn a new rod if there isn't already one spawned
    if let Ok(_) = rod.get_single() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        commands.spawn((
            Rod,
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

fn check_for_collisions(
    mut commands: Commands,
    rod_position: Query<(Entity, &Transform), (With<Rod>, Without<Player>)>,
    player_position: Query<&Transform, With<Player>>,
    mut collision_events: EventWriter<BoatCollisionEvent>,
) {
    let player = player_position.single();

    if let Ok((rod_entity, rod)) = rod_position.get_single() {
        // Despawn rod when it's reeled back in
        if let Some(_) = collide(
            player.translation,
            player.scale.truncate(),
            rod.translation,
            rod.scale.truncate(),
        ) {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            commands.entity(rod_entity).despawn();
        }
    }
}

fn rod_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut rod_position: Query<&mut Transform, (With<Rod>, Without<Player>)>,
    player_position: Query<&Transform, With<Player>>,
) {
    let player = player_position.single();

    if let Ok(mut transform) = rod_position.get_single_mut() {
        // Move rod up
        if keyboard_input.just_pressed(KeyCode::Space) {
            transform.translation.y += ROD_MOVEMENT_UP;
        }

        // Keep rod x aligned with player
        transform.translation.x = player.translation.x;

        // Constantly move the rod downwards as long as it's above
        // the length of the rod
        if transform.translation.y > (0.0 - ROD_LENGTH) {
            transform.translation.y -= 50.0 * time.delta_seconds();
        }
    }
}
