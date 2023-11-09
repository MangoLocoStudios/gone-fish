use bevy::prelude::*;
use crate::systems::game::GamePlugin;

pub mod player;
pub mod rod;
pub mod fish_storage;
pub mod port;
pub mod systems;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        .run();
}
