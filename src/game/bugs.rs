use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::{
    GameState,
    SimulationState,
    SCALE_FACTOR,
    bullet::Bullet,
};

pub const BUG_SIZE: f32 = 16.0;
const BUG_SPAWN_RATE: f32 = 1.84;
const BUG_SPEED: f32 = 20.0;
const BUG_ANIMATION_INTERVAL: f32 = 0.4;

#[derive(Component)]
struct Bug;

#[derive(Component)]
struct Flight;


pub struct BugPlugin;
impl Plugin for BugPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BugSpawnTimer(Timer::from_seconds(BUG_SPAWN_RATE, TimerMode::Repeating)))
            .insert_resource(BugAnimateTimer(Timer::from_seconds(BUG_ANIMATION_INTERVAL, TimerMode::Repeating)))
            .insert_resource(BugAtlas(Vec::new()))

            .add_systems(OnEnter(GameState::Game), init_bug_texture)
            .add_systems(Update, (
                move_bug,
                spawn_bug,
                despawn_bug,
                animate_bugs,
                eat_bullet_bug,
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(SimulationState::Running))
            )
        ;
    }
}

#[derive(Resource)]
struct BugSpawnTimer(Timer);

#[derive(Resource)]
struct BugAnimateTimer(Timer);

#[derive(Resource)]
struct BugAtlas(Vec<Handle<TextureAtlas>>);

fn init_bug_texture(
    asset_server: Res<AssetServer>,
    mut atlas_resource: ResMut<BugAtlas>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let bug1_atlas = TextureAtlas::from_grid (
        asset_server.load("sprites/bugs/fly_bug_0.png"),
        Vec2::new(16.0, 16.0),
        2, 1, None, None,
    );
    let bug2_atlas = TextureAtlas::from_grid (
        asset_server.load("sprites/bugs/fly_bug_1.png"),
        Vec2::new(16.0, 16.0),
        2, 1, None, None,
    );

    atlas_resource.0.push(texture_atlases.add(bug1_atlas));
    atlas_resource.0.push(texture_atlases.add(bug2_atlas));
}

fn spawn_bug(
    mut commands: Commands,
    mut timer: ResMut<BugSpawnTimer>,

    bug_atlas: Res<BugAtlas>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>
) {
    if !timer.0.tick(time.delta()).just_finished() { return }
    let Ok(window) = window_query.get_single() else { return };

    let mut bug_sprite = SpriteSheetBundle {
        texture_atlas: bug_atlas.0[
            if random::<bool>() { 1 } else { 0 }
        ].clone(),
        sprite: TextureAtlasSprite::new(0),
        ..default()
    };

    let transform = &mut bug_sprite.transform.translation;

    transform.y = (window.height()/SCALE_FACTOR)/2.0 * random::<f32>() * if random::<bool>() { -1.0 } else { 1.0 };
    transform.x = -(window.width()/SCALE_FACTOR)/2.0 - BUG_SIZE/2.0;

    commands.spawn((
        bug_sprite,
        Bug,
    ));
}

fn move_bug(
    mut bug_query: Query<&mut Transform, With<Bug>>,
    time: Res<Time>
) {
    for mut bug_transform in &mut bug_query {
        bug_transform.translation.x += BUG_SPEED * time.delta_seconds();
    }
}

fn despawn_bug(
    mut commands: Commands,
    transform_query: Query<(&Transform, Entity), With<Bug>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else { return };
    for (transform, entity) in &transform_query {
        if transform.translation.x - (BUG_SIZE/2.0) > (window.width()/2.0)/SCALE_FACTOR {
            commands.entity(entity).despawn();
        }
    }
}

fn animate_bugs(
    mut query: Query<&mut TextureAtlasSprite, With<Bug>>,
    mut animation_timer: ResMut<BugAnimateTimer>,
    time: Res<Time>,
) {
    if !animation_timer.0.tick(time.delta()).just_finished() { return }

    for mut sprite in &mut query {
        sprite.index = if sprite.index == 0 { 1 } else { 0 };
    }
}

fn eat_bullet_bug(
    mut commands: Commands,
    bullet_query: Query<(&Transform, Entity), With<Bullet>>,
    bug_query: Query<(&Transform, Entity), With<Bug>>,
) {
    for (bullet_tranform, bullet) in &bullet_query {
        for (bug_tranform, bug) in &bug_query {
            if bullet_tranform.translation.distance(bug_tranform.translation) < BUG_SIZE {
                commands.entity(bug).despawn();
                commands.entity(bullet).despawn();
            }
        }
    }
}
