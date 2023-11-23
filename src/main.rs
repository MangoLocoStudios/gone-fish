pub mod components;
pub mod events;
pub mod fish;
pub mod game;
pub mod menu;
pub mod player;
pub mod port;
pub mod resources;
pub mod rod;
pub mod systems;
pub mod trash;
mod ui;

use crate::components::{AnimationIndices, AnimationTimer, CameraShake};
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;
use crate::port::Port;
use crate::rod::Rod;
use crate::systems::animate_sprite;
use crate::ui::UIPlugin;
use crate::GameState::Game;
use bevy::{prelude::*, window::WindowTheme};
use noise::{NoiseFn, Perlin};
use rand::Rng;
use systems::tick_decay_timers;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const MAX_SHAKE_OFFSET: f32 = 50.;
const MAX_SHAKE_ANGLE: f32 = 5.;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins
                .build()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Gone Fish".into(),
                        resolution: (1280., 720.).into(),
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            MenuPlugin,
            GamePlugin,
            UIPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (camera, shake_camera, tick_decay_timers).run_if(in_state(Game)),
                animate_sprite,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window: Query<&mut Window>,
) {
    let window = window.single();
    // From center of screen.
    let window_width = window.resolution.width() / 2.;

    commands.spawn(Camera2dBundle::default());

    // Sky
    commands.spawn(SpriteBundle {
        texture: asset_server.load("craftpix/clouds/clouds_7/1.png"),
        transform: Transform {
            translation: Vec3::new(0., 0., -20.),
            scale: Vec3::splat(7.),
            ..default()
        },
        ..default()
    });

    // Border barrel
    commands.spawn(SpriteBundle {
        texture: asset_server.load("craftpix/objects/Fishbarrel3.png"),
        transform: Transform {
            translation: Vec3::new(window_width + 200., -40., 0.),
            scale: Vec3::splat(2.),
            ..default()
        },
        ..default()
    });

    // Water top
    let mut initial = -1000.;
    for _ in 0..100 {
        let texture_handle = asset_server.load("critters/water_tile.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 32, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 31 };

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform {
                    translation: Vec3::new(initial, -46., 3.),
                    scale: Vec3::splat(3.),
                    ..default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        ));

        initial += 48.;
    }

    // Water
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::hex("#7287D5").expect("is a valid colour."),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., -2050., 0.),
            scale: Vec3::new(5000., 4000., 0.),
            ..default()
        },
        ..default()
    });
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
                new_transform.y *= -1.;
            }
        }
    }

    if camera_transform.translation.y < 0. {
        new_transform.y = 100.;
    }

    camera_transform.translation += new_transform * time.delta_seconds();
}

fn shake_camera(
    mut commands: Commands,
    time: Res<Time>,
    camera_query: Query<(&mut Transform, Entity, &mut CameraShake), With<Camera2d>>,
) {
    if camera_query.is_empty() {
        return;
    }

    let (mut camera_transform, camera_entity, mut camera_shake) = camera_query.single_mut();

    let mut new_transform = Vec3::new(0., 0., 0.);

    // Create a Perlin noise generator
    let perlin = Perlin::new();

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

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
