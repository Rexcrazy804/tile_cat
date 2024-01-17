use bevy::{prelude::*, window::PrimaryWindow};
use super::{
    GameState,
    SimulationState,
    SCALE_FACTOR,
    cat::{
        Cat,
        CAT_SIZE
    },
};

const BULLET_SIZE: f32 = 16.0;
const BULLET_SPEED: f32 = 400.0;
const BULLET_Y_OFFSET: f32 = 10.0 / SCALE_FACTOR;

#[derive(Component)]
struct Bullet {
    direction_multiplier: f32,
}

#[derive(Event)]
pub struct BulletFireEvent(pub f32);

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BulletFireEvent>()
            .add_systems(Update, (
                spawn_bullet,
                move_bullet,
                despawn_bullet,
            )
                .run_if(in_state(GameState::Game))
                .run_if(in_state(SimulationState::Running))
            )
        ;
    }
}

fn spawn_bullet(
    mut commands: Commands,
    mut bullet_fire_reader: EventReader<BulletFireEvent>,
    cat_query: Query<&Transform, With<Cat>>,
    asset_server: Res<AssetServer>,
) {
    for direction_multiplier in bullet_fire_reader.read() {
        let Ok(cat_transform) = cat_query.get_single() else { return };

        let mut bullet_transform = cat_transform.clone();
        bullet_transform.translation.x += direction_multiplier.0 * (CAT_SIZE/2.0 + BULLET_SIZE/2.0);
        bullet_transform.translation.y -= BULLET_Y_OFFSET;

        let bullet_sprite_bundle = SpriteBundle {
            texture: asset_server.load("sprites/bullet/bullet.png"),
            transform: bullet_transform,
            ..default()
        };

        commands.spawn((
            bullet_sprite_bundle,
            Bullet { direction_multiplier: direction_multiplier.0 },
        ));
    }
}

fn move_bullet(
    mut transform_query: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut bullet_transform, bullet) in &mut transform_query {
        bullet_transform.translation.x += bullet.direction_multiplier * BULLET_SPEED * time.delta_seconds();
    }
}

fn despawn_bullet(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    entity_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    let Ok(window) = window_query.get_single() else { return };

    for (bullet, transform) in &entity_query {
        if transform.translation.x > ((window.width()/2.0)/SCALE_FACTOR) + BULLET_SIZE/2.0 {
            commands.entity(bullet).despawn()
        }
    }
}
