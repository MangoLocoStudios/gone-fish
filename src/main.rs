use bevy::prelude::*;
use player::{Player, PlayerMovementPlugin};
use rod::RodPlugin;
use crate::fish_storage::FishStorage;

pub mod player;
pub mod rod;
pub mod fish_storage;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerMovementPlugin, RodPlugin))
        .add_systems(Startup, setup)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        FishStorage {
            current: 0,
            max: 15,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 50.0, 0.0),
                scale: Vec3::new(100.0, 50.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
}
