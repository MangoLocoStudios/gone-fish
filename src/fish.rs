use crate::components::CameraShake;
use crate::events::{CatchFishEvent, DropFishEvent, ReelingFishEvent};
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
use rand::{
    distributions::{Standard, WeightedIndex},
    prelude::Distribution,
    Rng,
};
use std::slice::Iter;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FishVariant {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl FishVariant {
    pub fn iterator() -> Iter<'static, FishVariant> {
        static FISH_VARIANTS: [FishVariant; 8] = [
            FishVariant::One,
            FishVariant::Two,
            FishVariant::Three,
            FishVariant::Four,
            FishVariant::Five,
            FishVariant::Six,
            FishVariant::Seven,
            FishVariant::Eight,
        ];
        FISH_VARIANTS.iter()
    }

    pub fn texture_atlas(self, asset_server: AssetServer) -> (TextureAtlas, AnimationIndices) {
        (
            match self {
                FishVariant::One => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/1.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(12., 6.), 2, 1, None, None)
                }
                FishVariant::Two => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/2.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(16., 12.), 2, 1, None, None)
                }
                FishVariant::Three => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/3.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(20., 12.), 2, 1, None, None)
                }
                FishVariant::Four => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/4.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(26., 12.), 2, 1, None, None)
                }
                FishVariant::Five => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/5.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(30., 12.), 2, 1, None, None)
                }
                FishVariant::Six => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/6.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(54., 22.), 2, 1, None, None)
                }
                FishVariant::Seven => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/7.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(30., 12.), 2, 1, None, None)
                }
                FishVariant::Eight => {
                    let texture_handle = asset_server.load("craftpix/objects/Catch/8.png");
                    TextureAtlas::from_grid(texture_handle, Vec2::new(28., 24.), 2, 1, None, None)
                }
            },
            AnimationIndices { first: 0, last: 1 },
        )
    }

    pub fn get_spawn_depth_range(self) -> std::ops::Range<f32> {
        match self {
            FishVariant::One => 100.0..200.,
            FishVariant::Two => 150.0..500.,
            FishVariant::Three => 400.0..800.,
            FishVariant::Four => 700.0..1000.,
            FishVariant::Five => 950.0..1300.,
            FishVariant::Six => 100.0..2000.,
            FishVariant::Seven => 1600.0..2000.,
            FishVariant::Eight => 2300.0..2800.,
        }
    }

    pub fn get_weight_range(self) -> std::ops::Range<f32> {
        match self {
            FishVariant::One => 0.1..0.3,
            FishVariant::Two => 0.3..1.,
            FishVariant::Three => 3. ..6.,
            FishVariant::Four => 6. ..15.,
            FishVariant::Five => 15. ..25.,
            FishVariant::Six => 100. ..500.,
            FishVariant::Seven => 35. ..70.,
            FishVariant::Eight => 80. ..150.,
        }
    }
}

impl Distribution<FishVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FishVariant {
        match rng.gen_range(0..=7) {
            0 => FishVariant::One,
            1 => FishVariant::Two,
            2 => FishVariant::Three,
            3 => FishVariant::Four,
            4 => FishVariant::Five,
            5 => FishVariant::Six,
            6 => FishVariant::Seven,
            _ => FishVariant::Eight,
        }
    }
}

