use bevy::{prelude::*, window::{PrimaryWindow, WindowResized}};
use crate::{
    GameState,
    SCALE_FACTOR
};

mod cat;
mod clouds;
mod bullet;
mod ground;
mod flora;
mod bugs;

use cat::CatPlugin;
use clouds::CloudPlugin;
use bullet::BulletPlugin;
use ground::GroundPlugin;
use flora::FloraPlugin;
use bugs::BugPlugin;

const GRAVITY: f32 = 200.8;
const FRICTION: f32 = 0.8;

#[derive(Component)]
struct Background;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            .add_plugins((
                CatPlugin,
                CloudPlugin,
                BulletPlugin,
                GroundPlugin,
                FloraPlugin,
                BugPlugin,
            ))

            .add_systems(OnEnter(GameState::Game), spawn_background)
            .add_systems(Update, (
                toggle_simulation,
                resize_bacground,
            )
                .run_if(in_state(GameState::Game)))
        ;
    }
}

#[derive(States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum SimulationState {
    #[default]
    Running,
    Paused
}

enum EntityDirection {
    Left,
    Right,
}

fn spawn_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else { return };

    let mut my_background = SpriteBundle {
        sprite: Sprite {
            color: Color::hex("#fcdfcd").unwrap(),
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

    commands.spawn((
        my_background,
        Background,
    ));
}

fn resize_bacground(
    mut background_query: Query<&mut Sprite, With<Background>>,
    mut window_reized_reader: EventReader<WindowResized>
) {
    let Ok(mut background_sprite) = background_query.get_single_mut() else { return };
    for window_resized in window_reized_reader.read() {
        background_sprite.custom_size = Some(Vec2::new(
            window_resized.width/SCALE_FACTOR,
            window_resized.height/SCALE_FACTOR,
        ))
    }
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
