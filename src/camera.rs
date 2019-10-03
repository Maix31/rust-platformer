use ggez::graphics;
use ggez::nalgebra as na;

use crate::player::Player;

#[derive(Clone,Copy)]
pub struct Camera {
    pub rect: graphics::Rect,
}

impl Camera {
    pub fn update(&mut self, player: &Player) {
        
        let mut temp = self.rect;

        temp.x = player.rect.x - (crate::CAMERA_DIMENSIONS.0 as f32 - player.rect.w) / 2.0;
        temp.y = player.rect.y - (crate::CAMERA_DIMENSIONS.1 as f32 - player.rect.h) / 2.0;
    
        if temp.x < 0.0 { temp.x = 0.0;};
        if (temp.x + crate::CAMERA_DIMENSIONS.0 as f32) > crate::GAME_WIDTH  as f32 {
            temp.x = crate::GAME_WIDTH  as f32 - crate::CAMERA_DIMENSIONS.0 as f32;
        };

        if temp.y < 0.0 { temp.y = 0.0;};
        if (temp.y + crate::CAMERA_DIMENSIONS.1 as f32) > crate::GAME_HEIGHT as f32 {
            temp.y = crate::GAME_HEIGHT as f32 - crate::CAMERA_DIMENSIONS.1 as f32;
        };

        self.rect = temp;
    }
}

impl From<Camera> for graphics::DrawParam {
    fn from(camera: Camera) -> Self {
        graphics::DrawParam {
            offset: na::Point2::new(camera.rect.x, camera.rect.y),
            dest:  -na::Point2::new(camera.rect.x, camera.rect.y),
            scale:  na::Point2::new((crate::SCREEN_WIDTH / camera.rect.w as u32) as f32, (crate::SCREEN_HEIGHT / camera.rect.h as u32) as f32),
            ..
            Default::default()
        }
    }
}