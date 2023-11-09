use bevy::prelude::*;

pub fn setup_fish(mut commands: Commands, asset_server: Res<AssetServer>, vertical_position: f32) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("fish3.png"),
        transform: Transform::from_translation(Vec3::new(0.0, vertical_position, 0.0))
            .with_scale(Vec3::new(1.0, 1.0, 1.0)),
        ..default()
    });
}

pub fn update_fishies(all_fish: &Query<SpriteBundle>) {
    for fish in all_fish.iter() {
        println!("fish: {:?}", fish);
    }
}