#[derive(Component, Debug, PartialEq)]
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
            variant: FishVariant::One,
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
            .add_event::<CatchFishEvent>()
            .add_event::<DropFishEvent>()
            .add_event::<ReelingFishEvent>()
            .init_resource::<AliveFish>()
            .add_systems(OnEnter(Game), setup)
            .add_systems(
                Update,
                (
                    update_fish_count,
                    spawn_fish,
                    fish_movement,
                    orient_fish,
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
                // Move the thing
                match *direction {
                    Direction::Left => {
                        transform.translation.x -= 1.0 * time.delta_seconds() * speed.current
                    }
                    Direction::Right => {
                        transform.translation.x += 1.0 * time.delta_seconds() * speed.current
                    }
                    Direction::Down => {
                        transform.translation.y -= 0.5 * time.delta_seconds() * speed.current
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
    mut fish_query: Query<(&mut Transform, &FishState, &mut Direction, &CanDie), With<Fish>>,
) {
    for (transform, _fish_state, mut direction, can_die) in &mut fish_query {
        if transform.translation.y > -100. && *direction != Direction::Down {
            *direction = Direction::Down
        }

        if transform.translation.y < -100. && *direction == Direction::Down {
            *direction = Direction::random_y();
        }

        if can_die.dying {
            return;
        }

        // Flip the thing when at edge
        if transform.translation.x < -1800. {
            *direction = Direction::Right;
        } else if transform.translation.x > 1800. {
            *direction = Direction::Left;
        }
    }
}

pub fn orient_fish(
    mut fish_query: Query<(&mut TextureAtlasSprite, &mut Transform, &mut Direction), With<Fish>>,
) {
    for (mut fish, mut transform, direction) in &mut fish_query {
        match *direction {
            Direction::Left => {
                fish.flip_x = true;
                transform.rotation = Quat::IDENTITY;
            }
            Direction::Right => {
                fish.flip_x = false;
                transform.rotation = Quat::IDENTITY;
            }
            Direction::Up => {
                fish.flip_x = false;
                transform.rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.0);
            }
            Direction::Down => {
                fish.flip_x = false;
                transform.rotation = Quat::from_rotation_z(-std::f32::consts::PI / 2.0);
            }
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
    mut catch_fish_event: EventWriter<CatchFishEvent>,
    mut drop_fish_event: EventWriter<DropFishEvent>,
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

                        drop_fish_event.send_default();
                        println!(
                            "[DEBUG] Fish {:?} was too heavy, weighing at {} - current weight {} - max weight {}",
                            (fish_variant, weight), weight.current, fish_storage.current, fish_storage.max
                        );
                    } else {
                        fish_storage.current += weight.current;
                        fish_stored.fish.push((*fish_variant, *weight));
                        commands.entity(fish).despawn();
                        catch_fish_event.send(CatchFishEvent {
                            weight: *weight,
                            fish_variant: *fish_variant,
                        });

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
    mut reeling_fish_event: EventWriter<ReelingFishEvent>,
    mut fish_query: Query<
        (Entity, &mut FishState, &FishVariant, &Weight),
        (With<Fish>, Without<Invincibility>),
    >,
) {
    for ev in fish_collision_with_rod_event.read() {
        for (fish, mut state, fish_variant, weight) in &mut fish_query {
            if fish != ev.fish {
                continue;
            }
            reeling_fish_event.send(ReelingFishEvent {
                weight: *weight,
                fish_variant: *fish_variant,
            });

            *state = FishState::Caught
        }
    }
}

pub fn check_for_trash_collisions(
    mut commands: Commands,
    mut trash_collision_event: EventReader<TrashCollisionEvent>,
    mut fish_query: Query<(Entity, &mut FishState), With<Fish>>,
    camera_query: Query<(Entity, &Transform), (Without<Fish>, With<Camera2d>)>,
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
                    *state = FishState::Swimming;

                    let (camera_entity, camera_transform) = camera_query.single();

                    commands.entity(camera_entity).insert(CameraShake {
                        shake_timer: Timer::from_seconds(0.1, TimerMode::Once),
                        intensity: 0.8,
                        start_translation: camera_transform.translation,
                    });
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
    if alive_fish.count > 80 {
        return;
    }

    let fish_weights = [100, 70, 55, 40, 30, 2, 15, 9];
    let dist = WeightedIndex::new(fish_weights).unwrap();

    let mut rng = rand::thread_rng();

    let fish_index: usize = dist.sample(&mut rng);
    let fish = match fish_index {
        0 => FishVariant::One,
        1 => FishVariant::Two,
        2 => FishVariant::Three,
        3 => FishVariant::Four,
        4 => FishVariant::Five,
        5 => FishVariant::Six,
        6 => FishVariant::Seven,
        7 => FishVariant::Eight,
        _ => panic!("Invalid fish variant index"), // This should not happen
    };

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
