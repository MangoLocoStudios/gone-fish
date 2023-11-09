use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
    }
}

fn player_movement(
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

    if (transform.translation.x - player_width) + 1. > -window_width {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 150. * time.delta_seconds();
        }
    }

    if (transform.translation.x + player_width) + 1. < window_width {
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 150. * time.delta_seconds();
        }
    }
}
