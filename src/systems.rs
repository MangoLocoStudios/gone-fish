use bevy::prelude::*;

use crate::{
    components::{AnimationIndices, AnimationTimer, DecayTimer, PauseMenu},
    menu::MenuState,
    GameState,
};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn tick_decay_timers(mut can_die_query: Query<&mut DecayTimer>, timer: ResMut<Time>) {
    for mut decay_timer in can_die_query.iter_mut() {
        decay_timer.timer.tick(timer.delta());
    }
}

pub fn pause_the_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    menu_state: ResMut<State<MenuState>>,
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<Camera>>,
) {
    if *menu_state != MenuState::Disabled {
        return;
    }

    let camera = camera_query.single();

    let ui_text_style = TextStyle {
        font: asset_server.load("fonts/Pixellari.ttf"),
        font_size: 70.0,
        color: Color::WHITE,
    };

    if keyboard_input.just_pressed(KeyCode::Escape) {
        match *game_state.get() {
            GameState::Menu => {
                let pause_menu = pause_menu_query.single();
                commands.entity(pause_menu).despawn();

                *game_state = State::new(GameState::Game);
            }
            GameState::Game => {
                *game_state = State::new(GameState::Menu);
                let pause_text = commands
                    .spawn((
                        Text2dBundle {
                            text: Text::from_sections([TextSection::new("Paused", ui_text_style)])
                                .with_alignment(TextAlignment::Center),
                            transform: Transform {
                                translation: Vec3::new(0., 10., 100.),
                                scale: Vec3::splat(1.),
                                ..default()
                            },
                            ..default()
                        },
                        PauseMenu,
                    ))
                    .id();
                commands.entity(camera).push_children(&[pause_text]);
            }
        }
    }
}
