use bevy::prelude::*;
use crate::{
    GameState,
    SimulationState,
};

mod buttons;

use buttons::{
    ButtonType,
    attach_button,
    button_interactions,
};


#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PauseMenu;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), spawn_mainmenu)
            .add_systems(OnExit(GameState::MainMenu), despawn_mainmenu)

            .add_systems(OnEnter(SimulationState::Paused), spawn_pausemenu)
            .add_systems(OnExit(SimulationState::Paused), despawn_pausemenu)

            .add_systems(Update, button_interactions)
        ;
    }
}

fn spawn_mainmenu(
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
        MainMenu
    ))
        .with_children(|parent| {
            attach_button(parent, ButtonType::Play, "Play");
            attach_button(parent, ButtonType::Quit, "Quit");
        })
    ;
}

fn despawn_mainmenu (
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    let Ok(entity) = query.get_single() else { warn!("No menu Entity"); return };
    commands.entity(entity).despawn_recursive();
}

fn spawn_pausemenu(
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
        background_color: Color::rgba(0.988, 0.875, 0.804, 0.0).into(),
        ..default()
    };

    commands.spawn((
        base,
        PauseMenu
    ))
        .with_children(|parent| {
            attach_button(parent, ButtonType::Resume, "Continue");
            attach_button(parent, ButtonType::ReturnToMenu, "Main Menu");
            attach_button(parent, ButtonType::Quit, "Quit");
        })
    ;
}

fn despawn_pausemenu (
    mut commands: Commands,
    query: Query<Entity, With<PauseMenu>>,
) {
    let Ok(entity) = query.get_single() else { warn!("No menu Entity"); return };
    commands.entity(entity).despawn_recursive();
}
