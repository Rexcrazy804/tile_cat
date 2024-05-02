use crate::{game::Score, GameState, SimulationState};
use bevy::prelude::*;

mod buttons;

use buttons::{attach_button, button_interactions, ButtonType};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
struct ScoreBox;
#[derive(Component)]
struct ScoreText;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_mainmenu)
            .add_systems(OnExit(GameState::MainMenu), despawn_mainmenu)
            .add_systems(OnEnter(GameState::Game), spawn_scorebox)
            .add_systems(OnExit(GameState::Game), despawn_scorebox)
            .add_systems(OnEnter(SimulationState::Paused), spawn_pausemenu)
            .add_systems(OnExit(SimulationState::Paused), despawn_pausemenu)

            .add_systems(
                Update,
                (
                    button_interactions,
                    update_score.run_if(resource_changed::<Score>()),
                ),
            );
    }
}

fn spawn_mainmenu(mut commands: Commands) {
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

    commands.spawn((base, MainMenu)).with_children(|parent| {
        attach_button(parent, ButtonType::Play, "Play");
        attach_button(parent, ButtonType::Quit, "Quit");
    });
}

fn despawn_mainmenu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    let Ok(entity) = query.get_single() else {
        warn!("No menu Entity");
        return;
    };
    commands.entity(entity).despawn_recursive();
}

fn spawn_pausemenu(mut commands: Commands) {
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

    commands.spawn((base, PauseMenu)).with_children(|parent| {
        attach_button(parent, ButtonType::Resume, "Continue");
        attach_button(parent, ButtonType::ReturnToMenu, "Main Menu");
        attach_button(parent, ButtonType::Quit, "Quit");
    });
}

fn spawn_scorebox(mut commands: Commands) {
    let base_style = Style {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Start,
        justify_content: JustifyContent::Start,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        row_gap: Val::Px(10.0),
        column_gap: Val::Px(10.0),
        padding: UiRect {
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            bottom: Val::Px(5.0),
            top: Val::Px(5.0),
        },
        ..default()
    };

    let scorebox_style = Style {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Start,
        padding: UiRect {
            left: Val::Px(5.0),
            right: Val::Px(10.0),
            bottom: Val::Px(5.0),
            top: Val::Px(5.0),
        },
        ..default()
    };

    let text_style = TextStyle {
        font_size: 14.0,
        color: Color::WHITE,
        ..default()
    };

    let text = TextBundle {
        text: Text::from_sections([
            TextSection::new("score: ", text_style.clone()),
            TextSection::new("0", text_style),
        ]),
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: base_style,
                background_color: Color::rgba(0.988, 0.875, 0.804, 0.0).into(),
                ..default()
            },
            ScoreBox,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: scorebox_style,
                    background_color: Color::hsl(0.0, 0.1, 0.3).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((text, ScoreText));
                });
        });
}

fn update_score(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    let Ok(mut score_text) = query.get_single_mut() else {
        return;
    };
    score_text.sections[1].value = score.0.to_string();
}

fn despawn_pausemenu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    let Ok(entity) = query.get_single() else {
        warn!("No menu Entity");
        return;
    };
    commands.entity(entity).despawn_recursive();
}

fn despawn_scorebox(mut commands: Commands, query: Query<Entity, With<ScoreBox>>) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    commands.entity(entity).despawn_recursive();
}
