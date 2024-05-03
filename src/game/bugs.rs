use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use rand::{random, Rng};

use super::{bullet::Bullet, DifficultyMultiplier, GameState, Heart, Score, SimulationState, SCALE_FACTOR};

pub const BUG_SIZE: f32 = 16.0;
const BUG_SPAWN_RATE: f32 = 0.84;
const BUG_SPEED: f32 = 20.0;
const BUG_VERTICAL_SPEED: f32 = 15.0;
const BUG_ANIMATION_INTERVAL: f32 = 0.4;
const BUG_VERTICAL_MOVEMENT_INTERVAL: std::ops::Range<f32> = (1.0)..2.0;
const SPAWN_HORIZONTAL_PADDING: f32 = 16.0;

#[derive(Component)]
struct Bug;

#[derive(Component)]
struct BugCanFly(bool);

#[derive(Resource)]
struct BugSpawnTimer(Timer);

#[derive(Component)]
struct BugAnimateTimer(Timer);

#[derive(Component)]
struct BugVeritcalMovement(Timer, f32);

#[derive(Resource)]
struct BugAtlas(Vec<Handle<TextureAtlas>>);

fn repeating_timer(time: f32) -> Timer {
    Timer::from_seconds(time, TimerMode::Repeating)
}

pub struct BugPlugin;
impl Plugin for BugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BugSpawnTimer(repeating_timer(BUG_SPAWN_RATE)))
            .insert_resource(BugAtlas(Vec::new()))
            .add_systems(OnEnter(GameState::Game), init_bug_atlases)
            .add_systems(OnExit(GameState::Game), despawn_all_bugs)
            .add_systems(
                Update,
                (
                    move_bug.before(confine_bug),
                    confine_bug,
                    spawn_bug,
                    despawn_bug,
                    animate_bug,
                    eat_bullet_bug,
                    push_down_flightless_bug.run_if(on_event::<WindowResized>()),
                )
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

fn init_bug_atlases(
    asset_server: Res<AssetServer>,
    mut atlas_resource: ResMut<BugAtlas>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let bug1_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/bugs/fly_bug_0.png"),
        Vec2::new(16.0, 16.0),
        2,
        1,
        None,
        None,
    );

    let bug2_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/bugs/fly_bug_1.png"),
        Vec2::new(16.0, 16.0),
        2,
        1,
        None,
        None,
    );

    let bug3_atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/bugs/bug_0.png"),
        Vec2::new(16.0, 16.0),
        2,
        1,
        None,
        None,
    );

    atlas_resource.0.push(texture_atlases.add(bug1_atlas));
    atlas_resource.0.push(texture_atlases.add(bug2_atlas));
    atlas_resource.0.push(texture_atlases.add(bug3_atlas));
}

fn spawn_bug(
    mut commands: Commands,
    mut timer: ResMut<BugSpawnTimer>,

    bug_atlas: Res<BugAtlas>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let bug_selector = rand::thread_rng().gen_range(0..3);

    let mut bug_sprite = SpriteSheetBundle {
        texture_atlas: bug_atlas.0[bug_selector].clone(),
        sprite: TextureAtlasSprite::new(0),
        ..default()
    };

    let transform = &mut bug_sprite.transform.translation;

    if bug_selector == 2 {
        // CRAWLING BUG == 2, hence you don't want vertical random offset
        transform.y = -(window.height() / SCALE_FACTOR) / 2.0 + SPAWN_HORIZONTAL_PADDING
    } else {
        transform.y = ((window.height() / SCALE_FACTOR) / 2.0 - SPAWN_HORIZONTAL_PADDING)
            * random::<f32>()
            * if random::<bool>() { -1.0 } else { 1.0 };
    }
    transform.x = -(window.width() / SCALE_FACTOR) / 2.0 - BUG_SIZE / 2.0;

    let vertical_interval = rand::thread_rng().gen_range(BUG_VERTICAL_MOVEMENT_INTERVAL);

    commands.spawn((
        bug_sprite,
        Bug,
        BugAnimateTimer(repeating_timer(BUG_ANIMATION_INTERVAL)),
        BugVeritcalMovement(repeating_timer(vertical_interval), 0.0),
        BugCanFly(bug_selector != 2),
    ));
}

