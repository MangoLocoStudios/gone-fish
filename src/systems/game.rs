use bevy::prelude::*;

use crate::player::PlayerMovementPlugin;
use crate::rod::RodPlugin;
use crate::fish_storage::FishStorage;
use crate::player::Player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerMovementPlugin, RodPlugin))
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        FishStorage {
            current: 0,
            max: 10
        },
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