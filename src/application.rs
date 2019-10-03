use ggez::event;
use ggez::{Context, GameResult};
use ggez::event::{Keycode, Mod};

use std::time::Duration;

use crate::controls::Controls;
use crate::game_state::MainState;
use crate::menu_state::MenuState;

pub trait State {
    fn update(&mut self, input: Controls, dt: Duration) -> GameResult<()> ;
    fn draw(&self, ctx: &mut Context) -> GameResult<()> ;
    fn should_pop(&self) -> bool;
    fn push(&self) -> Option<Box<State>>;
}

pub struct Application {
    states: Vec<Box<State>>,
    pub input: Controls,
}

impl Application {
    pub fn new(ctx: &mut Context) -> GameResult<Application> {
        let app = Application {
            // states: vec![Box::new(MenuState::new(ctx))],
            states: vec![Box::new(MainState::new(ctx).unwrap())],
            input: Controls::default(),
        };

        Ok(app)
    }
}

impl event::EventHandler for Application {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.states.last_mut().unwrap().update(self.input, ggez::timer::get_delta(ctx))?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::clear(ctx);
        self.states.last().unwrap().draw(ctx)?;

        ggez::graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up     => { self.input.up     = true;}
            Keycode::Down   => { self.input.down   = true;}
            Keycode::Left   => { self.input.left   = true;}
            Keycode::Right  => { self.input.right  = true;}
            Keycode::Space  => { self.input.space  = true;}
            Keycode::Escape => { self.input.escape = true;}
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up     => { self.input.up     = false;}
            Keycode::Down   => { self.input.down   = false;}
            Keycode::Left   => { self.input.left   = false;}
            Keycode::Right  => { self.input.right  = false;}
            Keycode::Space  => { self.input.space  = false;}
            Keycode::Escape => { self.input.escape = false;}
            _ => (),
        }
    }
}