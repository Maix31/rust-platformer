use std::collections::HashMap;

use ggez::{
    Context,
    input::{
        keyboard::KeyCode,
        keyboard::KeyMods,
        mouse::MouseButton,
    }
};

#[derive(Default)]
pub struct Mouse {
    pub pos: (f32, f32),
    pub button: HashMap<MouseButton, (bool, bool)>,
    pub wheel: f32,
}

#[derive(Default)]
pub struct Keyboard {
    pub key_pressed:  HashMap<KeyCode, (KeyMods,bool)>,
    pub key_released: HashMap<KeyCode, KeyMods>,
}

pub struct Input {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse: Mouse::default(),
            keyboard: Keyboard::default(),
        }
    }

    pub fn update(&mut self) {
        self.keyboard.key_pressed.clear();
        self.keyboard.key_released.clear();
    }

    pub fn key_down(&self, key: KeyCode) -> bool {
        self.keyboard.key_pressed.get(&key).is_some()
    }
    
    pub fn key_up(&self, key: KeyCode) -> bool {
        !self.key_down(key)
    }

    pub fn key_repeat(&self, key: KeyCode) -> bool {
        if let Some(result) = self.keyboard.key_pressed.get(&key) {
            result.1
        } else {
            false
        }
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.key_pressed(key) && !self.key_repeat(key)
    }

    pub fn key_released(&self, key: KeyCode) -> bool {
        self.keyboard.key_released.get(&key).is_some()
    }

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse.button.get(&button).unwrap_or(&(false,false)).0
    }

    /// Coordinates (x,y)
    pub fn mouse_pos(&self) -> (f32,f32) {
        self.mouse.pos
    }

    pub fn wheel(&self) -> f32 {
        self.mouse.wheel
    }
}