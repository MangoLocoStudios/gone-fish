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
    mut player_position: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player_position.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        transform.translation.x -= 150. * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Right) {
        transform.translation.x += 150. * time.delta_seconds();
    }
}
