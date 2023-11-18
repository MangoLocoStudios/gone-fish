pub mod components;
pub mod events;
pub mod fish;
pub mod game;
pub mod menu;
pub mod player;
pub mod port;
pub mod resources;
pub mod rod;
pub mod shaders;
pub mod systems;
pub mod trash;

#[allow(unused_imports)] // TODO: Remove this once shaders are implemented
use bevy::sprite::MaterialMesh2dBundle;

use crate::components::{AnimationIndices, AnimationTimer};
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;
use crate::port::Port;
use crate::rod::Rod;
use crate::systems::animate_sprite;
use crate::GameState::Game;
use bevy::sprite::Material2dPlugin;
use bevy::{prelude::*, window::WindowTheme};
use shaders::GradientMaterial;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

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
            Material2dPlugin::<GradientMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (camera.run_if(in_state(Game)), animate_sprite))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut _shader: ResMut<Assets<GradientMaterial>>, // TODO: Remove '_' once shaders are implemented
    window: Query<&mut Window>,
    mut _meshes: ResMut<Assets<Mesh>>, // TODO: Remove '_' once shaders are implemented
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
            translation: Vec3::new(0., -550., 0.),
            scale: Vec3::new(5000., 1000., 0.),
            ..default()
        },
        ..default()
    });

    // Water shader
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes
    //         .add(
    //             shape::Quad {
    //                 size: Vec2::new(5000., 655.),
    //                 ..Default::default()
    //             }
    //             .into(),
    //         )
    //         .into(),
    //     transform: Transform {
    //         translation: Vec3::new(0., -397., 0.),
    //         ..default()
    //     },
    //     material: shader.add(GradientMaterial {}),
    //     ..default()
    // });

    // Test shader square
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes
    //         .add(
    //             shape::Quad {
    //                 size: Vec2::new(300., 300.),
    //                 ..Default::default()
    //             }
    //             .into(),
    //         )
    //         .into(),
    //     transform: Transform {
    //         translation: Vec3::new(200., 0., 40.),
    //         ..default()
    //     },
    //     material: shader.add(GradientMaterial {}),
    //     ..default()
    // });
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

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
