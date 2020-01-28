use std::collections::HashMap;

use ggez::{
    Context,
    input::{
        keyboard::KeyCode,
        mouse::MouseButton,
    }
};

#[derive(Default)]
pub struct Mouse {
    pub pos: (f32, f32),
    pub button: HashMap<MouseButton, (bool, bool)>,
    pub wheel: f32,
}

pub struct Input {
    pub mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse: Mouse::default(),
        }
    }

    pub fn key_pressed(&self, ctx: &mut Context, key: KeyCode) -> bool {
        let set = ggez::input::keyboard::pressed_keys(ctx);
        set.get(&key).is_some()
    }

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse.button.get(&button).is_some()
    }

    pub fn mouse_pos(&self) -> (f32,f32) {
        self.mouse.pos
    }

    pub fn wheel(&self) -> f32 {
        self.mouse.wheel
    }
}