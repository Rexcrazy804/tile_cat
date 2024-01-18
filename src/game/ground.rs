use bevy::{prelude::*, window::WindowResized};

use rand::random;
use super::{
    GameState,
    SCALE_FACTOR,
};

pub const GROUND_SIZE: f32 = 16.0;
const GROUND_SPACING: f32 = 1.0;

#[derive(Component)]
pub struct Ground;

pub struct GroundPlugin;
impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                despawn_old_ground,
                spawn_new_ground,
            )
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<WindowResized>())
            )
        ;
    }
}

#[allow(dead_code)]
fn spawn_new_ground(
    mut commands: Commands,
    mut window_resized_reader: EventReader<WindowResized>,
    asset_server: Res<AssetServer>
) {
    for window in window_resized_reader.read() {
        let ground_count = ((window.width/SCALE_FACTOR)/GROUND_SIZE).ceil();
        let initial_x_pos = -((window.width/2.0)/SCALE_FACTOR) + GROUND_SIZE/2.0;
        let y_pos = -(window.height/2.0)/SCALE_FACTOR;

        let random_sprite = || {
            format!( "sprites/ground/ground_{}.png",
                if random::<bool>() { 1 } else { 2 }
            )
        };

        for i in 0..ground_count as usize {
            let mut ground_sprite = SpriteBundle {
                texture: asset_server.load(random_sprite()),
                ..default()
            };

            ground_sprite.transform.translation.y = y_pos;
            ground_sprite.transform.translation.x = initial_x_pos + (i as f32 * GROUND_SIZE * GROUND_SPACING);

            commands.spawn((
                ground_sprite,
                Ground,
            ));
        }
    }
}

#[allow(dead_code)]
fn despawn_old_ground(
    mut commands: Commands,
    ground_query: Query<Entity, With<Ground>>,
) {
    for entity in &ground_query {
        commands.entity(entity).despawn()
    }
}
