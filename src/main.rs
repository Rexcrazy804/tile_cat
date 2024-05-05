use bevy::asset::AssetMetaCheck;
use bevy::window::PresentMode;
use bevy::{app::AppExit, prelude::*};

mod game;
mod menu;

use game::GamePlugin;
use menu::MainMenuPlugin;

pub const SCALE_FACTOR: f32 = 4.0;

#[derive(States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum SimulationState {
    #[default]
    InActive,
    Running,
    Paused,
}

fn main() {
    let custom_window = WindowPlugin {
        primary_window: Some(Window {
            title: "Tile Cat".to_string(),
            fit_canvas_to_parent: true,
            canvas: Some(String::from("#bevy")),
            present_mode: PresentMode::AutoVsync,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    };

    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(Msaa::Off)
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(custom_window)
                .set(AssetPlugin {
                    mode: AssetMode::Unprocessed,
                    ..default()
                }),
            GamePlugin,
            MainMenuPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_handler)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale /= SCALE_FACTOR;

    commands.spawn(camera_bundle);
}

fn exit_handler(key_input: Res<Input<KeyCode>>, mut exit_event_writer: EventWriter<AppExit>) {
    if key_input.pressed(KeyCode::ShiftLeft) && key_input.pressed(KeyCode::Q) {
        exit_event_writer.send(AppExit)
    }
}
