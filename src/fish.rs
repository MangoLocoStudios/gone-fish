use crate::{
    direction::{Direction, Directions},
    speed::Speed,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Fish;

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_fish);
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

        commands.spawn((
            Fish,
            SpriteBundle {
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
            Speed { current: 200. },
            Direction { direction },
        ));
    }
}

pub fn move_fish(
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
