use bevy::{prelude::*, window::PrimaryWindow};
use super::{
    GameState,
    SimulationState,
    SCALE_FACTOR,
};

#[derive(Component)]
struct Platform;

pub struct PlatformPlugin;
impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
    }
}
