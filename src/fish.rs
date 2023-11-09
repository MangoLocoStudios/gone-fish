use bevy::prelude::*;
use rand;

pub fn setup_fish(mut commands: Commands, asset_server: Res<AssetServer>) {
     for _ in 0..10 {
         let vertical_position = rand::random::<f32>() * -400.0 + 20.0;
         let horizontal_position = rand::random::<f32>() * -400.0 + 20.0;
         let going_left = rand::random::<bool>();

         let mut fishy = SpriteBundle {
             texture: asset_server.load("fish4.png"),
             transform: Transform::from_translation(Vec3::new(horizontal_position, vertical_position, 0.0))
                 .with_scale(Vec3::new(1.0, 1.0, 1.0)),
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

        println!("x: {}", transform.translation.x);

        // Flip the thing when at edge
        let delta_abs = 450.0;
        if transform.translation.x < -delta_abs  {
            fish.flip_x = true;
        } else if transform.translation.x > delta_abs {
            fish.flip_x = false;
        }
    }
}