use crate::{
    components::{AnimationIndices, AnimationTimer, FishStorage},
    events::PortCollisionEvent,
    port::Port,
    resources::PlayerFishStored,
    rod::{Rod, RodVariant},
    GameState::Game,
};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(Component)]
pub struct Player;

#[derive(Component, PartialEq, Debug)]
pub enum PlayerState {
    Rowing,
    Fishing,
    Idle,
    Catching,
}

#[derive(Component)]
pub struct Boat;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerFishStored>()
            .add_systems(OnEnter(Game), setup)
            .add_systems(
                Update,
                (
                    player_movement,
                    check_for_port_collisions,
                    handle_animation_states,
                )
                    .run_if(in_state(Game)),
            );
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let boat = commands
        .spawn((
            Boat,
            SpriteBundle {
                texture: asset_server.load("craftpix/objects/Boat.png"),
                transform: Transform {
                    translation: Vec3::new(13., -10., -1.),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let player = commands
        .spawn({
            let texture_handle = asset_server.load("craftpix/fisherman/fisherman_spritesheet.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(48., 48.), 6, 4, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            let animation_indices = AnimationIndices {
                first: 18,
                last: 21,
            };

            (
                Player,
                PlayerState::Idle,
                RodVariant::StickWithString,
                FishStorage {
                    current: 0.,
                    max: 3.,
                },
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform {
                        translation: Vec3::new(0., 0., 5.),
                        scale: Vec3::splat(3.),
                        ..default()
                    },
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            )
        })
        .id();

    commands.entity(player).push_children(&[boat]);
}

fn handle_animation_states(
    mut player_query: Query<(&mut AnimationIndices, &mut PlayerState), With<Player>>,
    rod: Query<&Rod>,
) {
    let (mut animation_indicies, mut player_state) = player_query.single_mut();

    println!("[DEBUG] state {:?} ", player_state);

    if rod.get_single().is_ok() {
        *player_state = PlayerState::Fishing;
    }

    *animation_indicies = match *player_state {
        PlayerState::Rowing => AnimationIndices {
            first: 12,
            last: 15,
        },
        PlayerState::Fishing => AnimationIndices { first: 0, last: 3 },
        PlayerState::Idle => AnimationIndices {
            first: 18,
            last: 21,
        },
        PlayerState::Catching => AnimationIndices { first: 6, last: 11 },
    }
}

fn player_movement(
    _event: EventReader<PortCollisionEvent>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    window: Query<&mut Window>,
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let (mut transform, mut player_state) = player_query.single_mut();
    let window = window.single();
    let mut camera = camera_query.single_mut();

    // From center of screen.
    let window_width = window.resolution.width() / 2.;
    // From center of player.
    let player_width = transform.scale.truncate().x / 2.;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Right) {
        if transform.translation.x - player_width > -window_width
            && keyboard_input.pressed(KeyCode::Left)
        {
            *player_state = PlayerState::Rowing;
            transform.translation.x -= 150. * time.delta_seconds();
            camera.translation.x -= 150. * time.delta_seconds();
        }

        if transform.translation.x + player_width < window_width
            && keyboard_input.pressed(KeyCode::Right)
        {
            *player_state = PlayerState::Rowing;
            transform.translation.x += 150. * time.delta_seconds();
            camera.translation.x += 150. * time.delta_seconds();
        }
    } else {
        *player_state = PlayerState::Idle;
    }
}

fn check_for_port_collisions(
    mut player_query: Query<&Transform, With<Player>>,
    mut port_query: Query<(&Transform, &Handle<Image>), With<Port>>,
    mut port_collision_event: EventWriter<PortCollisionEvent>,
    assets: Res<Assets<Image>>,
) {
    let player = player_query.single_mut();
    let (port, image) = port_query.single_mut();

    if let Some(collision) = collide(
        player.translation,
        player.scale.truncate(),
        port.translation,
        assets
            .get(image)
            .expect("port to always have an available image")
            .size()
            .as_vec2()
            * port.scale.truncate(),
    ) {
        port_collision_event.send(PortCollisionEvent {
            collision_direction: collision,
        });
    }
}