fn move_bug(
    mut bug_query: Query<(&mut Transform, &mut BugVeritcalMovement, &BugCanFly), With<Bug>>,
    time: Res<Time>,
    diff_mult: Res<DifficultyMultiplier>,
) {
    let mut rng = rand::thread_rng();
    for (mut bug_transform, mut movement, can_fly) in &mut bug_query {
        if movement.0.tick(time.delta()).just_finished() && can_fly.0 {
            movement.1 = rng.gen_range(-1..=1) as f32;
        }
        bug_transform.translation.y += movement.1 * BUG_VERTICAL_SPEED * time.delta_seconds() * diff_mult.0;
        bug_transform.translation.x += BUG_SPEED * time.delta_seconds() * diff_mult.0;
    }
}

fn despawn_bug(
    mut commands: Commands,
    mut hearts: ResMut<Heart>,
    transform_query: Query<(&Transform, Entity), With<Bug>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    for (transform, entity) in &transform_query {
        if transform.translation.x - (BUG_SIZE / 2.0) > (window.width() / 2.0) / SCALE_FACTOR {
            commands.entity(entity).despawn();
            if hearts.0 > 0 { hearts.0 -= 1 }
        }
    }
}

fn despawn_all_bugs(mut commands: Commands, query: Query<Entity, With<Bug>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn animate_bug(
    mut query: Query<(&mut TextureAtlasSprite, &mut BugAnimateTimer), With<Bug>>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation_timer) in &mut query {
        if !animation_timer.0.tick(time.delta()).just_finished() {
            continue;
        }
        sprite.index = if sprite.index == 0 { 1 } else { 0 };
    }
}

fn eat_bullet_bug(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    bug_query: Query<(&Transform, Entity), With<Bug>>,
) {
    for (bullet_tranform, bullet) in &bullet_query {
        for (bug_tranform, bug) in &bug_query {
            if bullet_tranform
                .translation
                .distance(bug_tranform.translation)
                < BUG_SIZE
            {
                commands.entity(bug).despawn();
                commands.entity(bullet).despawn();

                score.0 += 5;
            }
        }
    }
}

fn push_down_flightless_bug(
    mut window_resize: EventReader<WindowResized>,
    mut query: Query<(&BugCanFly, &mut BugVeritcalMovement), With<Bug>>,
) {
    for _ in window_resize.read() {
        for (can_fly, mut vertical_movement) in &mut query {
            if can_fly.0 {
                continue;
            }
            vertical_movement.1 = -1.0;
        }
    }
}

fn confine_bug(
    mut transform_query: Query<(&mut Transform, &mut BugVeritcalMovement), With<Bug>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let (y_min, y_max) = get_min_max(window.height());
    // let y_min = y_min + SPAWN_HORIZONTAL_PADDING;

    for (mut bug_transform, mut bug_movement) in &mut transform_query {
        if bug_transform.translation.y < y_min {
            bug_transform.translation.y = y_min;
            bug_movement.1 = 0.0;
        }

        if bug_transform.translation.y > y_max {
            bug_transform.translation.y = y_max;
            bug_movement.1 = 0.0;
            bug_movement.1 = 0.0;
        }
    }
}

// this is not similiar to the get_min_max in cat.rs
fn get_min_max(window_limit: f32) -> (f32, f32) {
    let min = SPAWN_HORIZONTAL_PADDING - ((window_limit / 2.0) / SCALE_FACTOR);
    let max = ((window_limit / 2.0) / SCALE_FACTOR) - BUG_SIZE / 2.0;
    (min, max)
}
