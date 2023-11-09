use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
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
}
