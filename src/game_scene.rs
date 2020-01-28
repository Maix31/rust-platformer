use ggez::{
    Context,
    GameResult,
    input::keyboard::KeyCode,
};

use crate::global_state::GlobalState;
use crate::scene::{SceneSwitch, Scene};

pub struct GameScene {
    
}

impl GameScene {
    pub fn new(_ctx: &mut Context) -> Box<GameScene> {
        Box::new(GameScene {

        })
    }
}

impl Scene for GameScene {
    fn update(
        &mut self,
        state: &mut GlobalState,
        ctx: &mut Context,
    ) -> GameResult<Option<SceneSwitch>> {
        if state.input.key_pressed(ctx, KeyCode::Escape) {
            Ok(Some(SceneSwitch::Pop))
        } else {
            Ok(None)
        }
    }
    
    fn draw(&mut self, _state: &mut GlobalState, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::clear(ctx, [0.0,0.0,0.0,0.0].into());
        Ok(())
    }

    fn draw_previous(&self) -> bool {
        false
    }
}


