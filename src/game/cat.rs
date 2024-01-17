use bevy::{prelude::*, window::PrimaryWindow};
use super::{
    GameState,
    SimulationState,
    SCALE_FACTOR,
    GRAVITY,
    FRICTION,
};

const CAT_SIZE: f32 = 16.0 * SCALE_FACTOR;
const CAT_SPEEED: f32 = 30.0;
const CAT_JUMP_FORCE: f32 = 100.0;

#[derive(Component)]
struct Cat {
    velocity: Vec3,
    can_jump: bool,
}

pub struct CatPlugin;
impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_cat)
            .add_systems(Update, (
                (move_cat, jump_cat).before(confine_cat),
                confine_cat,
                face_cat,
            )
                .run_if(in_state(SimulationState::Running))
                .run_if(in_state(GameState::Game))
            )
            .add_systems(OnExit(GameState::Game), despawn_cat)
        ;
    }
}

fn spawn_cat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) { 
    let cat_bundle = SpriteBundle {
        texture: asset_server.load("sprites/cat/cat_idle.png"),
        ..default()
    };

    commands.spawn((
        cat_bundle,
        Cat {
            velocity: Vec3::ZERO,
            can_jump: false,
        }
    ));
}

fn despawn_cat(
    mut commands: Commands,
    cat_query: Query<Entity, With<Cat>>
) {
    let Ok(entity) = cat_query.get_single() else { return };
    commands.entity(entity).despawn();
}

fn move_cat(
    mut transform_query: Query<(&mut Transform, &mut Cat)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let Ok((mut transform, mut cat)) = transform_query.get_single_mut() else { return };

    if keyboard_input.pressed(KeyCode::D) {
        cat.velocity.x += CAT_SPEEED;
    }
    if keyboard_input.pressed(KeyCode::A) {
        cat.velocity.x -= CAT_SPEEED;
    }
    // GRAVITY
    cat.velocity.y -= GRAVITY * time.delta_seconds();
    // FRICTION
    cat.velocity.x -= cat.velocity.x  * (1.0 - FRICTION);

    transform.translation += cat.velocity * time.delta_seconds();

}

fn confine_cat(
    mut transform_query: Query<(&mut Transform, &mut Cat)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>
) {
    let Ok(window) = window_query.get_single() else { return };
    let Ok((mut transform, mut cat)) = transform_query.get_single_mut() else { return };

    // println!("{:?}", (window.height(), window.width()));
    // window default: 1280, 720

    let (y_min, y_max) = get_min_max(window.height());
    let (x_min, x_max) = get_min_max(window.width());


    transform.translation += cat.velocity * time.delta_seconds();

    if transform.translation.y < y_min {
        transform.translation.y = y_min;
        cat.velocity.y = 0.0;
        cat.can_jump = true;
    }
    if transform.translation.y > y_max {
        transform.translation.y = y_max
    }

    if transform.translation.x < x_min {
        transform.translation.x = x_min
    }
    if transform.translation.x > x_max {
        transform.translation.x = x_max
    }
}

fn jump_cat(
    mut cat_query: Query<&mut Cat>,
    input: Res<Input<KeyCode>>,
) {
    let Ok(mut cat) = cat_query.get_single_mut() else { return };

    if input.pressed(KeyCode::W) && cat.can_jump {
        cat.velocity.y += CAT_JUMP_FORCE;
        cat.can_jump = false;
    }
}

fn face_cat(
    mut transform_query: Query<(&mut Transform, &Cat)>,
) {
    let Ok((mut transform, cat)) = transform_query.get_single_mut() else { return };
    if cat.velocity.x < 0.0 {
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    }
    if cat.velocity.x > 0.0 {
        transform.rotation = Quat::default();
    }
}


fn get_min_max(limit: f32) -> (f32, f32) {
    let y_min = (CAT_SIZE/2.0 - limit/2.0) / SCALE_FACTOR;
    let y_max = (limit/2.0 - CAT_SIZE/2.0) / SCALE_FACTOR;
    (y_min, y_max)
}
