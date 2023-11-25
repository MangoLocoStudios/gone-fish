use crate::events::{CatchFishEvent, DepositFishEvent, DropFishEvent, ReelingFishEvent, TrashCollisionEvent};
use crate::GameState::Game;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use crate::components::BGMPlayer;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                check_for_catch_fish_events,
                check_for_fish_deposit_events,
                check_for_drop_fish_events,
                check_for_trash_collision_events,
                check_for_reeling_events,
            )
                .run_if(in_state(Game)),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/bg-1.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new_absolute(0.5),
                ..default()
            }
        },
        BGMPlayer,
    ));
}

fn handle_audio_events<S, T>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<T>,
    audio_path: S,
) where
    T: Event,
    S: Into<&'static str> + std::marker::Copy,
{
    for _ in event_reader.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load(audio_path.into()),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new_absolute(0.5),
                ..default()
            },
        });
    }
}

fn check_for_catch_fish_events(
    commands: Commands,
    asset_server: Res<AssetServer>,
    catch_fish_event: EventReader<CatchFishEvent>,
) {
    handle_audio_events(commands, asset_server, catch_fish_event, "audio/pop-2.ogg");
}

fn check_for_drop_fish_events(
    commands: Commands,
    asset_server: Res<AssetServer>,
    drop_fish_event: EventReader<DropFishEvent>,
) {
    handle_audio_events(commands, asset_server, drop_fish_event, "audio/drop-2.ogg");
}

fn check_for_fish_deposit_events(
    commands: Commands,
    asset_server: Res<AssetServer>,
    deposit_fish_event: EventReader<DepositFishEvent>,
) {
    handle_audio_events(
        commands,
        asset_server,
        deposit_fish_event,
        "audio/pop-1.ogg",
    );
}

fn check_for_trash_collision_events(
    commands: Commands,
    asset_server: Res<AssetServer>,
    trash_collision_event: EventReader<TrashCollisionEvent>,
) {
    handle_audio_events(
        commands,
        asset_server,
        trash_collision_event,
        "audio/pop-1.ogg",
    );
}

fn check_for_reeling_events(
    commands: Commands,
    asset_server: Res<AssetServer>,
    trash_collision_event: EventReader<ReelingFishEvent>,
) {
    handle_audio_events(
        commands,
        asset_server,
        trash_collision_event,
        "audio/pop-1.ogg",
    );
}
