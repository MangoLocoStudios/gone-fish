use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{
    components::{Direction, Speed},
    events::TrashCollisionEvent,
    GameState::Game,
};

#[derive(Component, Clone, Copy, Debug)]
pub enum TrashVariant {
    Newspaper,
    OldShoe,
}

impl TrashVariant {
    pub fn image(self, asset_server: AssetServer) -> Handle<Image> {
        match self {
            TrashVariant::Newspaper => asset_server.load("craftpix/objects/Catch/Box.png"),
            TrashVariant::OldShoe => asset_server.load("craftpix/objects/Catch/Barrel.png"),
        }
    }
}

impl Distribution<TrashVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TrashVariant {
        match rng.gen_range(0..2) {
            0 => TrashVariant::Newspaper,
            _ => TrashVariant::OldShoe,
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

const TRASH_SPEED_MIN: f32 = 150.;
const TRASH_SPEED_MAX: f32 = 300.;

pub struct TrashPlugin;

impl Plugin for TrashPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TrashCollisionEvent>()
            .add_systems(OnEnter(Game), setup)
            .add_systems(Update, trash_movement.run_if(in_state(Game)));
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..10 {
        let vertical_position = rand::thread_rng().gen_range(50.0..400.);
        let horizontal_position = rand::thread_rng().gen_range(-1800.0..1800.);
        let trash: TrashVariant = rand::random();

        commands.spawn(TrashBundle {
            sprite: SpriteBundle {
                texture: trash.image(asset_server.clone()),
                transform: Transform {
                    translation: Vec3::new(horizontal_position, -vertical_position, 5.0),
                    scale: Vec3::splat(3.),
                    rotation: Quat::from_rotation_z(rand::thread_rng().gen_range(0.0..360.)),
                },
                ..default()
            },
            speed: Speed {
                current: rand::thread_rng().gen_range(TRASH_SPEED_MIN..TRASH_SPEED_MAX),
            },
            direction: Direction::random_y(),
            variant: trash,
            ..default()
        });
    }
}

pub fn trash_movement(
    time: Res<Time>,
    mut trash_query: Query<(&mut Transform, &mut Direction, &Speed), With<Trash>>,
) {
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
        if transform.translation.x < -1800. {
            *direction = Direction::Right;
        } else if transform.translation.x > 1800. {
            *direction = Direction::Left;
        }
    }
}
