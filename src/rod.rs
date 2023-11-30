use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::components::{CameraShake, Weight};
use crate::{
    components::{Acceleration, Velocity},
    events::{BoatCollisionEvent, FishCollisionWithRodEvent, TrashCollisionEvent},
    fish::Fish,
    player::{Boat, Player},
    trash::Trash,
    GameState::Game,
};

#[derive(Default)]
pub struct RodProperties {
    pub length: f32,
    pub pull: f32,
}

#[derive(Component, Clone, Copy)]
pub enum RodVariant {
    StickWithString,
    TwigAndTwineTackler,
    ReedReelRig,
    WillowWhiskerWeaver,
    BambooBlisscaster,
    FiberFusion,
    GraphiteGuardian,
    CarbonCaster9000,
}

impl RodVariant {
    pub fn get_rod_properties(self) -> RodProperties {
        let (length, pull) = match self {
            RodVariant::StickWithString => (200., 100.),
            RodVariant::TwigAndTwineTackler => (335., 105.),
            RodVariant::ReedReelRig => (450., 115.),
            RodVariant::WillowWhiskerWeaver => (650., 118.),
            RodVariant::BambooBlisscaster => (1000., 123.),
            RodVariant::FiberFusion => (1300., 127.),
            RodVariant::GraphiteGuardian => (1800., 130.),
            RodVariant::CarbonCaster9000 => (2400., 135.),
        };

        RodProperties { length, pull }
    }
}

#[derive(Component, Debug)]
enum RodState {
    Idle,
    Reeling,
}

#[derive(Component)]
pub struct Rod;

pub struct RodPlugin;

#[derive(Component)]
struct LineToPlayer;

#[derive(Component)]
struct Line;

impl Plugin for RodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoatCollisionEvent>().add_systems(
            Update,
            (
                cast_rod,
                rod_movement,
                check_for_boat_collisions,
                check_for_fish_collisions,
                check_for_trash_collisions,
                update_line,
            )
                .run_if(in_state(Game)),
        );
    }
}

const ROD_MOVEMENT_DOWN: f32 = 75.0;

fn cast_rod(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    rod: Query<&Rod>,
    player_position: Query<&Transform, With<Player>>,
) {
    let player = player_position.single();

    // Only spawn a new rod if there isn't already one spawned
    if rod.get_single().is_ok() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        commands.spawn((
            Rod,
            RodState::Idle,
            SpriteBundle {
                texture: asset_server.load("fish_hook.png"),
                transform: Transform {
                    translation: Vec3::new(player.translation.x, -50., 10.),
                    scale: Vec3::splat(1.5),
                    ..default()
                },
                ..default()
            },
            Velocity(Vec3::new(0., -ROD_MOVEMENT_DOWN, 0.)),
            Acceleration(Vec3::splat(0.)),
            LineToPlayer,
        ));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(2.0, 0.0)), // Thin and initially of zero length
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(player.translation.x, 0., 10.),
                    ..default()
                },
                ..default()
            },
            Line,
        ));
    }
}

fn update_line(
    mut line_query: Query<(&mut Transform, &mut Sprite), With<Line>>,
    rod_query: Query<&Transform, (With<Rod>, Without<Player>, Without<Line>)>,
    player_query: Query<&Transform, (With<Player>, Without<Rod>, Without<Line>)>,
) {
    if let (Ok((mut line_transform, mut line_sprite)), Ok(rod_transform), Ok(player_transform)) = (
        line_query.get_single_mut(),
        rod_query.get_single(),
        player_query.get_single(),
    ) {
        // Reset and show the line when rod is cast again
        let midpoint = (player_transform.translation + rod_transform.translation) / 2.0;
        let length = player_transform
            .translation
            .distance(rod_transform.translation);

        line_transform.translation = midpoint;
        line_sprite.custom_size = Some(Vec2::new(1.0, length));

        let direction = rod_transform.translation - player_transform.translation;
        let angle = direction.y.atan2(direction.x);
        line_transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
    }
}

