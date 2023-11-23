use crate::components::CameraShake;
use crate::events::DepositFishEvent;
use crate::{
    components::FishStorage,
    events::PortCollisionEvent,
    player::Player,
    resources::{PlayerFishStored, PortStorage, RodProperties},
    GameState::Game,
};
use bevy::prelude::*;
use std::option::Option;

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

    commands.spawn((SpriteBundle {
        texture: asset_server.load("craftpix/objects/Fishbarrel2.png"),
        transform: Transform {
            translation: Vec3::new(-window_width + 150., 17., -9.),
            scale: Vec3::splat(3.),
            ..default()
        },
        ..default()
    },));
}

fn check_for_deposit_events(
    mut commands: Commands,
    mut ev_deposit: EventReader<DepositFishEvent>,
    mut camera_query: Query<(Entity, &mut Transform), With<Camera2d>>,
) {
    for _ in ev_deposit.read() {
        println!("[DEBUG] Deposit Event Started");

        let (camera_entity, mut camera_transform) = camera_query.single();

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
    mut rod_props: ResMut<RodProperties>,
    mut player_fish: ResMut<PlayerFishStored>,
    mut player_query: Query<&mut FishStorage, With<Player>>,
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

        let mut player_storage = player_query.single_mut();
        port_fish.weight += player_storage.current;

        player_storage.current = 0.;
        let mut new_max: Option<f32> = None;

        // Trigger Upgrades
        if port_fish.weight > 2. && !port_fish.weight_two {
            rod_props.length += 200.;
            rod_props.pull += 30.;
            new_max = Some(6.);
            port_fish.weight_two = true;
        }

        if port_fish.weight > 5. && !port_fish.weight_five {
            rod_props.pull += 50.;
            new_max = Some(10.);
            port_fish.weight_five = true;
        }

        FishStorage::update_storage(0., new_max, &mut player_storage);
    }
}
