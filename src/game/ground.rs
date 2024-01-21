use bevy::{prelude::*, window::WindowResized};

use rand::random;
use super::{
    GameState,
    SCALE_FACTOR, cat::CAT_SIZE,
};

pub const GROUND_WIDTH: f32 = 16.0;
pub const GROUND_HEIGHT: f32 = GROUND_WIDTH/2.0;
const GROUND_SPACING: f32 = 1.0;
const MAX_TEMP_GROUND: usize = 4;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct TempGround;

#[derive(Event)]
pub struct GroundBuildEvent(pub Vec3);

pub struct GroundPlugin;
impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GroundBuildEvent>()
            .add_systems(Update, (
                despawn_old_ground,
                spawn_new_ground,
            )
                .run_if(in_state(GameState::Game))
                .run_if(on_event::<WindowResized>())
            )
            .add_systems(Update, (
                build_ground_underneath_cat,
                despawn_temp_ground,
            )
                    .run_if(on_event::<GroundBuildEvent>())
            )
        ;
    }
}

fn spawn_new_ground(
    mut commands: Commands,
    mut window_resized_reader: EventReader<WindowResized>,
    asset_server: Res<AssetServer>
) {
    for window in window_resized_reader.read() {
        let ground_count = ((window.width/SCALE_FACTOR)/GROUND_WIDTH).ceil();
        let initial_x_pos = -((window.width/2.0)/SCALE_FACTOR) + GROUND_WIDTH/2.0;
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
            ground_sprite.transform.translation.x = initial_x_pos + (i as f32 * GROUND_WIDTH * GROUND_SPACING);

            commands.spawn((
                ground_sprite,
                Ground,
            ));
        }
    }
}

fn despawn_old_ground(
    mut commands: Commands,
    ground_query: Query<Entity, With<Ground>>,
) {
    for entity in &ground_query {
        commands.entity(entity).despawn()
    }
}

fn build_ground_underneath_cat(
    mut commands: Commands,
    mut ground_build_reader: EventReader<GroundBuildEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in ground_build_reader.read() {
        let cat_transform = event.0;

        let mut ground_sprite = SpriteBundle {
            texture: asset_server.load("sprites/ground/temp_ground_new.png"),
            ..default()
        };

        ground_sprite.transform.translation.y = cat_transform.y - (CAT_SIZE/2.0) - (GROUND_WIDTH/2.0);
        ground_sprite.transform.translation.x = cat_transform.x;

        commands.spawn((
            ground_sprite,
            Ground,
            TempGround,
        ));
    }
}

fn despawn_temp_ground(
    mut commands: Commands,
    query: Query<Entity, With<TempGround>>,
) {
    let mut vec: Vec<Entity> = Vec::new();

    for entity in &query {
        vec.push(entity);
    }
    if vec.len() <= MAX_TEMP_GROUND { return }

    vec.sort();
    let removable = vec.len() - MAX_TEMP_GROUND;
    let mut i = 0;

    for entity in vec {
        if i < removable {
            commands.entity(entity).despawn();
            i += 1;
        }
    }
}
