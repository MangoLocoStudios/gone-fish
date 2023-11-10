mod fish;

use bevy::prelude::*;
use fish::FishPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_plugins(DefaultPlugins)
        .add_plugins(FishPlugin) // TODO: Make this plugin work :(
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((TextBundle::from_section(
        "Gone\nFish!",
        TextStyle {
            font_size: 100.0,
            ..default()
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(5.0),
        right: Val::Px(5.0),
        ..default()
    }),));

    fish::setup_fish(commands, asset_server);
}
