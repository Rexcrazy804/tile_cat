use super::{
    bullet::BulletFireEvent,
    ground::{Ground, GroundBuildEvent, GROUND_HEIGHT, GROUND_WIDTH},
    EntityDirection, GameState, SimulationState, FRICTION, GRAVITY, SCALE_FACTOR,
};
use bevy::{prelude::*, window::PrimaryWindow};

pub const CAT_SIZE: f32 = 16.0;
const CAT_SPEEED: f32 = 25.0;
const CAT_JUMP_FORCE: f32 = 80.0;
const CAT_BULLET_ANIMATION_DURATION: f32 = 0.12;
const MAX_COLLISION_RADIUS: f32 = 1.5;
const CAT_GUN_WEIGHT: f32 = 10.0; // subtracts from jump force when gun is equiped

// Inputs
const BUTTON_LEFT: [KeyCode; 2] = [KeyCode::A, KeyCode::Left];
const BUTTON_RIGHT: [KeyCode; 2] = [KeyCode::D, KeyCode::Right];
const BUTTON_JUMP: [KeyCode; 3] = [KeyCode::W, KeyCode::Up, KeyCode::Space];
const BUTTON_BUILD_GROUND: [KeyCode; 2] = [KeyCode::ShiftLeft, KeyCode::ShiftRight];

// Controller
const CONTROLLER_JUMP: GamepadButtonType = GamepadButtonType::South;
const CONTROLLER_TOGGLE_GUN: GamepadButtonType = GamepadButtonType::North;
const CONTROLLER_FIRE: GamepadButtonType = GamepadButtonType::RightTrigger2;
const CONTROLLER_BUILD_GROUND: GamepadButtonType = GamepadButtonType::LeftTrigger;

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
        app.add_systems(OnEnter(GameState::Game), spawn_cat)
            .insert_resource(CatBulletFireTimer(Timer::from_seconds(
                CAT_BULLET_ANIMATION_DURATION,
                TimerMode::Once,
            )))
            .add_systems(
                Update,
                (
                    move_cat.before(confine_cat),
                    jump_cat,
                    confine_cat,
                    animate_cat,
                    toggle_cat_gun,
                    fire_bullet_cat,
                    build_ground_cat,
                )
                    .run_if(in_state(SimulationState::Running))
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), despawn_cat);
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
        5,
        1,
        None,
        None,
    );

    let atlas_handle = texture_atlases.add(atlas);

    let cat_bundle = SpriteSheetBundle {
        texture_atlas: atlas_handle,
        sprite: TextureAtlasSprite::new(1),
        ..default()
    };

    commands.spawn((cat_bundle, Cat::new()));
}

fn despawn_cat(mut commands: Commands, cat_query: Query<Entity, With<Cat>>) {
    let Ok(entity) = cat_query.get_single() else {
        return;
    };
    commands.entity(entity).despawn();
}

fn move_cat(
    mut transform_query: Query<(&mut Transform, &mut Cat)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
) {
    let Ok((mut transform, mut cat)) = transform_query.get_single_mut() else {
        return;
    };

    let controller = gamepads.iter().next(); // get teh first connected game pad

    if let Some(id) = controller {
        // handle controller input
        let leftaxis_x = GamepadAxis::new(id, GamepadAxisType::LeftStickX);
        let leftaxis_y = GamepadAxis::new(id, GamepadAxisType::LeftStickY);

        if let (Some(x), Some(y)) = (axes.get(leftaxis_x), axes.get(leftaxis_y)) {
            let leftaxis = Vec2::new(x, y);

            if leftaxis.length() > 0.9 && leftaxis.x > 0.5 {
                cat.direction = EntityDirection::Right;
                cat.velocity.x += CAT_SPEEED;
            }

            if leftaxis.length() > 0.9 && leftaxis.x < 0.5 {
                cat.direction = EntityDirection::Left;
                cat.velocity.x -= CAT_SPEEED;
            }
        }
    }

    if keyboard_input.any_pressed(BUTTON_RIGHT) {
        cat.direction = EntityDirection::Right;
        cat.velocity.x += CAT_SPEEED;
    }
    if keyboard_input.any_pressed(BUTTON_LEFT) {
        cat.direction = EntityDirection::Left;
        cat.velocity.x -= CAT_SPEEED;
    }

    // GRAVITY
    cat.velocity.y -= GRAVITY * time.delta_seconds();
    // FRICTION
    cat.velocity.x -= cat.velocity.x * (1.0 - FRICTION);

    transform.translation += cat.velocity * time.delta_seconds();
}

