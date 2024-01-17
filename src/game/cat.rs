use bevy::{prelude::*, window::PrimaryWindow};
use super::{
    GameState,
    SimulationState,

    SCALE_FACTOR,
    GRAVITY,
    FRICTION,

    EntityDirection,
    bullet::BulletFireEvent,
};

pub const CAT_SIZE: f32 = 16.0;
const CAT_SPEEED: f32 = 25.0;
const CAT_JUMP_FORCE: f32 = 100.0;
const CAT_GUN_WEIGHT: f32 = 20.0;
const CAT_BULLET_ANIMATION_DURATION: f32 = 0.12;

#[derive(Component)]
pub struct Cat {
    velocity: Vec3,
    direction: EntityDirection,
    can_jump: bool,
    has_gun: bool,
    is_firing: bool,
}
impl Cat {
    fn new() -> Self {
        Self {
            velocity: Vec3::ZERO,
            direction: EntityDirection::Right,
            can_jump: false,
            has_gun: false,
            is_firing: false,
        }
    }
}

#[derive(Resource)]
struct CatBulletFireTimer(Timer);

pub struct CatPlugin;
impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), spawn_cat)
            .insert_resource(CatBulletFireTimer(
                Timer::from_seconds(CAT_BULLET_ANIMATION_DURATION, TimerMode::Once)
            ))
            .add_systems(Update, (
                (move_cat, jump_cat).before(confine_cat),
                confine_cat,
                animate_cat,
                toggle_cat_gun,
                fire_bullet_cat,
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) { 
    let texture_handle = asset_server.load("sprites/cat/cat_sheet_2.png");
    let atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        // columns, rows, padding, offset
        5, 1, None, None
    );

    let atlas_handle = texture_atlases.add(atlas);

    let cat_bundle = SpriteSheetBundle{
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(1),
        ..default()
    };

    commands.spawn((
        cat_bundle,
        Cat::new()
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
        cat.direction = EntityDirection::Right;
        cat.velocity.x += CAT_SPEEED;
    }
    if keyboard_input.pressed(KeyCode::A) {
        cat.direction = EntityDirection::Left;
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

fn get_min_max(limit: f32) -> (f32, f32) {
    let min = CAT_SIZE/2.0 - ((limit/2.0) / SCALE_FACTOR);
    let max = ((limit/2.0)/ SCALE_FACTOR) - CAT_SIZE/2.0;
    (min, max)
}

fn jump_cat(
    mut cat_query: Query<&mut Cat>,
    input: Res<Input<KeyCode>>,
) {
    let Ok(mut cat) = cat_query.get_single_mut() else { return };

    if input.pressed(KeyCode::W) && cat.can_jump {
        cat.velocity.y += CAT_JUMP_FORCE;
        if cat.has_gun {
            cat.velocity.y -= CAT_GUN_WEIGHT;
        }
        cat.can_jump = false;
    }
}

fn animate_cat(
    mut transform_query: Query<(&mut Transform, &Cat, &mut TextureAtlasSprite)>,
) {
    let Ok((mut transform, cat, mut sprite)) = transform_query.get_single_mut() else { return };

    if cat.velocity.x < 0.0 {
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    }
    if cat.velocity.x > 0.0 {
        transform.rotation = Quat::default();
    }

    if cat.can_jump {
        sprite.index = 0 + if cat.has_gun { 2 } else { 0 };
    } else {
        sprite.index = 1 + if cat.has_gun { 2 } else { 0 };
    }

    if cat.is_firing && cat.has_gun {
        sprite.index = 4;
    }
}



fn toggle_cat_gun(
    mut cat_query: Query<&mut Cat>,
    key_input: Res<Input<KeyCode>>,
) {
    if !key_input.just_pressed(KeyCode::G) { return }
    let Ok(mut cat) = cat_query.get_single_mut() else { return };

    cat.has_gun = !cat.has_gun
}

fn fire_bullet_cat(
    mut cat_query: Query<&mut Cat>,
    mut anim_time: ResMut<CatBulletFireTimer>,
    mut bullet_fire_writer: EventWriter<BulletFireEvent>,
    key_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut cat) = cat_query.get_single_mut() else { return };

    if !cat.has_gun { return }

    if anim_time.0.tick(time.delta()).just_finished() {
        cat.is_firing = false;
    }

    if key_input.pressed(KeyCode::Space) && anim_time.0.finished() {
        let direction_multiplier = match cat.direction {
            EntityDirection::Right => 1.0,
            EntityDirection::Left => -1.0,
        };
        bullet_fire_writer.send(BulletFireEvent(direction_multiplier));
        anim_time.0.reset();
        cat.is_firing = true;
    }
}
