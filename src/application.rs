use ggez::{
    Context,
    event::EventHandler,
    GameResult,
    input::{
        keyboard::{
            KeyCode,
            KeyMods,
        },
        mouse::MouseButton,
    }
};

use crate::global_state::GlobalState;
use crate::scene::SceneStack;
use crate::menu_scene::MenuScene;

pub struct Application {
    global_state: GlobalState,
    scenes: SceneStack,
    debug: bool,
}

impl Application {
    pub fn new(ctx: &mut Context) -> GameResult<Application> {
        let global_state = GlobalState::new(ctx)?;
        let mut scenes = SceneStack::new(ctx);
        scenes.push(MenuScene::new(ctx)?);

        let debug = false;

        Ok(Application {
            global_state,
            scenes,
            debug,
        })
    }
}

impl EventHandler for Application {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.scenes.update(&mut self.global_state, ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ggez::graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());

        let frames = ggez::timer::fps(ctx);
        let frames = ggez::graphics::Text::new(format!("FPS: {:.2}", frames));

        self.scenes.draw(&mut self.global_state, ctx)?;

        if self.debug {
            ggez::graphics::draw(
                ctx,
                &frames,
                ggez::graphics::DrawParam::default().scale([2.0, 2.0]),
            )?;
        }

        ggez::graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        let last_state = self
            .global_state
            .input
            .mouse
            .button
            .entry(button)
            .or_insert((false, false))
            .0;
        *self
            .global_state
            .input
            .mouse
            .button
            .entry(button)
            .or_insert((false, false)) = (true, last_state);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        let last_state = self
            .global_state
            .input
            .mouse
            .button
            .entry(button)
            .or_insert((true, true))
            .0;
        *self
            .global_state
            .input
            .mouse
            .button
            .entry(button)
            .or_insert((true, true)) = (false, last_state);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.global_state.input.mouse.pos.0 = x;
        self.global_state.input.mouse.pos.1 = y;
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, _y: f32) {
        self.global_state.input.mouse.wheel = x;
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        self.debug = true;
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        self.debug = false;
    }

    // fn text_input_event(&mut self, _ctx: &mut Context, _character: char)

    // fn gamepad_button_down_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _btn: Button,
    //     _id: GamepadId
    // )

    // fn gamepad_button_up_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _btn: Button,
    //     _id: GamepadId
    // )

    // fn gamepad_axis_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _axis: Axis,
    //     _value: f32,
    //     _id: GamepadId
    // )

    // fn focus_event(&mut self, _ctx: &mut Context, _gained: bool)
    // fn quit_event(&mut self, _ctx: &mut Context) -> bool
    // fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32)
}