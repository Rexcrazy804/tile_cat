use crate::{
    game::{
        controlls::{CatAction, Controlls, ACTION_LIST},
        reset_stats, DifficultyMultiplier, Heart, Score, INITIAL_HEART_COUNT,
    },
    GameState, SimulationState,
};
use bevy::prelude::*;

mod buttons;

use buttons::{attach_button, button_interactions, ButtonType};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
struct GameOverMenu;

#[derive(Component)]
struct SettingsMenu;

#[derive(Component)]
struct StatsBar;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HeartText;

#[derive(Component)]
struct DifficultyText;

#[derive(Component)]
struct SettingsText(bool, Option<CatAction>);

pub struct MenusPlugin;
impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), (spawn_mainmenu, reset_stats))
            .add_systems(OnExit(GameState::MainMenu), despawn_mainmenu)
            .add_systems(OnEnter(GameState::Game), spawn_statsbar)
            .add_systems(OnExit(GameState::Game), despawn_statsbar)
            .add_systems(OnEnter(SimulationState::Paused), spawn_pausemenu)
            .add_systems(OnExit(SimulationState::Paused), despawn_pausemenu)
            .add_systems(OnEnter(GameState::GameOver), spawn_gameovermenu)
            .add_systems(OnExit(GameState::GameOver), despawn_gameovermenu)
            .add_systems(OnEnter(GameState::Settings), spawn_settings_menu)
            .add_systems(OnExit(GameState::Settings), despawn_settings_menu)
            .add_systems(
                Update,
                (
                    update_settings_text.run_if(in_state(GameState::Settings)),
                    button_interactions,
                    update_score.run_if(resource_changed::<Score>()),
                    update_heart.run_if(resource_changed::<Heart>()),
                    update_difficulty.run_if(resource_changed::<DifficultyMultiplier>()),
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
        attach_button(parent, ButtonType::Settings, "Settings");
        attach_button(parent, ButtonType::Quit, "Quit");
    });
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

fn spawn_statsbar(mut commands: Commands) {
    let bar_style = Style {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Start,
        justify_content: JustifyContent::SpaceBetween,
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

    let box_style = Style {
        flex_direction: FlexDirection::Row,
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

    let score_text = TextBundle {
        text: Text::from_sections([
            TextSection::new("score: ", text_style.clone()),
            TextSection::new("0", text_style.clone()),
        ]),
        ..default()
    };

    let heart_text = TextBundle {
        text: Text::from_sections([
            TextSection::new("Hearts: ", text_style.clone()),
            TextSection::new(INITIAL_HEART_COUNT.to_string(), text_style.clone()),
        ]),
        ..default()
    };

    let difficulty_text = TextBundle {
        text: Text::from_sections([
            TextSection::new("Difficulty: ", text_style.clone()),
            TextSection::new("1x", text_style.clone()),
        ]),
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: bar_style,
                background_color: Color::rgba(0.988, 0.875, 0.804, 0.0).into(),
                ..default()
            },
            StatsBar,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: box_style.clone(),
                    background_color: Color::hsl(0.0, 0.1, 0.3).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((score_text, ScoreText));
                });
            parent
                .spawn(NodeBundle {
                    style: box_style.clone(),
                    background_color: Color::hsl(0.0, 0.1, 0.3).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((heart_text, HeartText));
                });
            parent
                .spawn(NodeBundle {
                    style: box_style.clone(),
                    background_color: Color::hsl(0.0, 0.1, 0.3).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((difficulty_text, DifficultyText));
                });
        });
}

fn spawn_gameovermenu(mut commands: Commands, score: Res<Score>) {
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

    let text_style = TextStyle {
        font_size: 20.0,
        color: Color::WHITE,
        ..default()
    };

    let base = NodeBundle {
        style: menu_style,
        background_color: Color::rgba(0.988, 0.875, 0.804, 0.0).into(),
        ..default()
    };

    commands
        .spawn((base, GameOverMenu))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(format!("Score: {}", score.0), text_style),
                ..default()
            });
            attach_button(parent, ButtonType::ReturnToMenu, "Main Menu");
            attach_button(parent, ButtonType::Quit, "Quit");
        });
}

