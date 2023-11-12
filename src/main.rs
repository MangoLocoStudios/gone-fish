pub mod components;
pub mod events;
pub mod fish;
pub mod player;
mod port;
pub mod resources;
pub mod rod;

use crate::port::PortPlugin;
use bevy::prelude::*;
use fish::FishPlugin;
use player::PlayerPlugin;
use rod::RodPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            RodPlugin,
            FishPlugin,
            PortPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
