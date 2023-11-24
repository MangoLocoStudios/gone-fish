use bevy::audio::Volume;
use bevy::prelude::*;
use crate::components::{BGMPlayer, FoleyPlayer};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/Windless Slopes.ogg"),
            settings: PlaybackSettings {
                volume: Volume::new_absolute(0.5),
                ..default()
            }
        },
        BGMPlayer,
    ));

    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/Windless Slopes.ogg"),
            settings: PlaybackSettings {
                volume: Volume::new_absolute(0.5),
                ..default()
            }
        },
        FoleyPlayer,
    ));
}