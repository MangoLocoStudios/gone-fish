use crate::{
    components::{Direction, FishStorage, Speed, Weight},
    events::{BoatCollisionEvent, FishCollisionEvent, TrashCollisionEvent},
    player::Player,
    resources::FishStored,
    rod::Rod,
};
use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

pub enum ThingsFishCanCollideWith {
    Boat,
    Rod,
}

#[derive(Component, Clone, Copy, Debug)]
pub enum FishVariant {
    Trout,
    Tuna,
    Salmon,
}

impl Distribution<FishVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FishVariant {
        match rng.gen_range(0..=2) {
            0 => FishVariant::Tuna,
            1 => FishVariant::Trout,
            _ => FishVariant::Salmon,
        }
    }
}

#[derive(Component)]
pub enum FishState {
    Swimming,
    Caught,
}

#[derive(Component)]
pub struct Fish;

#[derive(Bundle)]
struct FishBundle {
    marker: Fish,
    direction: Direction,
    speed: Speed,
    weight: Weight,
    state: FishState,
    variant: FishVariant,
    // This might change to a SpriteSheetBundle eventually.
    sprite: SpriteBundle,
}

impl Default for FishBundle {
    fn default() -> Self {
        FishBundle {
            marker: Fish,
            direction: Direction::Left,
            speed: Speed { current: 200. },
            weight: Weight { current: 0.1 },
            state: FishState::Swimming,
            variant: FishVariant::Tuna,
            sprite: Default::default(),
        }
    }
}

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoatCollisionEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, (fish_movement, check_for_collisions));
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&mut Window>) {
    // From center of screen.
    let window = window.single();
    let window_width = window.resolution.width() / 2.;

    for _ in 0..5 {
        let vertical_position = rand::random::<f32>() * -400. + 20.;
        let horizontal_position = rand::random::<f32>() * window_width + 20.;
        let direction = Direction::random_y();

        commands.spawn(FishBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("fish4.png"),
                transform: Transform {
                    translation: Vec3::new(horizontal_position, vertical_position, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.5),
                    ..default()
                },
                sprite: Sprite {
                    // Sprite is facing left, we must flip it if it is going
                    // right.
                    flip_x: matches!(direction, Direction::Right),
                    ..default()
                },
                ..default()
            },
            direction,
            speed: Speed {
                current: rand::thread_rng().gen_range(150.0..300.0),
            },
            variant: rand::random(),
            weight: Weight {
                // Round weight to .2 decimal places
                current: (rand::thread_rng().gen_range(0.0..3.0) * 100.0_f32).round() / 100.0,
            },
            ..default()
        });
    }
}

pub fn fish_movement(
    time: Res<Time>,
    window: Query<&mut Window>,
    rod_query: Query<&Transform, (With<Rod>, Without<Fish>)>,
    mut fish_query: Query<
        (
            &mut Sprite,
            &mut Transform,
            &mut Direction,
            &Speed,
            &FishState,
        ),
        With<Fish>,
    >,
) {
    // From center of screen.
    let window = window.single();
    let window_width = window.resolution.width() / 2.;

    for (mut fish, mut transform, mut direction, speed, state) in &mut fish_query {
        match state {
            FishState::Swimming => {
                // Move the thing
                match *direction {
                    Direction::Left => {
                        transform.translation.x -= 1.0 * time.delta_seconds() * speed.current
                    }
                    Direction::Right => {
                        transform.translation.x += 1.0 * time.delta_seconds() * speed.current
                    }
                    _ => {}
                }

                // Flip the thing when at edge
                if transform.translation.x < -window_width {
                    *direction = Direction::Right;
                    fish.flip_x = true;
                } else if transform.translation.x > window_width {
                    *direction = Direction::Left;
                    fish.flip_x = false;
                }
            }
            FishState::Caught => {
                if let Ok(rod) = rod_query.get_single() {
                    transform.translation.x = rod.translation.x;
                    transform.translation.y = rod.translation.y;
                }
            }
        }
    }
}

pub fn check_for_collisions(
    mut commands: Commands,
    mut fish_collision_event: EventReader<FishCollisionEvent>,
    mut trash_collision_event: EventReader<TrashCollisionEvent>,
    mut player_query: Query<&mut FishStorage, With<Player>>,
    mut fish_stored: ResMut<FishStored>,
    mut fish_query: Query<(Entity, &mut FishState, &FishVariant, &Weight), With<Fish>>,
) {
    let mut fish_storage = player_query.single_mut();

    // Check for collision with rod / boat
    for ev in fish_collision_event.read() {
        for (fish, mut state, fish_variant, weight) in &mut fish_query {
            if fish != ev.fish {
                continue;
            }

            match ev.entity {
                ThingsFishCanCollideWith::Boat => {
                    if (weight.current + fish_storage.current) > fish_storage.max {
                        *state = FishState::Swimming;

                        println!(
                            "[DEBUG] Fish {:?} was too heavy, weighing at {} - current weight {} - max weight {}",
                            (fish_variant, weight), weight.current, fish_storage.current, fish_storage.max
                        );
                    } else {
                        fish_storage.current += weight.current;
                        fish_stored.fish.push((*fish_variant, *weight));
                        commands.entity(ev.fish).despawn();

                        println!(
                            "[DEBUG] Fish caught {:?} - current weight {} - max weight {}",
                            fish_stored.fish, fish_storage.current, fish_storage.max
                        );
                    }
                }
                ThingsFishCanCollideWith::Rod => *state = FishState::Caught,
            }
        }
    }

    // Check for trash collision
    for _ in trash_collision_event.read() {
        for (_, mut state, _, _) in &mut fish_query {
            match *state {
                FishState::Swimming => {}
                FishState::Caught => *state = FishState::Swimming,
            }
        }
    }
}
