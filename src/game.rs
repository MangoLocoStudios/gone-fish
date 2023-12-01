use bevy::prelude::*;

use super::{despawn_screen, GameState};
use crate::{
    fish::FishPlugin, player::PlayerPlugin, port::PortPlugin, rod::RodPlugin,
    speech::PlayerTextPlugin, trash::TrashPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_plugins((
                PlayerPlugin,
                RodPlugin,
                FishPlugin,
                PortPlugin,
                TrashPlugin,
                PlayerTextPlugin,
            ))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup() {}
