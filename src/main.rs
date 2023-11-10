pub mod components;
pub mod directions;
pub mod fish;

use bevy::prelude::*;

use fish::FishPlugin;
use player::{Player, PlayerMovementPlugin};
use rod::RodPlugin;

pub mod player;
pub mod rod;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerMovementPlugin, RodPlugin, FishPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
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
