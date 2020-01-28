use ggez::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Settings {
    CAMERA_DIMENSIONS: (i32, i32),
    SCREEN_WIDTH: u32,
    SCREEN_HEIGHT: u32,
    GAME_WIDTH: u32,
    GAME_HEIGHT: u32,
    PLAYER_DIMENSIONS: (f32, f32),
    PLAYER_VELOCITY: (f32, f32),
    PLAYER_VELOCITY_MAX: (f32, f32),
    JUMP_COOLDOWN: f32,
    IN_AIR_PENALTY_X: f32,
    GRAVITY_Y: f32,
    DRAG_COEFFICIENT: (f32, f32),
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
