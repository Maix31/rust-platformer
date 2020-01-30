use ggez::{
    Context,
    GameResult,
};

use crate::global_state::GlobalState;

#[allow(dead_code)]
pub enum SceneSwitch {
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
}

pub struct SceneStack {
    scenes: Vec<Box<dyn Scene>>,
}

pub trait Scene {
    fn update(
        &mut self,
        state: &mut GlobalState,
        ctx: &mut Context,
    ) -> GameResult<Option<SceneSwitch>>;
    fn draw(&mut self, state: &mut GlobalState, ctx: &mut Context) -> GameResult<()>;
    fn draw_previous(&self) -> bool;
}

impl SceneStack {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        Self { scenes: Vec::new() }
    }

    pub fn push(&mut self, scene: Box<dyn Scene>) {
        self.scenes.push(scene)
    }

    /// Remove the top scene from the stack and returns it;
    /// panics if there is none.
    pub fn pop(&mut self) -> Box<dyn Scene> {
        self.scenes
            .pop()
            .expect("ERROR: Popped an empty scene stack.")
    }

    /// Returns the current scene; panics if there is none.
    pub fn current(&self) -> &dyn Scene {
        &**self
            .scenes
            .last()
            .expect("ERROR: Tried to get current scene of an empty scene stack.")
    }

    /// Executes the given SceneSwitch command; if it is a pop or replace
    /// it returns `Some(old_scene)`, otherwise `None`
    pub fn switch(&mut self, next_scene: SceneSwitch) -> Option<Box<dyn Scene>> {
        match next_scene {
            SceneSwitch::Pop => {
                let s = self.pop();
                Some(s)
            }
            SceneSwitch::Push(s) => {
                self.push(s);
                None
            }
            SceneSwitch::Replace(s) => {
                let old_scene = self.pop();
                self.push(s);
                Some(old_scene)
            }
        }
    }

    /// The update function must be on the SceneStack because otherwise
    /// if you try to get the current scene and the world to call
    /// update() on the current scene it causes a double-borrow.  :/
    pub fn update(&mut self, state: &mut GlobalState, ctx: &mut ggez::Context) -> GameResult<()> {
        let current_scene = &mut **self
            .scenes
            .last_mut()
            .expect("Tried to update empty scene stack");
        let switch = current_scene.update(state, ctx)?;

        if let Some(switch) = switch {
            self.switch(switch);
        }

        Ok(())
    }

    /// We walk down the scene stack until we find a scene where we aren't
    /// supposed to draw the previous one, then draw them from the bottom up.
    ///
    /// This allows for layering GUI's and such.
    fn draw_scenes(
        scenes: &mut [Box<dyn Scene>],
        world: &mut GlobalState,
        ctx: &mut ggez::Context,
    ) -> GameResult<()> {
        assert!(scenes.len() > 0);
        if let Some((current, rest)) = scenes.split_last_mut() {
            if current.draw_previous() {
                SceneStack::draw_scenes(rest, world, ctx)?;
            }
            current.draw(world, ctx)
        } else {
            Ok(())
        }
    }

    /// Draw the current scene.
    pub fn draw(&mut self, state: &mut GlobalState, ctx: &mut ggez::Context) -> GameResult<()> {
        SceneStack::draw_scenes(&mut self.scenes, state, ctx)
    }

    pub fn len(&self) -> usize {
        self.scenes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.is_empty()
    }
}