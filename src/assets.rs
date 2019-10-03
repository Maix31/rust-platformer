use ggez::{Context, GameResult};
use ggez::graphics;

use crate::gradient::{create_gradient, GradientDirection};

pub enum Texture {
    Air,
    Ground,
}

pub struct Assets {
    pub gradient: graphics::Image,
    pub ground:   graphics::Image,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> GameResult<Assets> {
        // normalize
        // norm [0.0 - 1.0] as f32
        let deep_sky_blue = graphics::Color::new(0.0, 191.0/255.0, 255.0/255.0, 1.0);
        let white = graphics::Color::new(0.0, 0.0, 0.0, 1.0);

        let colors = [
            deep_sky_blue,
            white,
        ];

        let gradient = create_gradient(ctx, &colors, GradientDirection::Vertical)?;
        let ground = graphics::Image::new(ctx, "/tile.png")?;

        Ok(Assets{
            gradient,
            ground,
        })
    }

    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        Self::load(ctx)
    }
} 