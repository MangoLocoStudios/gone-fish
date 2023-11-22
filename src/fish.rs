use crate::{
    components::{
        AnimationIndices, AnimationTimer, CanDie, DecayTimer, Direction, FishStorage,
        Invincibility, Speed, Weight,
    },
    events::{BoatCollisionEvent, FishCollisionWithRodEvent, TrashCollisionEvent},
    player::Player,
    resources::{AliveFish, PlayerFishStored},
    rod::Rod,
    GameState::Game,
};
use bevy::prelude::*;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::slice::Iter;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FishVariant {
    Trout,
    Tuna,
    Salmon,
}

impl FishVariant {
    pub fn iterator() -> Iter<'static, FishVariant> {
        static FISH_VARIANTS: [FishVariant; 3] =
            [FishVariant::Trout, FishVariant::Tuna, FishVariant::Salmon];
        FISH_VARIANTS.iter()
    }

    pub fn texture_atlas(self, asset_server: AssetServer) -> (TextureAtlas, AnimationIndices) {
        (
            match self {
                FishVariant::Trout => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/1.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(12., 6.), 2, 1, None, None)
                }
                FishVariant::Tuna => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/2.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(16., 12.), 2, 1, None, None)
                }
                FishVariant::Salmon => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/3.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(20., 12.), 2, 1, None, None)
                }
            },
            AnimationIndices { first: 0, last: 1 },
        )
    }

    pub fn get_spawn_depth_range(self) -> std::ops::Range<f32> {
        match self {
            FishVariant::Trout => 100.0..200.,
            FishVariant::Tuna => 300.0..500.,
            FishVariant::Salmon => 600.0..800.,
        }
    }

    pub fn get_weight_range(self) -> std::ops::Range<f32> {
        match self {
            FishVariant::Trout => 0.1..1.5,
            FishVariant::Tuna => 1.5..3.,
            FishVariant::Salmon => 3.0..4.,
        }
    }
}

impl Distribution<FishVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FishVariant {
        match rng.gen_range(0..=2) {
            0 => FishVariant::Tuna,
            1 => FishVariant::Trout,
            _ => FishVariant::Salmon,
        }
    }
}

#[derive(Component, Debug)]
pub enum FishState {
    Swimming,
    Caught,
}

#[derive(Component)]
pub struct Fish;

#[derive(Bundle)]
struct FishBundle {
    marker: Fish,
    direction: Direction,
    speed: Speed,
    weight: Weight,
    state: FishState,
    variant: FishVariant,
    sprite_sheet: SpriteSheetBundle,
    can_die: CanDie,
    decay_timer: DecayTimer,
}

impl Default for FishBundle {
    fn default() -> Self {
        FishBundle {
            marker: Fish,
            direction: Direction::Left,
            speed: Speed { current: 200. },
            weight: Weight { current: 0.1 },
            state: FishState::Swimming,
            variant: FishVariant::Tuna,
            can_die: CanDie { dying: false },
            sprite_sheet: Default::default(),
            decay_timer: Default::default(),
        }
    }
}

