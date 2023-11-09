use bevy::prelude::*;

#[derive(Component)]
pub struct Rod;

pub struct RodPlugin;

impl Plugin for RodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (cast_rod, rod_movement));
    }
}

fn cast_rod(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            Rod,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.75, 0.25),
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
        ));
    }
}

fn rod_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<&mut Transform, With<Rod>>,
) {
    for mut transform in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 150. * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 150. * time.delta_seconds();
        }
    }
}
