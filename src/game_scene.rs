use ggez::{
    Context,
    GameResult,
    nalgebra as na,
    graphics::{
        DrawParam,
        Rect,
        Image,
    },
    input::keyboard::KeyCode,
};

use crate::global_state::GlobalState;
use crate::scene::{SceneSwitch, Scene};

use crate::tilemap::TileMap;

struct Camera {
    rect: Rect,
    screen_info: (f32,f32)
}

impl Camera {
    fn new(rect: Rect) -> Camera {
        Camera {
            rect
            screen_info
        }
    }

    fn update(&mut self,state: &mut GlobalState, player: &Player) {

        let game_height = state.assets.settings.GAME_WIDTH;
        let game_width  = state.assets.settings.GAME_HEIGHT;

        self.rect.move_to([
            na::clamp(
                player.rect.x - (self.rect.w as f32 - player.rect.w) / 2.0, 
                    0.0 , 
                    game_width as f32  - self.rect.w as f32
                ),
            na::clamp(
                player.rect.y - (self.rect.h as f32 - player.rect.h) / 2.0, 
                    0.0 , 
                    game_height as f32 - self.rect.h as f32
            )
        ]);
    }
}


impl From<Camera> for graphics::DrawParam {
    fn from(camera: Camera) -> Self {
        DrawParam {
            offset: [camera.rect.x, camera.rect.y].into(),
            dest:   [-camera.rect.x, -camera.rect.y].into(),
            scale:  [(camera.screen_info.0 / camera.rect.w as u32) as f32, (self.screen_info.1 / camera.rect.h as u32) as f32].into(),
            ..
            Default::default()
        }
    }
}

pub struct GameScene {
    map: TileMap,
    background: Image,
    camera: Camera,
}

impl GameScene {
    pub fn new(state: &mut GlobalState, ctx: &mut Context) -> GameResult<Box<GameScene>> {
        
        let (w,h) = state.assets.settings.CAMERA_DIMENSIONS;
        Ok(Box::new(GameScene {
            map: TileMap::new(ctx, &std::path::Path::new("/tiled/world_1_1.tmx"))?,
            background: state.assets.gradient.clone(),
            camre: Camera::new(Rect::new_i32(0,0,w,h))
        }))
    }
}

impl Scene for GameScene {
    fn update(
        &mut self,
        state: &mut GlobalState,
        ctx: &mut Context,
    ) -> GameResult<Option<SceneSwitch>> {
        if state.input.key_released(KeyCode::Escape) {
            Ok(Some(SceneSwitch::Pop))
        } else {
            Ok(None)
        }
    }
    
    fn draw(&mut self, _state: &mut GlobalState, ctx: &mut Context) -> GameResult<()> {
    
        let (w,h) = (self.background.width(), self.background.height());
        let screen = ggez::graphics::screen_coordinates(ctx);
        let scale = [screen.w/w as f32, screen.h/h as f32];
        ggez::graphics::draw(ctx, &self.background, ggez::graphics::DrawParam::new().scale(scale))?;
        self.map.draw(ctx, ggez::graphics::DrawParam::new().scale([0.5,0.5])/*.dest([-100.0,-250.0])*/)
    
    }

    fn draw_previous(&self) -> bool {
        false
    }
}


