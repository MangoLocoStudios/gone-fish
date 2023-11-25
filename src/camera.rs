use crate::components::CameraShake;
use crate::port::Port;
use crate::rod::Rod;
use crate::GameState::Game;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub struct CameraPlugin;

const MAX_SHAKE_OFFSET: f32 = 50.;
const MAX_SHAKE_ANGLE: f32 = 5.;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, ((camera, shake_camera).run_if(in_state(Game)),));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    rod_query: Query<&Transform, (With<Rod>, Without<Camera2d>, Without<Port>)>,
    port_query: Query<&Transform, (With<Port>, Without<Rod>, Without<Camera2d>)>,
) {
    let rod = rod_query.get_single();
    let port = port_query.single();
    let (mut camera_transform, mut camera) = camera_query.single_mut();

    let diff = port.translation.x - camera_transform.translation.x;
    camera.scale = (diff.abs() / 1000.) + 0.5;
    camera.scale = camera.scale.clamp(0.5, 3.);

    let mut new_transform = Vec3::new(0., 0., camera_transform.translation.z);

    if let Ok(rod) = rod {
        if rod.translation.y < -205. {
            new_transform.y = 50.;

            if camera_transform.translation.y > rod.translation.y {
                new_transform.y = -50.;
            }
        } else if camera_transform.translation.y < 0. {
            new_transform.y = 100.;
        }
    } else if camera_transform.translation.y < 0. {
        new_transform.y = 100.;
    }

    camera_transform.translation += new_transform * time.delta_seconds();
}

fn shake_camera(
    mut commands: Commands,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, Entity, &mut CameraShake), With<Camera2d>>,
) {
    if camera_query.is_empty() {
        return;
    }

    let (mut camera_transform, camera_entity, mut camera_shake) = camera_query.single_mut();

    let mut new_transform = Vec3::new(0., 0., 0.);

    // Create a Perlin noise generator
    let perlin = Perlin::new(30);

    // Generate a random Vector3
    let mut rng = rand::thread_rng();
    let random_vector = [
        rng.gen_range(-100.0..100.0),
        rng.gen_range(-100.0..100.0),
        rng.gen_range(-100.0..100.0),
    ];

    // Use the random vector and time to generate Perlin noise
    let noise_value = perlin.get([random_vector[0], random_vector[1], random_vector[2]]);

    // Map the noise value to the range [-1, 1]
    let mapped_value = map_range(noise_value, -1.0, 1.0, -1.0, 1.0);

    let nz = MAX_SHAKE_ANGLE * camera_shake.intensity * mapped_value;
    new_transform.x = MAX_SHAKE_OFFSET * camera_shake.intensity * mapped_value;
    new_transform.y = MAX_SHAKE_OFFSET * camera_shake.intensity * mapped_value;

    camera_transform.translation += new_transform * time.delta_seconds();
    camera_transform.rotation.z += nz * time.delta_seconds();

    if camera_shake.shake_timer.tick(time.delta()).finished() {
        commands.entity(camera_entity).remove::<CameraShake>();
        camera_transform.rotation = Quat::IDENTITY;
        camera_transform.translation = camera_shake.start_translation;
    }
}

// Function to map a value from one range to another
fn map_range(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f32 {
    ((value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min) as f32
}
