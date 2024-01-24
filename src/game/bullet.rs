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
const BULLET_Y_OFFSET: f32 = 2.0;

#[derive(Component)]
pub struct Bullet {
    direction_multiplier: f32,
}

#[derive(Event)]
pub struct BulletFireEvent(pub f32);

#[derive(Event)]
struct DestroyBulletEvent(pub Entity);

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BulletFireEvent>()
            .add_event::<DestroyBulletEvent>()
            .add_systems(Update, (
                spawn_bullet.run_if(on_event::<BulletFireEvent>()),
                move_bullet,
                despawn_bullet
                    .run_if(on_event::<DestroyBulletEvent>())
                    .after(move_bullet)
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

        let mut bullet_transform = *cat_transform;
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
    mut transform_query: Query<(&mut Transform, &Bullet, Entity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut destruction_writter: EventWriter<DestroyBulletEvent>,
) {
    let Ok(window) = window_query.get_single() else { return };

    for (mut bullet_transform, bullet, entity) in &mut transform_query {
        if bullet_transform.translation.x > ((window.width()/2.0)/SCALE_FACTOR) + BULLET_SIZE/2.0 {
            destruction_writter.send(DestroyBulletEvent(entity));
            continue
        }

        bullet_transform.translation.x += bullet.direction_multiplier * BULLET_SPEED * time.delta_seconds();
    }
}

fn despawn_bullet(
    mut commands: Commands,
    mut destruction_reader: EventReader<DestroyBulletEvent>,
) {
    for entity in destruction_reader.read() {
        commands.entity(entity.0).despawn();
    }
}
