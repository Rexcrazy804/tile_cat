use bevy::prelude::*;
use crate::{
    GameState,
    SCALE_FACTOR
};

mod cat;
use cat::CatPlugin;

const GRAVITY: f32 = 200.8;
const FRICTION: f32 = 0.8;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CatPlugin)
            .add_state::<SimulationState>()
        ;
    }
}

#[derive(States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum SimulationState {
    #[default]
    Running,
    Paused
}
