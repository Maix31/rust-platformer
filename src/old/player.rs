use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra as na;

pub struct Player {
    pub velocity: na::Vector2<f32>,
    pub rect: graphics::Rect,
    pub rect_old: graphics::Rect,
    pub jumping: bool,
}

impl Player {

    pub fn new(x: f32, y: f32) -> Player {
        Player {
            velocity: na::zero(),
            rect:     graphics::Rect::new(x, y , crate::PLAYER_DIMENSIONS.0, crate::PLAYER_DIMENSIONS.1),
            rect_old: graphics::Rect::new(x, y , crate::PLAYER_DIMENSIONS.0, crate::PLAYER_DIMENSIONS.1),
            jumping:  false,
        }
    }

    pub fn top(&self) -> f32 {
        self.rect.y
    }

    pub fn right(&self) -> f32 {
        self.rect.x + self.rect.w
    }

    pub fn bottom(&self) -> f32 {
        self.rect.y + self.rect.h
    }

    pub fn left(&self) -> f32 {
        self.rect.x
    }

    pub fn set_top(&mut self, top: f32) {
        self.rect.y = top;
    }

    pub fn set_right(&mut self, right: f32) {
        self.rect.x = right - self.rect.w;
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        self.rect.y = bottom - self.rect.h;
    }

    pub fn set_left(&mut self, left: f32) {
        self.rect.x = left;
    }

    pub fn top_left(&self) -> (f32, f32) {
        (self.left(), self.top())
    }

    pub fn top_right(&self) -> (f32, f32) {
        (self.right(), self.top())
    }

    pub fn bottom_right(&self) -> (f32, f32) {
        (self.right(), self.bottom())
    }

    pub fn bottom_left(&self) -> (f32, f32) {
        (self.left(), self.bottom())
    }

    pub fn move_left(&mut self) {
        self.velocity.x -= crate::PLAYER_VELOCITY.0;
    }

    pub fn move_right(&mut self) {
        self.velocity.x += crate::PLAYER_VELOCITY.0;
    }

    pub fn jump(&mut self) {
        if !self.jumping {
            self.jumping = true;
            self.velocity.y -= crate::PLAYER_VELOCITY.1;
        }
    }

    pub fn update(&mut self, dt: f32) {

        self.rect_old = self.rect;

        self.rect.x += self.velocity.x * dt;
        self.rect.y += self.velocity.y * dt;
    }

    pub fn draw(&self, ctx: &mut Context, camera: graphics::DrawParam) -> GameResult<()> {
        
        let rect = graphics::MeshBuilder::new().rectangle(ggez::graphics::DrawMode::fill(), self.rect, ggez::graphics::BLACK).build(ctx)?;
        graphics::draw(ctx, &rect, camera)
    }
}