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
    mut sprite_position: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 150. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 150. * time.delta_seconds();
        }
    }
}
