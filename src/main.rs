mod fish;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, fish::update_fish)
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
