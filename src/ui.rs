use crate::components::FishStorage;
use crate::player::Player;
use crate::GameState::Game;
use bevy::prelude::*;

use super::GameState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(OnEnter(GameState::Game), ui_setup)
            .add_systems(Update, update_storage_ui.run_if(in_state(Game)));
    }
}

// Keeping in case needed later
enum UIState {}

#[derive(Component)]
struct StorageText;

#[derive(Component)]
struct StorageIcon;

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_text_style = TextStyle {
        font: asset_server.load("fonts/Pixellari.ttf"),
        font_size: 40.0,
        color: Color::GOLD,
        ..default()
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
        // Fish Storage
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(25.),
                        padding: UiRect::all(Val::Px(25.)),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Px(40.0),
                                height: Val::Px(40.0),
                                ..default()
                            },
                            image: UiImage::new(asset_server.load("craftpix/icons/Icons_16.png")),
                            ..default()
                        },
                        StorageIcon,
                    ));

                    parent.spawn((
                        // Create a TextBundle that has a Text with a list of sections.
                        TextBundle::from_sections([
                            TextSection::from_style(ui_text_style.clone()),
                            TextSection::from_style(ui_text_style.clone()),
                        ]),
                        StorageText,
                    ));
                });
        });
}

fn update_storage_ui(
    fish_storage_query: Query<&FishStorage, With<Player>>,
    mut storage_ui_query: Query<&mut Text, With<StorageText>>,
) {
    let mut stui = storage_ui_query.single_mut();
    let fish_storage = fish_storage_query.single();

    stui.sections[0].value = format!("{} kg /", fish_storage.current);
    stui.sections[1].value = format!(" {} kg", fish_storage.max);
}
