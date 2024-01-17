use bevy::{prelude::*, window::PrimaryWindow};
use crate::{
    GameState,
    SCALE_FACTOR
};

mod cat;
mod platform;
mod clouds;

use cat::CatPlugin;
use clouds::CloudPlugin;

const GRAVITY: f32 = 200.8;
const FRICTION: f32 = 0.8;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            .add_plugins((
                CatPlugin,
                CloudPlugin,
            ))

            .add_systems(OnEnter(GameState::Game), spawn_background)
            .add_systems(Update, toggle_simulation.run_if(in_state(GameState::Game)))
        ;
    }
}

#[derive(States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum SimulationState {
    #[default]
    Running,
    Paused
}

fn spawn_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else { return };

    let mut my_background = SpriteBundle {
        sprite: Sprite {
            color: Color::hex("#fcdfcd").unwrap().into(),
            custom_size: Some(
                Vec2::new(
                    window.width() / SCALE_FACTOR, 
                    window.height() / SCALE_FACTOR 
                )
            ),
            ..default()
        },
        ..default()
    };
    my_background.transform.translation.z = -0.1;

    commands.spawn(
        my_background
    );
}

fn toggle_simulation(
    key_input: Res<Input<KeyCode>>,
    current_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>
) {
    if !key_input.just_pressed(KeyCode::Escape) { return }
    match *current_state.get() {
        SimulationState::Running => next_state.set(SimulationState::Paused),
        SimulationState::Paused => next_state.set(SimulationState::Running),
    }
}
