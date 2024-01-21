use bevy::prelude::*;
use rand::Rng;

pub const FLORA_SPAWN_RATE: f32 = 0.12;
const FLORA_SIZE: f32 = 16.0;

#[derive(Component)]
struct Flora;

#[derive(Event)]
pub struct FloraSpawnEvent(pub Entity);

pub struct FloraPlugin;
impl Plugin for FloraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FloraSpawnEvent>()
            .add_systems(Update, 
                spawn_flora
                    .run_if(on_event::<FloraSpawnEvent>())
            )
        ;
    }
}

fn spawn_flora(
    mut commands: Commands,
    mut event_reader: EventReader<FloraSpawnEvent>,
    asset_server: Res<AssetServer>,
) {

    let mut rng = rand::thread_rng();
    let mut random_sprite = || {
        format!("sprites/flora/flora_{}.png", rng.gen_range(1..=6))
    };

    for FloraSpawnEvent(entity) in event_reader.read() {
        let mut flora_sprite = SpriteBundle {
            texture: asset_server.load(random_sprite()),
            ..default()
        };

        flora_sprite.transform.translation.y += FLORA_SIZE;
        flora_sprite.transform.translation.z = -0.1;


        commands.entity(*entity).with_children(|parent| {
            parent.spawn((
                flora_sprite,
                Flora
            ));
        });
    }
}
