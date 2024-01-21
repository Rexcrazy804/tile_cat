use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::{
    GameState,
    SimulationState,
    SCALE_FACTOR,
};

pub const BUG_SIZE: f32 = 16.0;
const BUG_SPEED: f32 = 10.0;

#[derive(Component)]
struct Bug;

#[derive(Component)]
struct Flight;

pub struct BugPlugin;
impl Plugin for BugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_bug)
            .add_systems(Update, (
                move_bug
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(SimulationState::Running))
            )
        ;
    }
}

fn spawn_bug(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else { return };

    let mut bug_sprite = SpriteBundle {
        texture: asset_server.load("sprites/bugs/bug_0_1.png"),
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
