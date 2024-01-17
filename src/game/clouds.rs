use bevy::{prelude::*, window::PrimaryWindow};
use rand::{random, Rng};
use super::{
    GameState,
    SimulationState,
    SCALE_FACTOR,
};

const CLOUD_SIZE: f32 = 16.0 * SCALE_FACTOR;
const CLOUD_SPAWN_RATE: f32 = 0.69;
const CLOUD_SPEED: f32 = 15.0;

#[derive(Component)]
struct Cloud {
    speed: f32,
}


#[derive(Resource)]
struct CloudTimer(Timer);
impl CloudTimer {
    fn new() -> Self {
        Self(Timer::from_seconds(CLOUD_SPAWN_RATE, TimerMode::Repeating))
    }
}

pub struct CloudPlugin;
impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CloudTimer::new())
            .add_systems(Update, (
                move_clouds,
                spawn_cloud,
                despawn_outbound_cloud,
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(GameState::Game), despawn_clouds)
        ;
    }
}

fn spawn_cloud(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<CloudTimer>
) {
    if !spawn_timer.0.tick(time.delta()).just_finished() { return; }

    let mut random_number = rand::thread_rng();
    let Ok(window) = window_query.get_single() else { return };

    let random_texture = format!("sprites/clouds/cloud_{}.png", random_number.gen_range(1..=3));
    let mut cloud_sprite = SpriteBundle {
        texture: asset_server.load(random_texture),
        ..default()
    };

    cloud_sprite.transform.translation.y += (window.height()/2.0)/SCALE_FACTOR * random::<f32>();
    cloud_sprite.transform.translation.x += (window.width()/2.0)/SCALE_FACTOR + (CLOUD_SIZE/SCALE_FACTOR)/2.0;

    commands.spawn((
        cloud_sprite,
        Cloud { speed: (0.5 + random::<f32>()%0.5) * CLOUD_SPEED }
    ));
}

fn move_clouds(
    mut transform_query: Query<(&mut Transform, &Cloud)>,
    time: Res<Time>,
) {
    for (mut transform, cloud) in &mut transform_query {
        transform.translation.x -=  cloud.speed * time.delta_seconds();
    }
}

fn despawn_outbound_cloud (
    mut commands: Commands,
    cloud_query: Query<(Entity, &Transform), With<Cloud>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else { return };

    for (entity, transform) in &cloud_query {
        if transform.translation.x < -(window.width()/2.0)/SCALE_FACTOR - CLOUD_SIZE {
            commands.entity(entity).despawn()
        }
    }
}

fn despawn_clouds(
    mut commands: Commands,
    cloud_query: Query<Entity, With<Cloud>>
) {
    for entity in &cloud_query {
        commands.entity(entity).despawn();
    }
}
