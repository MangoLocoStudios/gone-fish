use crate::{
    components::FishStorage,
    events::PortCollisionEvent,
    player::Player,
    resources::{PlayerFishStored, PortStorage},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Port;

pub struct PortPlugin;

impl Plugin for PortPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PortStorage>()
            .add_event::<PortCollisionEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, check_for_port_collisions);
    }
}

fn setup(mut commands: Commands, window: Query<&mut Window>) {
    let window = window.single();
    // From center of screen.
    let window_width = window.resolution.width() / 2.;

    commands.spawn((
        Port,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-window_width + 100., 50.0, 0.0),
                scale: Vec3::new(200., 200., 0.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn check_for_port_collisions(
    mut event: EventReader<PortCollisionEvent>,
    mut port_fish: ResMut<PortStorage>,
    mut player_fish: ResMut<PlayerFishStored>,
    mut player_query: Query<&mut FishStorage, With<Player>>,
) {
    if player_fish.fish.is_empty() {
        return;
    }

    for _ in event.read() {
        for fish in &player_fish.fish {
            if let Some(count) = port_fish.fish.get_mut(&fish.0) {
                *count += 1;
            }
        }

        player_fish.fish.clear();

        let mut player_storage = player_query.single_mut();
        port_fish.weight += player_storage.current;

        FishStorage::update_storage(0., None, &mut player_storage);

        // Trigger Upgrades
        // 10kg of fish - Rod upgrade
    }
}
