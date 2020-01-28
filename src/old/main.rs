use ggez::event;
use ggez::GameResult;

use std::env;
use std::path;

    const CAMERA_DIMENSIONS : (i32, i32) = (16_i32, 9_i32);
    const SCREEN_WIDTH  : u32 = 1024;
    const SCREEN_HEIGHT : u32 =  576;
    const GAME_WIDTH  : u32 = 32;
    const GAME_HEIGHT : u32 =  9;
    const PLAYER_DIMENSIONS : (f32, f32) = (0.7, 0.9);
    const PLAYER_VELOCITY : (f32, f32) = (0.07, 0.5);
    const PLAYER_VELOCITY_MAX : (f32, f32) = (3.0, 4.0);
    const JUMP_COOLDOWN : f32 = 0.;
    const IN_AIR_PENALTY_X : f32 = 0.1;
    const GRAVITY_Y : f32 = 0.03;
    const DRAG_COEFFICIENT : (f32, f32) = (0.6, 0.95);

mod application;
mod assets;
mod block;
mod camera;
mod collider;
mod gradient;
mod game_scene;
mod global_state;
mod menu_scene;
mod player;
mod tilemap;
mod settings;
mod imgui_wrapper;
mod input;

use application::Application;

pub fn main() -> GameResult<()> {

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("super_simple", "ggez")
                .add_resource_path(resource_dir)
                .window_mode(ggez::conf::WindowMode{
                    width: SCREEN_WIDTH as f32,
                    height: SCREEN_HEIGHT as f32,
                    ..
                    Default::default()
                })
                .build()?;
    let state = &mut Application::new(ctx)?;
    event::run(ctx, event_loop, state)
}