use ggez::{
    Context,
    GameResult,
    input::keyboard::KeyCode,
};

use crate::global_state::GlobalState;
use crate::scene::{SceneSwitch, Scene};
use crate::imgui_wrapper;

pub struct MenuScene {
    imgui: imgui_wrapper::ImGuiWrapper,
}

impl MenuScene {
    pub fn new(ctx: &mut Context) -> GameResult<Box<MenuScene>> {
        Ok(Box::new(MenuScene {
            imgui: imgui_wrapper::ImGuiWrapper::new(ctx),
        }))
    }
}

impl Scene for MenuScene {
    fn update(
        &mut self,
        state: &mut GlobalState,
        ctx: &mut Context,
    ) -> GameResult<Option<SceneSwitch>> {
        if state.input.key_released(KeyCode::Escape) {
            return Ok(Some(SceneSwitch::Pop))
        }
        self.imgui.update(state, ctx)
    }

    fn draw(&mut self, state: &mut GlobalState, ctx: &mut Context) -> GameResult<()> {
        self.imgui.draw(state, ctx, 1.0);
        Ok(())
    }

    fn draw_previous(&self) -> bool {
        false
    }
}