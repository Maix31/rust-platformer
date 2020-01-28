use std::time::Duration;

use ggez::graphics::{Rect, Text};
use ggez::graphics::Font;
use ggez::GameResult;
use ggez::Context;

use ggez::nalgebra as na;

use ggez_goodies::scene::Scene;
use ggez_goodies::scene::SceneSwitch;

use crate::imgui_wrapper;
use crate::global_state::GlobalState;
use crate::input::Input;

pub struct MenuScene {
    imgui: imgui_wrapper::ImGuiWrapper,
    name: String,
}

impl MenuScene {
    pub fn new(ctx: &mut Context) -> Box<MenuScene> {
        let scene = MenuScene {
            imgui: imgui_wrapper::ImGuiWrapper::new(ctx),
            name: "Menu".to_owned()
        };
        Box::new(scene)
    }
}

impl Scene<GlobalState, Input> for MenuScene {
    fn update(&mut self, gameworld: &mut GlobalState, ctx: &mut Context) -> SceneSwitch<GlobalState, Input> {
        unimplemented!()
    }

    fn draw(&mut self, gameworld: &mut GlobalState, ctx: &mut Context) -> GameResult<()> {
        unimplemented!()
    }
    
    fn input(&mut self, gameworld: &mut GlobalState, event: Input, started: bool) {
        unimplemented!()
    }

    fn name(&self) -> &str {
        &self.name
    }
}