fn check_for_boat_collisions(
    mut commands: Commands,
    rod_query: Query<(Entity, &Transform), (With<Rod>, Without<Player>)>,
    line_query: Query<Entity, With<Line>>,
    player_query: Query<&Transform, With<Player>>,
    boat_query: Query<(&Transform, &Handle<Image>), With<Boat>>,
    mut boat_collision_event: EventWriter<BoatCollisionEvent>,
    assets: Res<Assets<Image>>,
) {
    let player = player_query.single();
    let (boat, image) = boat_query.single();

    let (rod_entity, rod) = match rod_query.get_single() {
        Ok((rod_entity, rod)) => (rod_entity, rod),
        Err(_) => return,
    };

    let line_entity = match line_query.get_single() {
        Ok(line_query) => line_query,
        Err(_) => return,
    };

    if collide(
        // Child (boat) position is relative to parent (player) so we must
        // combine their translations to get the boats world translation
        player.translation + boat.translation,
        assets
            .get(image)
            .expect("boat to always have an available image")
            .size()
            .as_vec2()
            * boat.scale.truncate(),
        rod.translation,
        rod.scale.truncate(),
    )
    .is_none()
    {
        return;
    }

    boat_collision_event.send_default();

    // Despawn rod when it's reeled back in
    commands.entity(rod_entity).despawn();
    commands.entity(line_entity).despawn();
}

fn check_for_fish_collisions(
    assets: Res<Assets<Image>>,
    mut commands: Commands,
    fish_query: Query<(Entity, &Transform, &Weight), With<Fish>>,
    mut collision_events: EventWriter<FishCollisionWithRodEvent>,
    mut rod_query: Query<(&Transform, &mut RodState, &Handle<Image>), With<Rod>>,
    camera_query: Query<(Entity, &Transform), (With<Camera2d>, Without<Fish>)>,
) {
    let (rod, mut state, image) = match rod_query.get_single_mut() {
        Ok((rod, state, image)) => (rod, state, image),
        Err(_) => return,
    };

    for (fish, fish_transform, fish_weight) in &fish_query {
        if collide(
            fish_transform.translation,
            fish_transform.scale.truncate(),
            rod.translation,
            assets
                .get(image)
                .expect("boat to always have an available image")
                .size()
                .as_vec2()
                * rod.scale.truncate(),
        )
        .is_none()
        {
            continue;
        }

        match *state {
            RodState::Idle => {
                collision_events.send(FishCollisionWithRodEvent { fish });
                *state = RodState::Reeling;

                let (camera_entity, camera_transform) = camera_query.single();

                commands.entity(camera_entity).insert(CameraShake {
                    shake_timer: Timer::from_seconds(0.1, TimerMode::Once),
                    intensity: 0.25 * fish_weight.current,
                    start_translation: camera_transform.translation,
                });
            }
            RodState::Reeling => {}
        }
    }
}

fn check_for_trash_collisions(
    assets: Res<Assets<Image>>,
    mut rod_query: Query<(&Transform, &mut RodState, &Handle<Image>), With<Rod>>,
    trash_query: Query<&Transform, With<Trash>>,
    mut collision_events: EventWriter<TrashCollisionEvent>,
) {
    let (rod, mut state, image) = match rod_query.get_single_mut() {
        Ok((rod, state, image)) => (rod, state, image),
        Err(_) => return,
    };

    for trash_transform in &trash_query {
        if collide(
            trash_transform.translation,
            trash_transform.scale.truncate(),
            rod.translation,
            assets
                .get(image)
                .expect("boat to always have an available image")
                .size()
                .as_vec2()
                * rod.scale.truncate(),
        )
        .is_none()
        {
            continue;
        }

        match *state {
            RodState::Idle => {}
            RodState::Reeling => {
                collision_events.send_default();
                *state = RodState::Idle;
            }
        }
    }
}

fn rod_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut rod_query: Query<
        (&mut Transform, &mut Acceleration, &mut Velocity),
        (With<Rod>, Without<Player>),
    >,
    player_query: Query<(&Transform, &RodVariant), With<Player>>,
) {
    let (player, rod_stats) = player_query.single();
    let (mut transform, mut acceleration, mut velocity) = match rod_query.get_single_mut() {
        Ok((transform, acceleration, velocity)) => (transform, acceleration, velocity),
        Err(_) => return,
    };

    let rod_stats = rod_stats.get_rod_properties();

    // Keep rod x aligned with player
    transform.translation.x = player.translation.x;

    // Move rod
    if keyboard_input.just_pressed(KeyCode::Space) && acceleration.0.y < 150. {
        acceleration.0.y += rod_stats.pull * 1.5;
    }

    let calculated_velocity = velocity.0 + acceleration.0;
    transform.translation += calculated_velocity * time.delta_seconds();

    // Decay acceleration
    if acceleration.0.y > 0. {
        acceleration.0.y -= 3.;
    } else {
        acceleration.0.y = 0.;
    }

    // Ensure rod doesn't move further than its length
    if transform.translation.y < -rod_stats.length {
        velocity.0 = Vec3::splat(0.);
    } else {
        velocity.0 = Vec3::new(0., -ROD_MOVEMENT_DOWN, 0.);
    }
}
