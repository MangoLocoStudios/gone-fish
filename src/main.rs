pub mod fish;
pub mod direction;
pub mod speed;

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
