pub mod components;
pub mod events;
pub mod fish;
pub mod fish_storage;
pub mod player;
pub mod rod;

use bevy::prelude::*;
use fish::FishPlugin;
use player::PlayerPlugin;
use rod::RodPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, RodPlugin, FishPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
