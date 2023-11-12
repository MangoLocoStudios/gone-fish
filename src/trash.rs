use crate::components::{Direction, Speed};
use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Component, Clone, Copy, Debug)]
pub enum TrashVariant {
    Newspaper,
    OldShoe,
}

impl Distribution<TrashVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TrashVariant {
        match rng.gen_range(0..1) {
            0 => TrashVariant::OldShoe,
            _ => TrashVariant::Newspaper,
        }
    }
}

#[derive(Component)]
pub struct Trash;

#[derive(Bundle)]
struct TrashBundle {
    marker: Trash,
    direction: Direction,
    speed: Speed,
    variant: TrashVariant,
    // This might change to a SpriteSheetBundle eventually.
    sprite: SpriteBundle,
}

impl Default for TrashBundle {
    fn default() -> Self {
        TrashBundle {
            marker: Trash,
            direction: Direction::Left,
            speed: Speed { current: 100. },
            variant: TrashVariant::Newspaper,
            sprite: Default::default(),
        }
    }
}

pub struct TrashPlugin;

impl Plugin for TrashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, trash_movement);
    }
}

pub fn setup(mut commands: Commands, window: Query<&mut Window>) {
    // From center of screen.
    let window = window.single();
    let window_width = window.resolution.width() / 2.;

    for _ in 0..5 {
        let vertical_position = rand::random::<f32>() * -400. + 20.;
        let horizontal_position = rand::random::<f32>() * window_width + 20.;

        commands.spawn(TrashBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.75, 0.25, 0.25),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(horizontal_position, vertical_position, 0.0),
                    scale: Vec3::new(40., 40., 40.),
                    ..default()
                },
                ..default()
            },
            speed: Speed {
                current: rand::thread_rng().gen_range(100.0..200.0),
            },
            direction: Direction::random_y(),
            variant: rand::random(),
            ..default()
        });
    }
}

pub fn trash_movement(
    time: Res<Time>,
    window: Query<&mut Window>,
    mut trash_query: Query<(&mut Transform, &mut Direction, &Speed), With<Trash>>,
) {
    // From center of screen.
    let window = window.single();
    let window_width = window.resolution.width() / 2.;

    for (mut transform, mut direction, speed) in &mut trash_query {
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
        } else if transform.translation.x > window_width {
            *direction = Direction::Left;
        }
    }
}
