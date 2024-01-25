use bevy::{prelude::*, app::AppExit};

mod game;
mod mainmenu;

use game::GamePlugin;
use mainmenu::MainMenuPlugin;

pub const SCALE_FACTOR: f32 = 4.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tile Cat".to_string(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
            GamePlugin,
            MainMenuPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_handler)
        .run()
    ;
}

#[derive(States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
}

fn spawn_camera(
    mut commands: Commands,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale /= SCALE_FACTOR;

    commands.spawn(
        camera_bundle
    );
}

fn exit_handler(
    key_input: Res<Input<KeyCode>>,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    if key_input.pressed(KeyCode::ShiftLeft) && key_input.pressed(KeyCode::Q) {
        exit_event_writer.send(AppExit)
    }
}
