use crate::components::{BGMPlayer, FoleyPlayer};
use crate::events::{CatchFishEvent, DepositFishEvent, DropFishEvent};
use crate::GameState::Game;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                check_for_catch_fish_events,
                check_for_fish_deposit_events,
                check_for_drop_fish_events,
            )
                .run_if(in_state(Game)),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((
    //     AudioBundle {
    //         source: asset_server.load("sounds/Windless Slopes.ogg"),
    //         settings: PlaybackSettings {
    //             volume: Volume::new_absolute(0.5),
    //             ..default()
    //         }
    //     },
    //     BGMPlayer,
    // ));
}

fn check_for_catch_fish_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut catch_fish_event: EventReader<CatchFishEvent>,
) {
    for _ in catch_fish_event.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/pop-2.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new_absolute(0.5),
                ..default()
            },
        });
    }
}

fn check_for_drop_fish_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut drop_fish_event: EventReader<DropFishEvent>,
) {
    for _ in drop_fish_event.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/drop-2.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new_absolute(0.5),
                ..default()
            },
        });
    }
}

fn check_for_fish_deposit_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut deposit_fish_event: EventReader<DepositFishEvent>,
) {
    for _ in deposit_fish_event.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/pop-1.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new_absolute(0.5),
                ..default()
            },
        });
    }
}
