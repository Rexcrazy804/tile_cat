use bevy::prelude::*;

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
        .add_systems(Startup, (
            spawn_camera,
        ))
        .run()
    ;
}

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

