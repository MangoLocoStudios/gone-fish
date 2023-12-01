use crate::events::{DepositFishEvent, UpgradeEvent};
use crate::{
    components::FishStorage,
    events::PortCollisionEvent,
    player::Player,
    resources::{PlayerFishStored, PortStorage},
    rod::RodVariant,
    GameState::Game,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Port;

#[derive(Component)]
pub struct PortUI;

pub struct PortPlugin;

impl Plugin for PortPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PortStorage>()
            .add_event::<PortCollisionEvent>()
            .add_event::<DepositFishEvent>()
            .add_event::<UpgradeEvent>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(Game), setup_port_ui)
            .add_systems(
                Update,
                (check_for_port_collisions, update_port_ui).run_if(in_state(Game)),
            );
    }
}

fn setup(
    mut commands: Commands,
    window: Query<&mut Window>,
    asset_server: Res<AssetServer>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let window = window.single();
    // From center of screen.
    let window_width = window.resolution.width() / 2.;

    commands.spawn((
        Port,
        SpriteBundle {
            texture: asset_server.load("craftpix/objects/Fishing_hut.png"),
            transform: Transform {
                translation: Vec3::new(-window_width, 0., -10.),
                scale: Vec3::splat(3.),
                ..default()
            },
            ..default()
        },
    ));

    let ui_text_style = TextStyle {
        font: asset_server.load("fonts/Pixellari.ttf"),
        font_size: 40.0,
        color: Color::WHITE,
    };

    // The animation API uses the `Name` component to target entities
    let ui = Name::new("planet");

    // Creating the animation
    let mut animation = AnimationClip::default();
    // A curve can modify a single part of a transform, here the translation
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![ui.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 3., 6.],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(20.0, 12.0, 0.0),
                Vec3::new(20.0, 15.0, 0.0),
                Vec3::new(20.0, 12.0, 0.0),
            ]),
        },
    );

    // Create the animation player, and set it to repeat
    let mut player = AnimationPlayer::default();
    player.play(animations.add(animation)).repeat();

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("craftpix/objects/Fishbarrel2.png"),
            transform: Transform {
                translation: Vec3::new(-window_width + 150., 17., -9.),
                scale: Vec3::splat(3.),
                ..default()
            },
            ..default()
        })
        // Fish Storage
        .with_children(|parent| {
            parent.spawn((
                Text2dBundle {
                    text: Text::from_sections([
                        TextSection::from_style(ui_text_style.clone()),
                        TextSection::from_style(ui_text_style.clone()),
                    ])
                    .with_alignment(TextAlignment::Center),
                    transform: Transform {
                        translation: Default::default(),
                        rotation: Default::default(),
                        scale: Vec3::splat(0.25),
                    },
                    ..default()
                },
                ui,
                player,
                PortUI,
            ));
        });
}

fn check_for_port_collisions(
    mut ev_deposit: EventWriter<DepositFishEvent>,
    mut ev_upgrade: EventWriter<UpgradeEvent>,
    mut ev_port_collison: EventReader<PortCollisionEvent>,
    mut port_fish: ResMut<PortStorage>,
    mut player_fish: ResMut<PlayerFishStored>,
    mut player_query: Query<(&mut FishStorage, &mut RodVariant), With<Player>>,
) {
    if player_fish.fish.is_empty() {
        return;
    }

    for _ in ev_port_collison.read() {
        for fish in &player_fish.fish {
            if let Some(count) = port_fish.fish.get_mut(&fish.0) {
                *count += 1;
            }
        }

        player_fish.fish.clear();

        let (mut player_storage, mut rod_variant) = player_query.single_mut();
        port_fish.weight += player_storage.current;

        player_storage.current = 0.;

        // Trigger Upgrades
        let (new_max, next_upgrade) = match port_fish.weight {
            w if w > 1200. => {
                if *rod_variant != RodVariant::CarbonCaster9000 {
                    ev_upgrade.send_default();
                }
                *rod_variant = RodVariant::CarbonCaster9000;
                (Some(1000.), Some(9999.))
            }
            w if w > 200. => {
                if *rod_variant != RodVariant::GraphiteGuardian {
                    ev_upgrade.send_default();
                }
                *rod_variant = RodVariant::GraphiteGuardian;
                (Some(200.), Some(1000.))
            }
            w if w > 150. => {
                if *rod_variant != RodVariant::FiberFusion {
                    ev_upgrade.send_default();
                }
                *rod_variant = RodVariant::FiberFusion;
                (Some(100.), Some(200.))
            }
            w if w > 50. => {
                if *rod_variant != RodVariant::BambooBlisscaster {
                    ev_upgrade.send_default();
                }
                *rod_variant = RodVariant::BambooBlisscaster;
                (Some(70.), Some(150.))
            }
            w if w > 15. => {
                if *rod_variant != RodVariant::WillowWhiskerWeaver {
                    ev_upgrade.send_default();
                }
                *rod_variant = RodVariant::WillowWhiskerWeaver;
                (Some(30.), Some(50.))
            }
            w if w > 8. => {
                if *rod_variant != RodVariant::ReedReelRig {
                    ev_upgrade.send_default();
                }
                *rod_variant = RodVariant::ReedReelRig;
                (Some(10.), Some(15.))
            }
            w if w > 4. => {
                if *rod_variant != RodVariant::TwigAndTwineTackler {
                    ev_upgrade.send_default();
                }
                *rod_variant = RodVariant::TwigAndTwineTackler;
                (Some(6.), Some(8.))
            }
            _ => (None, None),
        };

        ev_deposit.send(DepositFishEvent {
            port_weight: port_fish.weight,
            new_max,
            next_upgrade,
        });

        FishStorage::update_storage(0., new_max, &mut player_storage);
    }
}

fn setup_port_ui(mut port_ui_query: Query<&mut Text, With<PortUI>>) {
    let mut stui = port_ui_query.single_mut();

    stui.sections[0].value = "0.00 kg /".into();
    stui.sections[1].value = " 4 kg".into();
}

fn update_port_ui(
    mut ev_deposit: EventReader<DepositFishEvent>,
    mut port_ui_query: Query<&mut Text, With<PortUI>>,
) {
    for event in ev_deposit.read() {
        let current = event.port_weight;
        let mut stui = port_ui_query.single_mut();

        stui.sections[0].value = format!("{:.2} kg /", current);

        if let Some(next_upgrade) = event.next_upgrade {
            stui.sections[1].value = format!(" {:?} kg", next_upgrade);
        }
    }
}
