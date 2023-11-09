use bevy::prelude::*;
use player::{Player, PlayerMovementPlugin};
use rod::RodPlugin;

pub mod player;
pub mod rod;

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
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(100.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
            ..default()
        },
    ));
}
