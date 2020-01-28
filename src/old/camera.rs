use ggez::graphics;
use ggez::nalgebra as na;

use crate::player::Player;

#[derive(Clone,Copy)]
pub struct Camera {
    pub rect: graphics::Rect,
}

impl Camera {
    pub fn update(&mut self, player: &Player) {

        self.rect.move_to([
            na::clamp(
                player.rect.x - (crate::CAMERA_DIMENSIONS.0 as f32 - player.rect.w) / 2.0, 
                    0.0 , 
                    crate::GAME_WIDTH as f32  - crate::CAMERA_DIMENSIONS.0 as f32
                ),
            na::clamp(
                player.rect.y - (crate::CAMERA_DIMENSIONS.1 as f32 - player.rect.h) / 2.0, 
                    0.0 , 
                    crate::GAME_HEIGHT as f32 - crate::CAMERA_DIMENSIONS.1 as f32
            )
        ]);
    }
}

impl From<Camera> for graphics::DrawParam {
    fn from(camera: Camera) -> Self {
        graphics::DrawParam {
            offset: [camera.rect.x, camera.rect.y].into(),
            dest:   [-camera.rect.x, -camera.rect.y].into(),
            scale:  [(crate::SCREEN_WIDTH / camera.rect.w as u32) as f32, (crate::SCREEN_HEIGHT / camera.rect.h as u32) as f32].into(),
            ..
            Default::default()
        }
    }
}