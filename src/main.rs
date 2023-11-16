pub mod components;
pub mod events;
pub mod fish;
pub mod player;
pub mod port;
pub mod resources;
pub mod rod;
pub mod systems;
pub mod trash;

use crate::port::PortPlugin;
use bevy::{prelude::*, window::WindowTheme};
use components::{AnimationIndices, AnimationTimer};
use fish::FishPlugin;
use player::PlayerPlugin;
use port::Port;
use rod::{Rod, RodPlugin};
use trash::TrashPlugin;

fn main() {
    App::new()
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
            PlayerPlugin,
            RodPlugin,
            FishPlugin,
            PortPlugin,
            TrashPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, camera)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Water
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::hex("#7287D5").expect("is a valid colour."),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., -550., 0.),
            scale: Vec3::new(5000., 1000., 0.),
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
}

fn camera(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    port_query: Query<&Transform, (With<Port>, Without<Rod>, Without<Camera2d>)>,
) {
    let port = port_query.single();
    let (camera_transform, mut camera) = camera_query.single_mut();

    let diff = port.translation.x - camera_transform.translation.x;
    camera.scale = (diff.abs() / 1000.) + 0.5;
    camera.scale = camera.scale.clamp(0.5, 3.);
}