fn spawn_settings_menu(mut commands: Commands, kbd_controlls: Res<Controlls<KeyCode>>) {
    let menu_style = Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        row_gap: Val::Px(10.0),
        column_gap: Val::Px(10.0),
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: menu_style,
                background_color: Color::rgb(0.988, 0.875, 0.804).into(),
                ..default()
            },
            SettingsMenu,
        ))
        .with_children(|parent| {
            attach_button(parent, ButtonType::ReturnToMenu, "Return");
            for action in ACTION_LIST {
                attach_button(
                    parent,
                    ButtonType::SettingsButton(action),
                    &get_action_text(action, &kbd_controlls),
                );
            }
        });
}

fn get_action_text(action: CatAction, kbd_controlls: &Res<Controlls<KeyCode>>) -> String {
    match action {
        CatAction::Up => format!("Up: {:?}", kbd_controlls.up.unwrap_or(KeyCode::Unlabeled)),
        CatAction::Left => format!(
            "Left: {:?}",
            kbd_controlls.left.unwrap_or(KeyCode::Unlabeled)
        ),
        CatAction::Right => format!(
            "Right: {:?}",
            kbd_controlls.right.unwrap_or(KeyCode::Unlabeled)
        ),
        CatAction::Jump => format!(
            "Jump: {:?}",
            kbd_controlls.jump.unwrap_or(KeyCode::Unlabeled)
        ),
        CatAction::Fire => format!(
            "Fire: {:?}",
            kbd_controlls.fire.unwrap_or(KeyCode::Unlabeled)
        ),
        CatAction::ToggleWeapon => format!(
            "ToggleWeapon: {:?}",
            kbd_controlls.toggle_weapon.unwrap_or(KeyCode::Unlabeled)
        ),
        CatAction::PlaceBlock => format!(
            "PlaceBlock: {:?}",
            kbd_controlls.place_block.unwrap_or(KeyCode::Unlabeled)
        ),
        CatAction::Pause => format!(
            "Pause: {:?}",
            kbd_controlls.pause.unwrap_or(KeyCode::Unlabeled)
        ),
    }
}

fn update_settings_text(
    mut query: Query<(&mut Text, &SettingsText)>,
    kbd_controlls: Res<Controlls<KeyCode>>,
) {
    for (mut text, settings_text) in &mut query {
        if !settings_text.0 {
            continue;
        } // if the text does not belong to a setting, ignore it

        let Some(action) = settings_text.1 else {
            continue;
        };

        text.sections[0].value = get_action_text(action, &kbd_controlls);
    }
}

fn update_score(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    let Ok(mut score_text) = query.get_single_mut() else {
        return;
    };
    score_text.sections[1].value = score.0.to_string();
}

fn update_heart(mut query: Query<&mut Text, With<HeartText>>, heart: Res<Heart>) {
    let Ok(mut heart_text) = query.get_single_mut() else {
        return;
    };
    heart_text.sections[1].value = heart.0.to_string();
}

fn update_difficulty(
    mut query: Query<&mut Text, With<DifficultyText>>,
    diff: Res<DifficultyMultiplier>,
) {
    let Ok(mut diff_text) = query.get_single_mut() else {
        return;
    };
    diff_text.sections[1].value = format!("{:.2}x", diff.0);
}

fn despawn_mainmenu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    commands.entity(entity).despawn_recursive();
}

fn despawn_pausemenu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    commands.entity(entity).despawn_recursive();
}

fn despawn_gameovermenu(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    commands.entity(entity).despawn_recursive();
}

fn despawn_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsMenu>>) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    commands.entity(entity).despawn_recursive();
}

fn despawn_statsbar(mut commands: Commands, query: Query<Entity, With<StatsBar>>) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    commands.entity(entity).despawn_recursive();
}
