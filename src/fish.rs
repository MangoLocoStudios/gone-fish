use crate::components::{Direction, Speed, Weight};
use crate::directions::Directions;
use bevy::prelude::*;

#[derive(Component)]
pub struct Fish;

#[derive(Bundle)]
struct FishBundle {
    marker: Fish,
    direction: Direction,
    speed: Speed,
    weight: Weight,
    // This might change to a SpriteSheetBundle eventually.
    sprite: SpriteBundle,
}

impl Default for FishBundle {
    fn default() -> Self {
        FishBundle {
            marker: Fish,
            direction: Direction {
                direction: Directions::LEFT,
            },
            speed: Speed { current: 200. },
            weight: Weight { current: 0.1 },
            sprite: Default::default(),
        }
    }
}

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, fish_movement);
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&mut Window>) {
    // From center of screen.
    let window = window.single();
    let window_width = window.resolution.width() / 2.;

    for _ in 0..5 {
        let vertical_position = rand::random::<f32>() * -400. + 20.;
        let horizontal_position = rand::random::<f32>() * window_width + 20.;
        let direction = Directions::random();

        commands.spawn(FishBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("fish4.png"),
                transform: Transform {
                    translation: Vec3::new(horizontal_position, vertical_position, 0.0),
                    scale: Vec3::new(0.5, 0.5, 0.5),
                    ..default()
                },
                sprite: Sprite {
                    flip_x: !direction.going_left(),
                    ..default()
                },
                ..default()
            },
            direction: Direction { direction },
            ..default()
        });
    }
}

pub fn fish_movement(
    time: Res<Time>,
    window: Query<&mut Window>,
    mut fish_query: Query<(&mut Sprite, &mut Transform, &mut Direction, &Speed), With<Fish>>,
) {
    // From center of screen.
    let window = window.single();
    let window_width = window.resolution.width() / 2.;

    for (mut fish, mut transform, mut direction, speed) in &mut fish_query {
        // Move the thing
        match direction.direction {
            Directions::LEFT => {
                transform.translation.x -= 1.0 * time.delta_seconds() * speed.current
            }
            Directions::RIGHT => {
                transform.translation.x += 1.0 * time.delta_seconds() * speed.current
            }
        }

        // Flip the thing when at edge
        if transform.translation.x < -window_width {
            direction.direction = Directions::RIGHT;
            fish.flip_x = true;
        } else if transform.translation.x > window_width {
            direction.direction = Directions::LEFT;
            fish.flip_x = false;
        }
    }
}
