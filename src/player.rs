use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 50.0, 0.0),
                scale: Vec3::new(100.0, 50.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
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

    if transform.translation.x - player_width > -window_width
        && keyboard_input.pressed(KeyCode::Left)
    {
        transform.translation.x -= 150. * time.delta_seconds();
    }

    if transform.translation.x + player_width < window_width
        && keyboard_input.pressed(KeyCode::Right)
    {
        transform.translation.x += 150. * time.delta_seconds();
    }
}
