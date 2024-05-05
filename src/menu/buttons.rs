use crate::game::controlls::{CatAction, ControllChange};
use crate::SimulationState;
use bevy::{app::AppExit, prelude::*};

use super::{GameState, SettingsText};

const DEFUALT_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVER_BUTTON_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component, Clone)]
pub enum ButtonType {
    Play,
    Quit,
    Settings,
    Resume,
    ReturnToMenu,
    SettingsButton(CatAction),
}


pub fn button_interactions(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    mut query: Query<(&Interaction, &ButtonType, &mut BackgroundColor), Changed<Interaction>>,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    for (&interaction, button_type, background) in &mut query {
        handle_background(interaction, background);
        if interaction == Interaction::Pressed {
            match *button_type {
                ButtonType::Play => next_game_state.set(GameState::Game),
                ButtonType::Quit => exit_event_writer.send(AppExit),
                ButtonType::Resume => next_sim_state.set(SimulationState::Running),
                ButtonType::Settings => next_game_state.set(GameState::Settings),
                ButtonType::ReturnToMenu => {
                    next_game_state.set(GameState::MainMenu);
                }
                ButtonType::SettingsButton(action) => {
                    commands.insert_resource(ControllChange(action));
                }
            };
        }
    }
}

fn handle_background(interaction: Interaction, mut background: Mut<'_, BackgroundColor>) {
    match interaction {
        Interaction::Pressed => *background = PRESSED_BUTTON_COLOR.into(),
        Interaction::Hovered => *background = HOVER_BUTTON_COLOR.into(),
        Interaction::None => *background = DEFUALT_BUTTON_COLOR.into(),
    };
}

pub fn attach_button(parent: &mut ChildBuilder, button_type: ButtonType, button_text: &str) {
    let button_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let text_style = TextStyle {
        font_size: 14.0,
        color: Color::WHITE,
        ..default()
    };

    parent.spawn((
        ButtonBundle {
            style: button_style,
            background_color: DEFUALT_BUTTON_COLOR.into(),
            ..default()
        },
        button_type.clone(),
    )).with_children(|parent| {
        parent.spawn((
            TextBundle {
                text: Text::from_section(button_text, text_style),
                ..default()
            },
            if let ButtonType::SettingsButton(action) = button_type {
                SettingsText(true, Some(action))
            } else {
                SettingsText(false, None)
            }
        ));
    });
}
