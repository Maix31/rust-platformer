use std::time::Duration;

use ggez::event;
use ggez::{Context, GameResult};

use ggez_goodies;

use crate::global_state;
use crate::input;

use crate::game_scene::GameScene;
use crate::menu_scene::MenuScene;

pub struct Application {
    global_state: global_state::GlobalState, 
    scenes: ggez_goodies::scene::SceneStack<global_state::GlobalState, input::Input>,
}

impl Application {
    pub fn new(ctx: &mut Context) -> GameResult<Application> {

        let global_state = global_state::GlobalState::new(ctx)?;
        let mut  scenes = ggez_goodies::scene::SceneStack::new(ctx, global_state);
        scenes.push(MenuScene::new(ctx));

        Ok(Application {
            global_state,
            scenes,
        })
    }
}

impl event::EventHandler for Application {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        let dt = ggez::timer::delta(ctx).as_millis() as f32 / 1000.0;

        self.global_state.input.state.update(dt);
        self.scenes.input(self.global_state.input, true);
        self.scenes.update(ctx);
        

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
        self.scenes.draw(ctx);
        ggez::graphics::present(ctx)
    }
}