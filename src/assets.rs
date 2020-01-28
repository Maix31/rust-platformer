use ggez::graphics;
use ggez::{Context, GameResult};
// use serde_yaml;

use crate::gradient::{create_gradient, GradientDirection};
use crate::settings::Settings;

pub struct Assets {
    pub gradient: graphics::Image,
    pub ground: graphics::Image,
    pub settings: Settings,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> GameResult<Assets> {
        // normalize
        // norm [0.0 - 1.0] as f32
        let deep_sky_blue = graphics::Color::new(109.0 / 255.0, 213.0 / 255.0, 237.0 / 255.0, 1.0);
        let piglet = graphics::Color::new(255.0 / 255.0, 231.0 / 255.0, 235.0 / 255.0, 1.0);

        let colors = [deep_sky_blue, piglet];

        let gradient = create_gradient(ctx, &colors, GradientDirection::Vertical)?;
        let ground = graphics::Image::new(ctx, "/tile.png")?;

        let settings = Settings::load(ctx)?;

        Ok(Assets {
            gradient,
            ground,
            settings,
        })
    }
    
    #[allow(dead_code)]
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        Self::load(ctx)
    }
}
