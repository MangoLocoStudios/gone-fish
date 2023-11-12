use crate::{
    components::{Direction, FishStorage, Invincibility, Speed, Weight},
    events::{BoatCollisionEvent, FishCollisionWithRodEvent, TrashCollisionEvent},
    player::Player,
    resources::{AliveFish, PlayerFishStored},
    rod::Rod,
};
use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::slice::Iter;

pub enum ThingsFishCanCollideWith {
    Boat,
    Rod,
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FishVariant {
    Trout,
    Tuna,
    Salmon,
}

impl FishVariant {
    pub fn iterator() -> Iter<'static, FishVariant> {
        static FISH_VARIANTS: [FishVariant; 3] =
            [FishVariant::Trout, FishVariant::Tuna, FishVariant::Salmon];
        FISH_VARIANTS.iter()
    }
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

const FISH_INVINCIBILITY_TIME: f32 = 1.;
const FISH_SPEED_MIN: f32 = 150.;
const FISH_SPEED_MAX: f32 = 300.;
const FISH_WEIGHT_MIN: f32 = 0.1;
const FISH_WEIGHT_MAX: f32 = 3.;

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FishCollisionWithRodEvent>()
            .init_resource::<AliveFish>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    fish_movement,
                    check_for_rod_collisions,
                    check_for_trash_collisions,
                    check_for_boat_collisions,
                    handle_invincibilities,
                ),
            );
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
                current: rand::thread_rng().gen_range(FISH_SPEED_MIN..FISH_SPEED_MAX),
            },
            variant: rand::random(),
            weight: Weight {
                // Round weight to .2 decimal places
                current: (rand::thread_rng().gen_range(FISH_WEIGHT_MIN..FISH_WEIGHT_MAX)
                    * 100.0_f32)
                    .round()
                    / 100.0,
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

pub fn check_for_boat_collisions(
    mut commands: Commands,
    mut boat_collision_event: EventReader<BoatCollisionEvent>,
    mut player_query: Query<&mut FishStorage, With<Player>>,
    mut fish_stored: ResMut<PlayerFishStored>,
    mut fish_query: Query<(Entity, &mut FishState, &FishVariant, &Weight), With<Fish>>,
) {
    let mut fish_storage = player_query.single_mut();

    for _ in boat_collision_event.read() {
        for (fish, mut state, fish_variant, weight) in &mut fish_query {
            match *state {
                FishState::Swimming => {}
                FishState::Caught => {
                    if (weight.current + fish_storage.current) > fish_storage.max {
                        *state = FishState::Swimming;

                        println!(
                            "[DEBUG] Fish {:?} was too heavy, weighing at {} - current weight {} - max weight {}",
                            (fish_variant, weight), weight.current, fish_storage.current, fish_storage.max
                        );
                    } else {
                        fish_storage.current += weight.current;
                        fish_stored.fish.push((*fish_variant, *weight));
                        commands.entity(fish).despawn();

                        println!(
                            "[DEBUG] Fish caught {:?} - current weight {} - max weight {}",
                            fish_stored.fish, fish_storage.current, fish_storage.max
                        );
                    }
                }
            }
        }
    }
}

pub fn check_for_rod_collisions(
    mut fish_collision_with_rod_event: EventReader<FishCollisionWithRodEvent>,
    mut fish_query: Query<(Entity, &mut FishState), (With<Fish>, Without<Invincibility>)>,
) {
    for ev in fish_collision_with_rod_event.read() {
        for (fish, mut state) in &mut fish_query {
            if fish != ev.fish {
                continue;
            }

            *state = FishState::Caught
        }
    }
}

pub fn check_for_trash_collisions(
    mut commands: Commands,
    mut trash_collision_event: EventReader<TrashCollisionEvent>,
    mut fish_query: Query<(Entity, &mut FishState), With<Fish>>,
) {
    for _ in trash_collision_event.read() {
        for (fish, mut state) in &mut fish_query {
            match *state {
                FishState::Swimming => {}
                FishState::Caught => {
                    commands.entity(fish).insert(Invincibility {
                        invincibility_timer: Timer::from_seconds(
                            FISH_INVINCIBILITY_TIME,
                            TimerMode::Once,
                        ),
                    });
                    *state = FishState::Swimming
                }
            }
        }
    }
}

fn handle_invincibilities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Invincibility)>,
    timer: Res<Time>,
) {
    for (e, mut i) in &mut query {
        if i.invincibility_timer.tick(timer.delta()).finished() {
            commands.entity(e).remove::<Invincibility>();
        }
    }
}

pub fn update_fish_count(
    fish_query: Query<&Sprite, With<Fish>>,
    mut alive_fish: ResMut<AliveFish>,
) {
    let fish_found = fish_query.iter().count() as u32;

    if fish_found == alive_fish.count {
        return;
    }

    alive_fish.count = fish_found;
    println!("[DEBUG] Current fish count: {}", alive_fish.count);
}
