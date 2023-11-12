use crate::{
    components::FishStorage, events::PortCollisionEvent, port::Port, resources::PlayerFishStored,
};
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::de::Unexpected::Option;
use bevy::sprite::collide_aabb::{collide, Collision};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerFishStored>()
            .add_systems(Startup, setup)
            .add_systems(Update, (player_movement, check_for_port_collisions));
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Player,
        FishStorage {
            current: 0.,
            max: 3.,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 50.0, 0.0),
                scale: Vec3::new(100.0, 50.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn player_movement(
    mut event: EventReader<PortCollisionEvent>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    window: Query<&mut Window>,
    mut player_position: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player_position.single_mut();
    let window = window.single();

    // From center of screen.
    let window_width = window.resolution.width() / 2.;
    // From center of player.
    let player_width = transform.scale.truncate().x / 2.;

    if transform.translation.x - player_width > -window_width
        && keyboard_input.pressed(KeyCode::Left)
    {
        transform.translation.x -= 150. * time.delta_seconds();
    }

    if transform.translation.x + player_width < window_width
        && keyboard_input.pressed(KeyCode::Right)
    {
        transform.translation.x += 150. * time.delta_seconds();
    }
}

fn check_for_port_collisions(
    mut player_query: Query<&Transform, With<Player>>,
    mut port_query: Query<&Transform, With<Port>>,
    mut port_collision_event: EventWriter<PortCollisionEvent>,
) {
    let player = player_query.single_mut();
    let port = port_query.single_mut();

    if let Some(collision) = collide(
        player.translation,
        player.scale.truncate(),
        port.translation,
        port.scale.truncate(),
    )
    {
        port_collision_event.send(PortCollisionEvent {
            collision_direction: collision
        });
    }
}
