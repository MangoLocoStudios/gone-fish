use crate::components::DecayTimer;
use crate::events::{CatchFishEvent, DepositFishEvent, UpgradeEvent, WeightLimitEvent};
use crate::player::Player;
use crate::GameState::Game;
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

#[derive(Component)]
pub struct PlayerText;

pub struct PlayerTextPlugin;

impl Plugin for PlayerTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeightLimitEvent>().add_systems(
            Update,
            (
                despawn_player_text,
                check_for_weight_limit_event,
                check_for_catch_fish_collision_event,
                check_for_fish_deposit_event,
                check_for_upgrade_event,
            )
                .run_if(in_state(Game)),
        );
    }
}

pub fn generate_text_entity(
    commands: &mut Commands,
    text: String,
    style: TextStyle,
    relative_position: Vec3,
    scale: Vec3,
    duration: Duration,
) -> Entity {
    commands
        .spawn((
            Text2dBundle {
                text: Text::from_sections([TextSection::new(text, style)])
                    .with_alignment(TextAlignment::Center),
                transform: Transform {
                    translation: relative_position,
                    scale,
                    ..default()
                },
                ..default()
            },
            DecayTimer {
                timer: Timer::from_seconds(duration.as_secs() as f32, TimerMode::Once),
            },
            PlayerText,
        ))
        .id()
}

pub fn despawn_player_text(
    mut commands: Commands,
    mut text_query: Query<(&DecayTimer, Entity), With<PlayerText>>,
) {
    for (timer, entity) in &mut text_query {
        if timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn check_for_weight_limit_event(
    mut commands: Commands,
    mut ev_weight_limit: EventReader<WeightLimitEvent>,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
) {
    let player = player_query.single();

    let ui_text_style = TextStyle {
        font: asset_server.load("fonts/Pixellari.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    let speech_options = [
        "Boat's loaded, time to reel in\nand head for port!",
        "Can't squeeze in more, let's\nsail back with this haul.",
        "Full to the brim, setting course\nfor the harbor now.",
        "No room left for more, homeward\nbound with the catch.",
        "Deck's overflowing, navigating\nback to the harbor docks.",
        "Caught plenty today, time to\nmake our way to port.",
        "Hold's packed, steering back to\nunload our fish bounty.",
        "Maxed out on fish, charting a\ncourse for the harbor.",
        "Overflowing nets, returning to port\nwith the day's catch.",
        "Boat's heavy with fish, heading\nback to port to unload.",
    ];

    for _ in ev_weight_limit.read() {
        let text = generate_text_entity(
            &mut commands,
            speech_options[rand::thread_rng().gen_range(0..speech_options.len() - 1)].into(),
            ui_text_style.clone(),
            Vec3::new(40., 30., 0.),
            Vec3::splat(0.25),
            Duration::new(5, 0),
        );

        commands.entity(player).push_children(&[text]);
    }
}

fn check_for_upgrade_event(
    mut commands: Commands,
    mut ev_upgrade: EventReader<UpgradeEvent>,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
) {
    let player = player_query.single();

    let ui_text_style = TextStyle {
        font: asset_server.load("fonts/Pixellari.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    let speech_options = [
        "Upgraded gear!\nReady to cast with this new fishing rod.",
        "New boat, better rod!\nTime to reel in bigger catches.",
        "Improved rig, setting sail on\nthe upgraded fishing vessel.",
        "Upgraded to prime gear.\nLet's fish with style now.",
        "New rod, upgraded boat.\nChasing bigger fish today.",
        "Enhanced tools, navigating the\nwaters with top-notch equipment.",
        "Geared up! New boat and rod,\naiming for a record-breaking haul.",
        "Upgrade complete!\nFishing with precision in the improved vessel.",
        "New gear in hand, casting off for\na day of fishing excellence.",
        "Leveling up! New boat, superior rod.\nFishing game strong.",
    ];

    for _ in ev_upgrade.read() {
        let text = generate_text_entity(
            &mut commands,
            speech_options[rand::thread_rng().gen_range(0..speech_options.len() - 1)].into(),
            ui_text_style.clone(),
            Vec3::new(75., 75., 0.),
            Vec3::splat(0.25),
            Duration::new(5, 0),
        );

        commands.entity(player).push_children(&[text]);
    }
}

fn check_for_catch_fish_collision_event(
    mut commands: Commands,
    mut ev_catch_fish: EventReader<CatchFishEvent>,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
) {
    let player = player_query.single();

    let ui_text_style = TextStyle {
        font: asset_server.load("fonts/Pixellari.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    for fish in ev_catch_fish.read() {
        let text = generate_text_entity(
            &mut commands,
            format!("+ {:.2} kg", fish.weight.current),
            ui_text_style.clone(),
            Vec3::new(50., 15., 0.),
            Vec3::splat(0.25),
            Duration::new(1, 0),
        );

        commands.entity(player).push_children(&[text]);
    }
}

fn check_for_fish_deposit_event(
    mut commands: Commands,
    mut deposit_fish_event: EventReader<DepositFishEvent>,
    asset_server: Res<AssetServer>,
    player_query: Query<Entity, With<Player>>,
) {
    let player = player_query.single();

    let ui_text_style = TextStyle {
        font: asset_server.load("fonts/Pixellari.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    let speech_options = [
        "Fresh start! Ready for another round,\nbaiting the hooks.",
        "Empty hold, gearing up for\nthe next big catch.",
        "Cleared the deck, setting out for\nanother fruitful fishing.",
        "Unload complete, time to cast\nagain for a new haul.",
        "Back at sea, hooks in the water\nfor more fish.",
        "Emptied the nets, let's head back\nand cast anew.",
        "Port visit done, preparing for\nthe next fishing adventure.",
        "Fresh bait, clear skies – setting\nsail for another bounty.",
        "Docked and done, now back to\nthe open waters.",
        "Recharged and refueled, ready to\nreel in the next load.",
    ];

    for _ in deposit_fish_event.read() {
        let text = generate_text_entity(
            &mut commands,
            speech_options[rand::thread_rng().gen_range(0..speech_options.len() - 1)].into(),
            ui_text_style.clone(),
            Vec3::new(40., 30., 0.),
            Vec3::splat(0.25),
            Duration::new(5, 0),
        );

        commands.entity(player).push_children(&[text]);
    }
}
