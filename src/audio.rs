use crate::components::BGMPlayer;
use crate::events::{
    CatchFishEvent, DepositFishEvent, DropFishEvent, ReelingFishEvent, TrashCollisionEvent,
};
use crate::GameState::Game;
use bevy::audio::{PlaybackMode, Volume as BevyVolume};
use bevy::prelude::*;

pub struct AudioPlugin;

#[derive(Resource, Debug, Component, PartialEq, Clone, Copy)]
pub struct Volume(pub(crate) f32);

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
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
            )
            .add_systems(Update, check_for_volume_events);
    }
}

fn setup(volume: Res<Volume>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/bg-1.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: BevyVolume::new_absolute(volume.0),
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
    let asset_server = world
        .get_resource::<AssetServer>()
        .expect("AssetServer to exist.");
    let volume = world.get_resource::<Volume>().expect("Volume to exist.");

    for _ in event_reader.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load(audio_path.into()),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: BevyVolume::new_absolute(volume.0),
                ..default()
            },
        });
    }
}

fn check_for_volume_events(
    volume: Res<Volume>,
    music_controller: Query<&AudioSink, With<BGMPlayer>>,
) {
    if let Ok(sink) = music_controller.get_single() {
        sink.set_volume(volume.0);
    }
}

fn check_for_catch_fish_events(
    commands: Commands,
    world: &World,
    ev_catch_fish: EventReader<CatchFishEvent>,
) {
    handle_audio_events(commands, world, ev_catch_fish, "audio/pop-2.ogg");
}

fn check_for_drop_fish_events(
    commands: Commands,
    world: &World,
    ev_drop_fish: EventReader<DropFishEvent>,
) {
    handle_audio_events(commands, world, ev_drop_fish, "audio/drop-2.ogg");
}

fn check_for_fish_deposit_events(
    commands: Commands,
    world: &World,
    ev_deposit_fish: EventReader<DepositFishEvent>,
) {
    handle_audio_events(commands, world, ev_deposit_fish, "audio/pop-1.ogg");
}

fn check_for_trash_collision_events(
    commands: Commands,
    world: &World,
    ev_trash_collision: EventReader<TrashCollisionEvent>,
) {
    handle_audio_events(commands, world, ev_trash_collision, "audio/pop-1.ogg");
}

fn check_for_reeling_events(
    commands: Commands,
    world: &World,
    ev_reeling_fish: EventReader<ReelingFishEvent>,
) {
    handle_audio_events(commands, world, ev_reeling_fish, "audio/pop-1.ogg");
}
