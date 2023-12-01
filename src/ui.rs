use super::GameState;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(GameState::Game), ui_setup);
    }
}

#[derive(Component)]
struct StorageText;

#[derive(Component)]
struct ControlsText;

#[derive(Component)]
struct StorageIcon;

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let control_text_style = TextStyle {
        font: asset_server.load("fonts/Pixellari.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
    };

    // UI Canvas
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        // Controls
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_sections([TextSection::new(
                            "Left/Right Arrows - Move boat\nDown Arrow - Cast rod\nSpace - Jerk rod upwards\nEscape - Pause the game",
                            control_text_style,
                        )]),
                        ControlsText,
                    ));
                });
        });
}
