mod application;
mod assets;
mod game_scene;
mod global_state;
mod gradient;
mod imgui_wrapper;
mod input;
mod menu_scene;
mod scene;
mod settings;
mod tilemap;

// #[macro_use] extern crate text_io;

// enum Button {}

// struct Player {}

// struct TileMap {}

fn main() -> ggez::GameResult<()> {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Super Mario Bros.", "Maix")
        .add_resource_path("./resources")
        .add_resource_path("./resources/tiled")
        .build()?;

    let app = &mut application::Application::new(ctx)?;

    ggez::event::run(ctx, event_loop, app)
}
