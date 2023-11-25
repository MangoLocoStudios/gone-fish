use crate::components::BGMPlayer;
use crate::events::{
    CatchFishEvent, DepositFishEvent, DropFishEvent, ReelingFishEvent, TrashCollisionEvent,
};
use crate::resources::AudioSettings;
use crate::GameState::Game;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioSettings>()
            .add_systems(Startup, setup)
            .add_systems(
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_settings: Res<AudioSettings>,
) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/bg-1.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new_absolute(audio_settings.volume),
                ..default()
            },
        },
        BGMPlayer,
    ));
}

fn handle_audio_events<S, T>(
    mut commands: Commands,
    world: &World,
    mut event_reader: EventReader<T>,
    audio_path: S,
) where
    T: Event,
    S: Into<&'static str> + std::marker::Copy,
{
    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let audio_settings = world.get_resource::<AudioSettings>().unwrap();

    for _ in event_reader.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load(audio_path.into()),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::new_absolute(audio_settings.volume),
                ..default()
            },
        });
    }
}

fn check_for_catch_fish_events(
    commands: Commands,
    world: &World,
    catch_fish_event: EventReader<CatchFishEvent>,
) {
    handle_audio_events(commands, world, catch_fish_event, "audio/pop-2.ogg");
}

fn check_for_drop_fish_events(
    commands: Commands,
    world: &World,
    drop_fish_event: EventReader<DropFishEvent>,
) {
    handle_audio_events(commands, world, drop_fish_event, "audio/drop-2.ogg");
}

fn check_for_fish_deposit_events(
    commands: Commands,
    world: &World,
    deposit_fish_event: EventReader<DepositFishEvent>,
) {
    handle_audio_events(commands, world, deposit_fish_event, "audio/pop-1.ogg");
}

fn check_for_trash_collision_events(
    commands: Commands,
    world: &World,
    trash_collision_event: EventReader<TrashCollisionEvent>,
) {
    handle_audio_events(commands, world, trash_collision_event, "audio/pop-1.ogg");
}

fn check_for_reeling_events(
    commands: Commands,
    world: &World,
    trash_collision_event: EventReader<ReelingFishEvent>,
) {
    handle_audio_events(commands, world, trash_collision_event, "audio/pop-1.ogg");
}
