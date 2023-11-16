use bevy::prelude::*;

use super::{despawn_screen, GameState, Volume};
use crate::{
    fish::FishPlugin, player::PlayerPlugin, port::PortPlugin, rod::RodPlugin, trash::TrashPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_plugins((PlayerPlugin, RodPlugin, FishPlugin, PortPlugin, TrashPlugin))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup(_volume: Res<Volume>) {}