fn confine_cat(
    mut transform_query: Query<(&mut Transform, &mut Cat), Without<Ground>>,
    ground_query: Query<&Transform, With<Ground>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok((mut cat_transform, mut cat)) = transform_query.get_single_mut() else {
        return;
    };

    // println!("{:?}", (window.height(), window.width()));
    // window default: 1280, 720

    let (y_min, y_max) = get_min_max(window.height());
    let (x_min, x_max) = get_min_max(window.width());

    cat_transform.translation += cat.velocity * time.delta_seconds();

    if cat_transform.translation.y < y_min {
        cat_transform.translation.y = y_min;
        cat.velocity.y = 0.0;
        cat.can_jump = true;
    }
    if cat_transform.translation.y > y_max {
        cat_transform.translation.y = y_max
    }

    if cat_transform.translation.x < x_min {
        cat_transform.translation.x = x_min
    }
    if cat_transform.translation.x > x_max {
        cat_transform.translation.x = x_max
    }

    // ground_collision
    for ground_transfrom in &ground_query {
        let mut ground_top = ground_transfrom.translation;
        ground_top.y += GROUND_WIDTH / 2.0;

        let mut cat_bottom = cat_transform.translation;
        cat_bottom.y -= CAT_SIZE / 2.0;

        if !(cat_bottom.x + CAT_SIZE / 2.0 >= ground_top.x - (GROUND_WIDTH / 2.0)
            && cat_bottom.x - CAT_SIZE <= ground_top.x + (GROUND_WIDTH / 2.0))
        {
            continue;
        }
        if cat_bottom.distance(ground_top) > GROUND_HEIGHT * MAX_COLLISION_RADIUS {
            continue;
        }

        let ground_limit = ground_transfrom.translation.y + GROUND_WIDTH;

        if cat_transform.translation.y < ground_limit {
            cat_transform.translation.y = ground_limit;
            cat.velocity.y = 0.0;
            cat.can_jump = true;
        }
    }
}

fn get_min_max(window_limit: f32) -> (f32, f32) {
    let min = CAT_SIZE / 2.0 - ((window_limit / 2.0) / SCALE_FACTOR);
    let max = ((window_limit / 2.0) / SCALE_FACTOR) - CAT_SIZE / 2.0;
    (min, max)
}

fn jump_cat(
    mut cat_query: Query<&mut Cat>,
    input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    button: Res<Input<GamepadButton>>,
) {
    let Ok(mut cat) = cat_query.get_single_mut() else {
        return;
    };

    let controller = gamepads.iter().next(); // get teh first connected game pad
    let mut controller_jump = false;

    if let Some(id) = controller {
        // handle controller input
        let jump_button = GamepadButton::new(id, CONTROLLER_JUMP);

        controller_jump = button.just_pressed(jump_button);
    }

    if (input.any_just_pressed(BUTTON_JUMP) || controller_jump) && cat.can_jump {
        cat.velocity.y += CAT_JUMP_FORCE;
        if cat.has_gun {
            cat.velocity.y -= CAT_GUN_WEIGHT;
        }
        cat.can_jump = false;
    }
}

fn animate_cat(mut transform_query: Query<(&mut Transform, &Cat, &mut TextureAtlasSprite)>) {
    let Ok((mut transform, cat, mut sprite)) = transform_query.get_single_mut() else {
        return;
    };

    match cat.direction {
        EntityDirection::Left => transform.rotation = Quat::from_rotation_y(std::f32::consts::PI),

        EntityDirection::Right => transform.rotation = Quat::default(),
    }

    if cat.can_jump {
        sprite.index = if cat.has_gun { 2 } else { 0 };
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
    controller_input: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
) {
    let Ok(mut cat) = cat_query.get_single_mut() else {
        return;
    };

    let controller = gamepads.iter().next(); // get teh first connected game pad
    let mut controller_toggle = false;

    if let Some(id) = controller {
        // handle controller input
        let gun_toggle_button = GamepadButton::new(id, CONTROLLER_TOGGLE_GUN);

        controller_toggle = controller_input.just_pressed(gun_toggle_button);
    }

    if key_input.just_pressed(KeyCode::F) || controller_toggle {
        cat.has_gun = !cat.has_gun
    }
}

fn fire_bullet_cat(
    mut cat_query: Query<&mut Cat>,
    mut anim_time: ResMut<CatBulletFireTimer>,
    mut bullet_fire_writer: EventWriter<BulletFireEvent>,
    mouse_input: Res<Input<MouseButton>>,
    controller_input: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
    time: Res<Time>,
) {
    let Ok(mut cat) = cat_query.get_single_mut() else {
        return;
    };

    if !cat.has_gun {
        return;
    }

    if anim_time.0.tick(time.delta()).just_finished() {
        cat.is_firing = false;
    }

    let controller = gamepads.iter().next(); // get teh first connected game pad
    let mut controller_fire = false;

    if let Some(id) = controller {
        // handle controller input
        let fire_gun = GamepadButton::new(id, CONTROLLER_FIRE);

        controller_fire = controller_input.just_pressed(fire_gun);
    }

    if (mouse_input.just_pressed(MouseButton::Left) || controller_fire) && anim_time.0.finished() {
        let direction_multiplier = match cat.direction {
            EntityDirection::Right => 1.0,
            EntityDirection::Left => -1.0,
        };
        bullet_fire_writer.send(BulletFireEvent(direction_multiplier));
        anim_time.0.reset();
        cat.is_firing = true;
    }
}

fn build_ground_cat(
    mut ground_build_writer: EventWriter<GroundBuildEvent>,
    controller_input: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
    key_input: Res<Input<KeyCode>>,
    transform_query: Query<&Transform, With<Cat>>,
) {
    let Ok(transform) = transform_query.get_single() else {
        return;
    };

    let controller = gamepads.iter().next(); // get teh first connected game pad
    let mut controller_build = false;

    if let Some(id) = controller {
        // handle controller input
        let build_button = GamepadButton::new(id, CONTROLLER_BUILD_GROUND);

        controller_build = controller_input.just_pressed(build_button);
    }

    if key_input.any_just_pressed(BUTTON_BUILD_GROUND) || controller_build {
        ground_build_writer.send(GroundBuildEvent(transform.translation));
    }
}
