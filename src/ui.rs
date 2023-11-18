use bevy::{app::AppExit, prelude::*};

use super::{GameState, TEXT_COLOR};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), ui_setup);
    }
}

// Keeping in case needed later
enum UIState {}

fn ui_setup(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "Gone Fish",
                TextStyle {
                    font_size: 80.0,
                    color: TEXT_COLOR,
                    ..default()
                },
            )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
        );
    });
}