const FISH_INVINCIBILITY_TIME: f32 = 1.;
const FISH_SPEED_MIN: f32 = 150.;
const FISH_SPEED_MAX: f32 = 250.;

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FishCollisionWithRodEvent>()
            .init_resource::<AliveFish>()
            .add_systems(OnEnter(Game), setup)
            .add_systems(
                Update,
                (
                    update_fish_count,
                    spawn_fish,
                    fish_movement,
                    fish_boundary,
                    die_the_fish,
                    cull_fish,
                    check_for_rod_collisions,
                    check_for_trash_collisions,
                    check_for_boat_collisions,
                    handle_invincibilities,
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
    for _ in 0..20 {
        let fish: FishVariant = rand::random();
        let direction = Direction::random_y();

        let vertical_position = rand::thread_rng().gen_range(fish.get_spawn_depth_range());
        let horizontal_position = rand::thread_rng().gen_range(-1800.0..1800.);

        commands.spawn({
            let (texture_atlas, animation_indices) = fish.texture_atlas(asset_server.clone());
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            (
                FishBundle {
                    sprite_sheet: SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        sprite: TextureAtlasSprite {
                            index: animation_indices.first,
                            flip_x: matches!(direction, Direction::Left),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(horizontal_position, -vertical_position, 5.),
                            scale: Vec3::splat(3.),
                            ..default()
                        },
                        ..default()
                    },
                    direction,
                    speed: Speed {
                        current: rand::thread_rng().gen_range(FISH_SPEED_MIN..FISH_SPEED_MAX),
                    },
                    variant: fish,
                    weight: Weight {
                        // Round weight to .2 decimal places
                        current: (rand::thread_rng().gen_range(fish.get_weight_range())
                            * 100.0_f32)
                            .round()
                            / 100.0,
                    },
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            )
        });
    }
}

pub fn fish_movement(
    time: Res<Time>,
    rod_query: Query<&Transform, (With<Rod>, Without<Fish>)>,
    mut fish_query: Query<(&mut Transform, &mut Direction, &Speed, &FishState), With<Fish>>,
) {
    for (mut transform, direction, speed, state) in &mut fish_query {
        match state {
            FishState::Swimming => {
                // Handle fish that are too high up
                if transform.translation.y > -100. {
                    transform.translation.y -= 1.0 * time.delta_seconds() * speed.current;
                };
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
            }
            FishState::Caught => {
                if let Ok(rod) = rod_query.get_single() {
                    transform.translation.x = rod.translation.x;
                    transform.translation.y = rod.translation.y;
                }
            }
        }
    }
}

pub fn fish_boundary(
    mut fish_query: Query<
        (
            &mut TextureAtlasSprite,
            &mut Transform,
            &mut Direction,
            &CanDie,
        ),
        With<Fish>,
    >,
) {
    for (mut fish, transform, mut direction, can_die) in &mut fish_query {
        if can_die.dying {
            return;
        }
        // Flip the thing when at edge
        if transform.translation.x < -1800. {
            *direction = Direction::Right;
            fish.flip_x = false;
        } else if transform.translation.x > 1800. {
            *direction = Direction::Left;
            fish.flip_x = true;
        }
    }
}

pub fn die_the_fish(mut fishicide_query: Query<(&DecayTimer, &mut CanDie), With<Fish>>) {
    for (timer, mut can_die) in &mut fishicide_query {
        if timer.timer.finished() && !can_die.dying {
            can_die.dying = true;
        }
    }
}

pub fn cull_fish(
    mut commands: Commands,
    mut fish_query: Query<(Entity, &CanDie, &Transform), With<Fish>>,
) {
    for (fish, can_die, &transform) in &mut fish_query {
        let fish_out_of_bounds =
            transform.translation.x > 1800. || transform.translation.x < -1800.;
        if can_die.dying && fish_out_of_bounds {
            commands.entity(fish).despawn();
        }
    }
}

pub fn check_for_boat_collisions(
    mut commands: Commands,
    mut boat_collision_event: EventReader<BoatCollisionEvent>,
    mut player_query: Query<&mut FishStorage, With<Player>>,
    mut fish_stored: ResMut<PlayerFishStored>,
    mut fish_query: Query<(Entity, &mut FishState, &FishVariant, &Weight), With<Fish>>,
) {
    let mut fish_storage = player_query.single_mut();

    for _ in boat_collision_event.read() {
        for (fish, mut state, fish_variant, weight) in &mut fish_query {
            match *state {
                FishState::Swimming => {}
                FishState::Caught => {
                    if (weight.current + fish_storage.current) > fish_storage.max {
                        *state = FishState::Swimming;

                        println!(
                            "[DEBUG] Fish {:?} was too heavy, weighing at {} - current weight {} - max weight {}",
                            (fish_variant, weight), weight.current, fish_storage.current, fish_storage.max
                        );
                    } else {
                        fish_storage.current += weight.current;
                        fish_stored.fish.push((*fish_variant, *weight));
                        commands.entity(fish).despawn();

                        println!(
                            "[DEBUG] Fish caught {:?} - current weight {} - max weight {}",
                            fish_stored.fish, fish_storage.current, fish_storage.max
                        );
                    }
                }
            }
        }
    }
}

pub fn check_for_rod_collisions(
    mut fish_collision_with_rod_event: EventReader<FishCollisionWithRodEvent>,
    mut fish_query: Query<(Entity, &mut FishState), (With<Fish>, Without<Invincibility>)>,
) {
    for ev in fish_collision_with_rod_event.read() {
        for (fish, mut state) in &mut fish_query {
            if fish != ev.fish {
                continue;
            }

            *state = FishState::Caught
        }
    }
}

pub fn check_for_trash_collisions(
    mut commands: Commands,
    mut trash_collision_event: EventReader<TrashCollisionEvent>,
    mut fish_query: Query<(Entity, &mut FishState), With<Fish>>,
) {
    for _ in trash_collision_event.read() {
        for (fish, mut state) in &mut fish_query {
            match *state {
                FishState::Swimming => {}
                FishState::Caught => {
                    commands.entity(fish).insert(Invincibility {
                        invincibility_timer: Timer::from_seconds(
                            FISH_INVINCIBILITY_TIME,
                            TimerMode::Once,
                        ),
                    });
                    *state = FishState::Swimming
                }
            }
        }
    }
}

fn handle_invincibilities(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Invincibility)>,
    timer: Res<Time>,
) {
    for (e, mut i) in &mut query {
        if i.invincibility_timer.tick(timer.delta()).finished() {
            commands.entity(e).remove::<Invincibility>();
        }
    }
}

pub fn spawn_fish(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    alive_fish: Res<AliveFish>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if alive_fish.count > 20 {
        return;
    }

    let fish: FishVariant = rand::random();
    let direction = Direction::random_y();

    let vertical_position = rand::thread_rng().gen_range(fish.get_spawn_depth_range());
    let horizontal_position = rand::thread_rng().gen_range(-1800.0..1800.);

    commands.spawn({
        let (texture_atlas, animation_indices) = fish.texture_atlas(asset_server.clone());
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        (
            FishBundle {
                sprite_sheet: SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite {
                        index: animation_indices.first,
                        flip_x: matches!(direction, Direction::Left),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(horizontal_position, -vertical_position, 5.),
                        scale: Vec3::splat(3.),
                        ..default()
                    },
                    ..default()
                },
                direction,
                speed: Speed {
                    current: rand::thread_rng().gen_range(FISH_SPEED_MIN..FISH_SPEED_MAX),
                },
                variant: fish,
                weight: Weight {
                    // Round weight to .2 decimal places
                    current: (rand::thread_rng().gen_range(fish.get_weight_range()) * 100.0_f32)
                        .round()
                        / 100.0,
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        )
    });
}

pub fn update_fish_count(fish_query: Query<&Fish>, mut alive_fish: ResMut<AliveFish>) {
    let fish_found = fish_query.iter().count() as u32;

    alive_fish.count = fish_found;
}
