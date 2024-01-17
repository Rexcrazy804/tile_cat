use bevy::{prelude::*, app::AppExit};

mod game;
use game::GamePlugin;

pub const SCALE_FACTOR: f32 = 4.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()), 
            GamePlugin
        ))
        .add_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_handler)
        .run()
    ;
}

#[allow(dead_code)]
#[derive(States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum GameState {
    MainMenu,
    #[default]
    Game,
    Gameover
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
    if key_input.pressed(KeyCode::Q) && key_input.pressed(KeyCode::Q) {
        exit_event_writer.send(AppExit)
    }
}
