use crate::components::CameraShake;
use crate::events::DepositFishEvent;
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

pub struct PortPlugin;

impl Plugin for PortPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PortStorage>()
            .add_event::<PortCollisionEvent>()
            .add_event::<DepositFishEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (check_for_port_collisions, check_for_deposit_events).run_if(in_state(Game)),
            );
    }
}

fn setup(mut commands: Commands, window: Query<&mut Window>, asset_server: Res<AssetServer>) {
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

    commands.spawn(SpriteBundle {
        texture: asset_server.load("craftpix/objects/Fishbarrel2.png"),
        transform: Transform {
            translation: Vec3::new(-window_width + 150., 17., -9.),
            scale: Vec3::splat(3.),
            ..default()
        },
        ..default()
    });
}

fn check_for_deposit_events(
    mut commands: Commands,
    mut ev_deposit: EventReader<DepositFishEvent>,
    mut camera_query: Query<(Entity, &Transform), With<Camera2d>>,
) {
    for _ in ev_deposit.read() {
        println!("[DEBUG] Deposit Event Started");

        let (camera_entity, camera_transform) = camera_query.single();

        commands.entity(camera_entity).insert(CameraShake {
            shake_timer: Timer::from_seconds(0.15, TimerMode::Once),
            intensity: 0.5,
            start_translation: camera_transform.translation.clone(),
        });
    }
}

fn check_for_port_collisions(
    mut ev_deposit: EventWriter<DepositFishEvent>,
    mut ev_port_collison: EventReader<PortCollisionEvent>,
    mut port_fish: ResMut<PortStorage>,
    mut player_fish: ResMut<PlayerFishStored>,
    mut player_query: Query<(&mut FishStorage, &mut RodVariant), With<Player>>,
) {
    if player_fish.fish.is_empty() {
        return;
    }

    for _ in ev_port_collison.read() {
        ev_deposit.send(DepositFishEvent);

        println!("[DEBUG] Depositing fish");
        for fish in &player_fish.fish {
            if let Some(count) = port_fish.fish.get_mut(&fish.0) {
                *count += 1;
            }
        }

        player_fish.fish.clear();

        let (mut player_storage, mut rod_variant) = player_query.single_mut();
        port_fish.weight += player_storage.current;

        player_storage.current = 0.;

        println!("[DEBUG] {}", port_fish.weight);

        // Trigger Upgrades
        let new_max = match port_fish.weight {
            w if w > 150. => {
                *rod_variant = RodVariant::CarbonCaster9000;
                Some(75.)
            }
            w if w > 100. => {
                *rod_variant = RodVariant::GraphiteGuardian;
                Some(55.)
            }
            w if w > 75. => {
                *rod_variant = RodVariant::FiberFusion;
                Some(30.)
            }
            w if w > 50. => {
                *rod_variant = RodVariant::BambooBlisscaster;
                Some(20.)
            }
            w if w > 25. => {
                *rod_variant = RodVariant::WillowWhiskerWeaver;
                Some(15.)
            }
            w if w > 10. => {
                *rod_variant = RodVariant::ReedReelRig;
                Some(10.)
            }
            w if w > 4. => {
                *rod_variant = RodVariant::TwigAndTwineTackler;
                Some(6.)
            }
            _ => None,
        };

        FishStorage::update_storage(0., new_max, &mut player_storage);
    }
}
