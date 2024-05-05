use bevy::prelude::*;

#[derive(Resource)]
pub struct CurrentGamepad(pub Option<Gamepad>);

#[derive(Resource)]
pub struct Controlls<T> {
    pub left: Option<T>,
    pub right: Option<T>,

    pub jump: Option<T>,
    pub fire: Option<T>,
    pub toggle_weapon: Option<T>,
    pub place_block: Option<T>,
    pub pause: Option<T>,
}

impl Controlls<KeyCode> {
    fn new() -> Controlls<KeyCode> {
        Controlls {
            left: Some(KeyCode::A),
            right: Some(KeyCode::D),

            jump: Some(KeyCode::Space),
            fire: Some(KeyCode::J),
            toggle_weapon: Some(KeyCode::F),
            place_block: Some(KeyCode::ShiftLeft),
            pause: Some(KeyCode::Escape),
        }
    }
}

impl<T> Controlls<T> {
    fn empty() -> Self {
        Self {
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

impl Controlls<GamepadButton> {
    fn new(gamepad: Gamepad) -> Controlls<GamepadButton> {
        Controlls {
            left: Some(GamepadButton::new(gamepad, GamepadButtonType::DPadLeft)),
            right: Some(GamepadButton::new(gamepad, GamepadButtonType::DPadRight)),

            jump: Some(GamepadButton::new(gamepad, GamepadButtonType::South)),
            fire: Some(GamepadButton::new(
                gamepad,
                GamepadButtonType::RightTrigger2,
            )),
            toggle_weapon: Some(GamepadButton::new(gamepad, GamepadButtonType::North)),
            place_block: Some(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger)),
            pause: Some(GamepadButton::new(gamepad, GamepadButtonType::Start)),
        }
    }
}

pub struct ControllsPlugin;
impl Plugin for ControllsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Controlls::<KeyCode>::new())
            .insert_resource(Controlls::<GamepadButton>::empty())
            .insert_resource(CurrentGamepad(None))
            .add_systems(
                Update,
                update_gamepad.run_if(resource_changed::<Gamepads>()),
            );
    }
}

pub fn update_gamepad(
    mut controller: ResMut<Controlls<GamepadButton>>,
    mut current: ResMut<CurrentGamepad>,
    gamepads: Res<Gamepads>,
) {
    if let Some(gamepad) = gamepads.iter().next() {
        *controller = Controlls::<GamepadButton>::new(gamepad);
        current.0 = Some(gamepad);
    }
}
