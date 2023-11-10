use bevy::prelude::*;
use rand;

// TODO: Make the plugin work innit
pub struct FishPlugin {}

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fish);
        app.add_systems(Update, update_fish);
    }
}

pub fn setup_fish(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..5 {
        let vertical_position = rand::random::<f32>() * -400.0 + 20.0;
        let horizontal_position = rand::random::<f32>() * -400.0 + 20.0;
        let going_left = rand::random::<bool>();

        let mut fishy = SpriteBundle {
            texture: asset_server.load("fish4.png"),
            transform: Transform::from_translation(Vec3::new(
                horizontal_position,
                vertical_position,
                0.0,
            ))
            .with_scale(Vec3::new(0.8, 0.8, 0.8)),
            ..default()
        };
        fishy.sprite.flip_x = going_left;
        commands.spawn(fishy);
    }
}

pub fn update_fish(mut all_fish: Query<(&mut Sprite, &mut Transform)>, time: Res<Time>) {
    for (mut fish, mut transform) in all_fish.iter_mut() {
        // Move the thing
        let going_left = !fish.flip_x;
        if going_left {
            transform.translation.x -= 1.0 * time.delta_seconds() * 500.0;
        } else {
            transform.translation.x += 1.0 * time.delta_seconds() * 500.0;
        }

        let stutter = rand::random::<f32>() * 200.0 * time.delta_seconds();
        transform.translation.x += stutter;

        // Flip the thing when at edge
        let delta_abs = 450.0;
        if transform.translation.x < -delta_abs {
            fish.flip_x = true;
        } else if transform.translation.x > delta_abs {
            fish.flip_x = false;
        }
    }
}
