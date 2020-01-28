use crate::assets;
use crate::input;

use ggez::GameResult;
use ggez::Context;

pub struct GlobalState {
    pub assets: assets::Assets,
    pub input: input::Input,
}

impl GlobalState {
    pub fn new(ctx: &mut Context) -> GameResult<GlobalState> {
        let assets = assets::Assets::load(ctx)?;
        let input = input::Input::new();

        Ok(GlobalState {
            assets,
            input,
        })
    }
}