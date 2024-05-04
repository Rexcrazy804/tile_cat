use bevy::prelude::*;

#[derive(Resource)]
pub struct Controlls<T> {
    pub up: Option<T>,
    pub down: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,

    pub jump: Option<T>,
    pub fire: Option<T>,
    pub toggle_weapon: Option<T>,
    pub place_block: Option<T>,
}

impl Controlls<KeyCode> {
    fn new() -> Controlls<KeyCode> {
        Controlls {
            up: Some(KeyCode::W),
            down: Some(KeyCode::S),
            left: Some(KeyCode::A),
            right: Some(KeyCode::D),

            jump: Some(KeyCode::Space),
            fire: Some(KeyCode::J),
            toggle_weapon: Some(KeyCode::F),
            place_block: Some(KeyCode::ShiftLeft),
        }
    }
}

impl<T> Controlls<T> {
    fn empty() -> Self {
        Self {
            up: None,
            down: None,
            left: None,
            right: None,

            jump: None,
            fire: None,
            toggle_weapon: None,
            place_block: None,
        }
    }
}

impl Controlls<GamepadButton> {
    fn new(gamepad: Gamepad) -> Controlls<GamepadButton> {
        Controlls {
            up: Some(GamepadButton::new(gamepad, GamepadButtonType::DPadUp)),
            down: Some(GamepadButton::new(gamepad, GamepadButtonType::DPadDown)),
            left: Some(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)),
            right: Some(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)),

            jump: Some(GamepadButton::new(gamepad, GamepadButtonType::South)),
            fire: Some(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger2)),
            toggle_weapon: Some(GamepadButton::new(gamepad, GamepadButtonType::North)),
            place_block: Some(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger)),
        }
    }
}

pub struct ControllsPlugin;
impl Plugin for ControllsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Controlls::<KeyCode>::new())
            .insert_resource(Controlls::<GamepadButton>::empty())
            .add_systems(Update, update_gamepad.run_if(resource_changed::<Gamepads>()))
        ;
    }
}

pub fn update_gamepad(
    mut controller: ResMut<Controlls<GamepadButton>>,
    gamepads: Res<Gamepads>,
) {
    if let Some(gamepad) = gamepads.iter().next() {
        *controller = Controlls::<GamepadButton>::new(gamepad)
    }
}
