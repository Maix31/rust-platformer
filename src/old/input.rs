use ggez_goodies::input;
use ggez::event::KeyCode;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Axis {}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Button {
    A,
    B,
    Up,
    Left,
    Right,
    Down,
    Select,
    Start,
    Exit
}
pub struct Input {
    pub bindings: input::InputBinding<Axis, Button>,
    pub state: input::InputState<Axis, Button>,
}

impl Input  {
    pub fn new() -> Input {
        let bindings = input::InputBinding::new()
            .bind_key_to_button(KeyCode::Period, Button::A)
            .bind_key_to_button(KeyCode::Slash,  Button::B)
            .bind_key_to_button(KeyCode::A,      Button::Left)
            .bind_key_to_button(KeyCode::W,      Button::Up)
            .bind_key_to_button(KeyCode::S,      Button::Down)
            .bind_key_to_button(KeyCode::D,      Button::Right)
            .bind_key_to_button(KeyCode::Escape, Button::Exit)
            .bind_key_to_button(KeyCode::Return, Button::Start) //Enter Key
            .bind_key_to_button(KeyCode::Space,  Button::Select);
        
        let state = input::InputState::new();

        Input {
            bindings,
            state,
        }
    }
}