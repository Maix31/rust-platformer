use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra as na;

pub enum PlayerMove {
    Jump,
    Left,
    Right,
}

pub enum PlayerState {
    Idle,
    Running,
    Jumping,
}

pub enum PlayerDirection {
    Left,
    Right,
}

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

    pub fn direction(&self) -> PlayerDirection {
        match self.velocity.x.is_sign_negative() {
            true  => PlayerDirection::Left,
            false => PlayerDirection::Right,
        }
    }

    pub fn update(&mut self, _dt: f32) {

        self.rect_old = self.rect;

        self.rect.x += self.velocity.x;
        self.rect.y += self.velocity.y;
    }

    pub fn draw(&self, ctx: &mut Context, camera: graphics::DrawParam) -> GameResult<()> {
        
        let points = [
            na::Point2::new(self.rect.x              , self.rect.y),
            na::Point2::new(self.rect.x + self.rect.w, self.rect.y),
            na::Point2::new(self.rect.x + self.rect.w, self.rect.y + self.rect.h),
            na::Point2::new(self.rect.x              , self.rect.y + self.rect.h),
        ];

        let rect = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::Fill,
            &points,
        )?;

        let p_center = na::Point2::new(self.rect.x + self.rect.w / 2.0 , self.rect.y + self.rect.h / 2.0);
        let x_axis = [p_center, p_center + na::Vector2::new(self.velocity.x, 0.0)];
        let y_axis = [p_center, p_center + na::Vector2::new(0.0, self.velocity.y)];
        let x_axis = graphics::Mesh::new_line(ctx, &x_axis, 0.2)?;
        let y_axis = graphics::Mesh::new_line(ctx, &y_axis, 0.2)?;
        
        graphics::draw_ex(ctx, &x_axis, camera)?;
        graphics::draw_ex(ctx, &y_axis, camera)?;
        graphics::draw_ex(ctx, &rect, camera)
    }
}