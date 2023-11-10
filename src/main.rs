pub mod fish;
pub mod directions;
pub mod components;

use bevy::prelude::*;
use fish::FishPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FishPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
