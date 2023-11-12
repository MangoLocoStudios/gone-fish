pub mod components;
pub mod events;
pub mod fish;
pub mod player;
pub mod port;
pub mod resources;
pub mod rod;
pub mod trash;

use crate::port::PortPlugin;
use bevy::prelude::*;
use fish::FishPlugin;
use player::PlayerPlugin;
use rod::RodPlugin;
use trash::TrashPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            RodPlugin,
            FishPlugin,
            PortPlugin,
            TrashPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
