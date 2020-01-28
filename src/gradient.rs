use ggez::graphics;
use ggez::{Context, GameResult};

#[allow(dead_code)]
pub enum GradientDirection {
    Horizontal,
    Vertical,
}

pub fn create_gradient(
    ctx: &mut Context,
    colors: &[graphics::Color],
    direction: GradientDirection,
) -> GameResult<graphics::Image> {
    let buf_size = colors.len();
    let mut buffer: Vec<u8> = Vec::with_capacity(buf_size);
    for color in colors {
        //Hack because no color to iter
        let (r, g, b, a) = color.to_rgba();
        buffer.push(r);
        buffer.push(g);
        buffer.push(b);
        buffer.push(a);
    }
    let mut result;
    match direction {
        GradientDirection::Horizontal => {
            result = graphics::Image::from_rgba8(ctx, buf_size as u16, 1, &buffer)?;
            result.set_filter(graphics::FilterMode::Linear);
        }
        GradientDirection::Vertical => {
            result = graphics::Image::from_rgba8(ctx, 1, buf_size as u16, &buffer)?;
            result.set_filter(graphics::FilterMode::Linear);
        }
    }
    Ok(result)
}
