mod audio;
mod camera;
pub mod components;
pub mod events;
pub mod fish;
pub mod game;
pub mod menu;
pub mod player;
pub mod port;
pub mod resources;
pub mod rod;
pub mod speech;
pub mod systems;
pub mod trash;
mod ui;

use crate::audio::AudioPlugin;
use crate::camera::CameraPlugin;
use crate::components::{AnimationIndices, AnimationTimer};
use crate::game::GamePlugin;
use crate::menu::MenuPlugin;
use crate::systems::animate_sprite;
use crate::ui::UIPlugin;
use crate::GameState::Game;
use audio::Volume;
use bevy::{prelude::*, window::WindowTheme};
use systems::{pause_the_game, tick_decay_timers};

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .insert_resource(Volume(0.5))
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
            CameraPlugin,
            MenuPlugin,
            GamePlugin,
            UIPlugin,
            AudioPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                tick_decay_timers.run_if(in_state(Game)),
                animate_sprite,
                pause_the_game,
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

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
