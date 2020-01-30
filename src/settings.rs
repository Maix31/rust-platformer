use ggez::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub CAMERA_DIMENSIONS: (i32, i32),
    pub SCREEN_WIDTH: u32,
    pub SCREEN_HEIGHT: u32,
    pub GAME_WIDTH: u32,
    pub GAME_HEIGHT: u32,
    pub PLAYER_DIMENSIONS: (f32, f32),
    pub PLAYER_VELOCITY: (f32, f32),
    pub PLAYER_VELOCITY_MAX: (f32, f32),
    pub JUMP_COOLDOWN: f32,
    pub IN_AIR_PENALTY_X: f32,
    pub GRAVITY_Y: f32,
    pub DRAG_COEFFICIENT: (f32, f32),
}

impl Settings {
    pub fn load(ctx: &mut Context) -> ggez::GameResult<Settings> {
        let settings_file = ggez::filesystem::open(ctx, "/settings.yaml")?;
        match serde_yaml::from_reader(settings_file) {
            Ok(val) => Ok(val),
            Err(_) => Err(ggez::GameError::ConfigError(
                "Couldn't load settings.yaml".to_owned(),
            )),
        }
    }
}
