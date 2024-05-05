use std::hash::Hash;

use bevy::input::gamepad::GamepadButtonInput;
use bevy::input::keyboard::KeyboardInput;
use bevy::{log, prelude::*};

#[derive(Resource)]
pub struct CurrentGamepad(pub Option<Gamepad>);

#[derive(Resource)]
pub struct ControllChange(pub CatAction);

#[derive(Resource)]
pub struct Controlls<T> {
    pub up: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,

    pub jump: Option<T>,
    pub fire: Option<T>,
    pub toggle_weapon: Option<T>,
    pub place_block: Option<T>,
    pub pause: Option<T>,
}

impl<T> Controlls<T> {
    fn empty() -> Self {
        Self {
            up: None,
            left: None,
            right: None,

            jump: None,
            fire: None,
            toggle_weapon: None,
            place_block: None,
            pause: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CatAction {
    Up,
    Left,
    Right,

    Jump,
    Fire,
    ToggleWeapon,
    PlaceBlock,
    Pause,
}

pub const ACTION_LIST: [CatAction; 8] = [
    CatAction::Up,
    CatAction::Left,
    CatAction::Right,
    CatAction::Jump,
    CatAction::Fire,
    CatAction::ToggleWeapon,
    CatAction::PlaceBlock,
    CatAction::Pause,
];

pub struct ControllsPlugin;
impl Plugin for ControllsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Controlls::<KeyCode>::empty())
            .insert_resource(Controlls::<GamepadButton>::empty())
            .insert_resource(Controlls::<MouseButton>::empty())
            .insert_resource(CurrentGamepad(None))
            .add_systems(Startup, (initialize_mouse_buttons, initialize_kbd_buttons))
            .add_systems(
                Update,
                (
                    initialize_gamepad.run_if(resource_changed::<Gamepads>()),
                    handle_controll_change.run_if(resource_exists::<ControllChange>()),
                )
            );
    }
}

pub fn initialize_gamepad(
    mut controller: ResMut<Controlls<GamepadButton>>,
    mut current: ResMut<CurrentGamepad>,
    gamepads: Res<Gamepads>,
) {
    if let Some(gamepad) = gamepads.iter().next() {
        current.0 = Some(gamepad); // required for axis controlls :/

        controller.up = Some(GamepadButton::new(gamepad, GamepadButtonType::DPadUp));
        controller.left = Some(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft));
        controller.right = Some(GamepadButton::new(gamepad, GamepadButtonType::DPadRight));
        controller.jump = Some(GamepadButton::new(gamepad, GamepadButtonType::South));
        controller.fire = Some(GamepadButton::new(
            gamepad,
            GamepadButtonType::RightTrigger2,
        ));
        controller.toggle_weapon = Some(GamepadButton::new(gamepad, GamepadButtonType::North));
        controller.place_block = Some(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger));
        controller.pause = Some(GamepadButton::new(gamepad, GamepadButtonType::Start));
    }
}

fn initialize_mouse_buttons(mut controller: ResMut<Controlls<MouseButton>>) {
    controller.fire = Some(MouseButton::Left);
    controller.toggle_weapon = Some(MouseButton::Right);
}

fn initialize_kbd_buttons(mut controller: ResMut<Controlls<KeyCode>>) {
    controller.up = Some(KeyCode::W);
    controller.left = Some(KeyCode::A);
    controller.right = Some(KeyCode::D);

    controller.jump = Some(KeyCode::Space);
    controller.fire = Some(KeyCode::J);
    controller.toggle_weapon = Some(KeyCode::F);
    controller.place_block = Some(KeyCode::ShiftLeft);
    controller.pause = Some(KeyCode::Escape);
}

fn handle_controll_change(
    mut commands: Commands,
    mut kbd_events: EventReader<KeyboardInput>,
    mut gamepad_events: EventReader<GamepadButtonInput>,
    mut kbd_controller: ResMut<Controlls<KeyCode>>, // will needa change this in the future
    mut gamepad_controller: ResMut<Controlls<GamepadButton>>, // will needa change this in the future
    controllchange: Res<ControllChange>,
) {
    if let Some(key_event) = kbd_events.read().next() {
        update_button(&mut kbd_controller, controllchange.0, key_event.key_code);
        commands.remove_resource::<ControllChange>();
    } else if let Some(button_event) = gamepad_events.read().next() {
        update_button(&mut gamepad_controller, controllchange.0, Some(button_event.button));
        commands.remove_resource::<ControllChange>();
    }
}

pub fn update_button<T: Copy + Eq + Hash + Send + Sync + 'static>(
    controller: &mut ResMut<Controlls<T>>,
    action: CatAction,
    button: Option<T>,
) {
    match action {
        CatAction::Up => controller.up = button,
        CatAction::Left => controller.left = button, 
        CatAction::Right => controller.right = button, 
        CatAction::Jump => controller.jump = button, 
        CatAction::Fire => controller.fire = button, 
        CatAction::ToggleWeapon => controller.toggle_weapon = button, 
        CatAction::PlaceBlock => controller.place_block = button, 
        CatAction::Pause => controller.pause = button, 
    }
}
