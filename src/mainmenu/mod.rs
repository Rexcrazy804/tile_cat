use bevy::prelude::*;
use crate::GameState;

mod buttons;

use buttons::{
    ButtonType,
    attach_button,
    button_interactions,
};


#[derive(Component)]
struct Menu;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(Update, button_interactions)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu)
        ;
    }
}

fn spawn_menu(
    mut commands: Commands,
) {
    let menu_style = Style {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        row_gap: Val::Px(10.0),
        column_gap: Val::Px(10.0),
        ..default()
    };

    let base = NodeBundle {
        style: menu_style,
        background_color: Color::rgb(0.988, 0.875, 0.804).into(),
        ..default()
    };

    commands.spawn((
        base,
        Menu
    ))
        .with_children(|parent| {
            attach_button(parent, ButtonType::PlayButton, "Play");
            attach_button(parent, ButtonType::QuitButton, "Quit");
        })
    ;
}

fn despawn_menu (
    mut commands: Commands,
    query: Query<Entity, With<Menu>>,
) {
    let Ok(entity) = query.get_single() else { warn!("No menu Entity"); return };
    commands.entity(entity).despawn_recursive();
}

