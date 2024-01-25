use bevy::{app::AppExit, prelude::*};
use super::GameState;

const DEFUALT_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVER_BUTTON_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component)]
pub enum ButtonType {
    PlayButton,
    QuitButton,
}

pub fn button_interactions(
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<(&Interaction, &ButtonType, &mut BackgroundColor), Changed<Interaction>>,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    for (&interaction, button_type, background) in &mut query {
        handle_background(interaction, background);
        if interaction == Interaction::Pressed {
            match *button_type {
                ButtonType::PlayButton => next_state.set(GameState::Game),
                ButtonType::QuitButton => exit_event_writer.send(AppExit),
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

pub fn attach_button(
    parent: &mut ChildBuilder,
    button_type: ButtonType,
    button_text: &str,
) {
    let button_style = Style {
        width: Val::Percent(10.0),
        height: Val::Percent(5.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let text_style = TextStyle {
        font_size: 14.0,
        color: Color::WHITE,
        ..default()
    };

    let button = ButtonBundle {
        style: button_style,
        background_color: DEFUALT_BUTTON_COLOR.into(),
        ..default()
    };

    let text = TextBundle {
        text: Text::from_section(
            button_text, 
            text_style
        ),
        ..default()
    };

    parent.spawn((
        button,
        button_type,
    ))
        .with_children(|parent| {
            parent.spawn(text);
        })
    ;
}

