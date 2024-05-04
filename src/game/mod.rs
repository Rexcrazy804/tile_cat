use crate::{GameState, SimulationState, SCALE_FACTOR};
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

mod bugs;
mod bullet;
mod cat;
mod clouds;
mod flora;
mod ground;

use bugs::BugPlugin;
use bullet::BulletPlugin;
use cat::CatPlugin;
use clouds::CloudPlugin;
use flora::FloraPlugin;
use ground::GroundPlugin;

pub const INITIAL_HEART_COUNT: u8 = 5;
const GRAVITY: f32 = 200.8;
const FRICTION: f32 = 0.8;
const DIFFICULTY_STEP: f32 = 0.15;
const DIFFICULTY_UPPER_LIMIT: f32 = 4.0;

#[derive(Component)]
struct Background;

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct Heart(pub u8);

#[derive(Resource)]
pub struct DifficultyMultiplier(pub f32);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugins((
                CatPlugin,
                CloudPlugin,
                BulletPlugin,
                GroundPlugin,
                FloraPlugin,
                BugPlugin,
            ))
            .insert_resource(Score(0))
            .insert_resource(Heart(INITIAL_HEART_COUNT))
            .insert_resource(DifficultyMultiplier(1.0))
            .add_systems(
                OnEnter(GameState::Game),
                (spawn_background, start_simulation),
            )
            .add_systems(
                OnExit(GameState::Game),
                (despawn_background, stop_simulation),
            )
            .add_systems(
                Update,
                (
                    toggle_simulation,
                    resize_bacground,
                    game_over.run_if(resource_changed::<Heart>()),
                    step_difficulty.run_if(resource_changed::<Score>()),
                )
                    .run_if(in_state(GameState::Game)),
            );
    }
}

enum EntityDirection {
    Left,
    Right,
}

fn spawn_background(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let mut my_background = SpriteBundle {
        sprite: Sprite {
            color: Color::hex("#fcdfcd").unwrap(),
            custom_size: Some(Vec2::new(
                window.width() / SCALE_FACTOR,
                window.height() / SCALE_FACTOR,
            )),
            ..default()
        },
        ..default()
    };
    my_background.transform.translation.z = -0.1;

    commands.spawn((my_background, Background));
}

fn despawn_background(mut commands: Commands, query: Query<Entity, With<Background>>) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    commands.entity(entity).despawn();
}

fn resize_bacground(
    mut background_query: Query<&mut Sprite, With<Background>>,
    mut window_reized_reader: EventReader<WindowResized>,
) {
    let Ok(mut background_sprite) = background_query.get_single_mut() else {
        return;
    };
    for window_resized in window_reized_reader.read() {
        background_sprite.custom_size = Some(Vec2::new(
            window_resized.width / SCALE_FACTOR,
            window_resized.height / SCALE_FACTOR,
        ))
    }
}

fn start_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::Running)
}

fn stop_simulation(mut next_state: ResMut<NextState<SimulationState>>) {
    next_state.set(SimulationState::InActive)
}

fn toggle_simulation(
    key_input: Res<Input<KeyCode>>,
    current_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if !key_input.just_pressed(KeyCode::Escape) {
        return;
    }
    match *current_state.get() {
        SimulationState::Running => next_state.set(SimulationState::Paused),
        SimulationState::Paused => next_state.set(SimulationState::Running),
        SimulationState::InActive => (),
    }
}

// fn handle_gamepad(
//     mut commands: Commands,
//     current_gamepad: Option<Res<Controller>>,
//     mut gamepad_evr: EventReader<GamepadEvent>,
// ) {
//     for event in gamepad_evr.read() {
//         if let GamepadEvent::Connection(connection) = event {
//             if connection.connected() {
//                 if current_gamepad.is_some() { return; }
//
//                 commands.insert_resource(Controller(dbg!(connection.gamepad.id)))
//             } else if let Some(ref controller) = current_gamepad {
//                 if controller.0 == connection.gamepad.id {
//                     commands.remove_resource::<Controller>()
//                 }
//             }
//         }
//     }
// }

pub fn reset_stats(
    mut score: ResMut<Score>,
    mut hearts: ResMut<Heart>,
    mut diffculty: ResMut<DifficultyMultiplier>,
) {
    score.0 = 0;
    hearts.0 = INITIAL_HEART_COUNT;
    diffculty.0 = 1.0;
}

fn step_difficulty(mut diffculty: ResMut<DifficultyMultiplier>, score: Res<Score>) {
    if diffculty.0 >= DIFFICULTY_UPPER_LIMIT {
        return;
    }
    diffculty.0 = 1.0 + ((score.0 / 50) as f32 * DIFFICULTY_STEP);
}

fn game_over(
    mut simulation_state: ResMut<NextState<SimulationState>>,
    mut game_state: ResMut<NextState<GameState>>,
    hearts: Res<Heart>,
) {
    if hearts.0 > 0 {
        return;
    }

    simulation_state.set(SimulationState::InActive);
    game_state.set(GameState::GameOver);
}
