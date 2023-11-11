use crate::{
    components::{Direction, Speed, Weight},
    events::{BoatCollisionEvent, FishCollisionEvent},
    rod::Rod,
};
use bevy::prelude::*;

// All things a fish can collide with
pub enum ThingsFishCanCollideWith {
    Boat,
    Rod,
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
    mut event: EventReader<FishCollisionEvent>,
    mut fish_query: Query<(Entity, &mut FishState), With<Fish>>,
) {
    for ev in event.read() {
        match ev.entity {
            ThingsFishCanCollideWith::Boat => commands.entity(ev.fish).despawn(),
            ThingsFishCanCollideWith::Rod => {
                for (fish, mut state) in &mut fish_query {
                    if fish == ev.fish {
                        *state = FishState::Caught
                    }
                }
            }
        }
    }
}
