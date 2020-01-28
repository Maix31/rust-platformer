use std::time::Duration;

use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::nalgebra as na;

use ggez::event::{
    KeyCode,
    KeyMods
};

use std::path::Path;

use crate::application;
use crate::assets::Assets;
use crate::camera::Camera;
use crate::collider;
use crate::player::Player;
use crate::global_state::GlobalState;
use crate::input::Input;

use ggez_goodies::tilemap;
use ggez_goodies::scene::Scene;
use ggez_goodies::scene::SceneSwitch;

use crate::input::Button;

pub struct GameScene {
    camera: Camera,
    player: Player,
    // tilemap: tilemap::Map,
}

impl GameScene {
    pub fn new(ctx: &mut Context) -> GameResult<GameScene> {
        Ok(GameScene{
            camera: Camera{rect: graphics::Rect::new_i32(0, 0, crate::CAMERA_DIMENSIONS.0, crate::CAMERA_DIMENSIONS.1)},
            player: Player::new(0.0, 0.0),
            // tilemap: tiled::parse_file(Path::new("C:/Users/darkm/Documents/Rust/rust-platformer/tiled/world_1_1.tmx")).expect("Coudn't parse world 1"),
        })
    }
}

impl Scene<GlobalState, Input> for GameScene {

    fn update(&mut self, gameworld: &mut GlobalState, ctx: &mut Context) -> SceneSwitch<GlobalState, Input> {
             
        //     let dt = ggez::timer::delta(ctx).as_millis() as f32 / 1000.0;
        //     let input = gameworld.input.state;
        //     let Left = input.get_button_down(Button::A);
        // // input
        //     match (self.input.right, self.input.left) {
        //         (true,false) => {self.player.move_right()},
        //         (false,true) => {self.player.move_left() },
        //         _ => (),
        //     };

        //     if self.input.space || self.input.up {
        //         self.player.jump();
        //     };

        //     if self.input.escape {
        //         self.should_pop = true;
        //     };
            
        // // update
        //     self.player.velocity.y += crate::GRAVITY_Y;
        //     self.player.update(dt);

        //     self.player.velocity.x *= crate::DRAG_COEFFICIENT.0;

        //     // if self.player.velocity.y.is_sign_positive() {self.player.velocity.y *= crate::DRAG_COEFFICIENT.1;}
        //     self.player.velocity.y *= crate::DRAG_COEFFICIENT.1;

        //     collider::collide(&mut self.player, &self.tilemap);

        //     self.camera.update(&self.player);

        //     Ok(())
            unimplemented!()
        }

        fn draw(&mut self, gameworld: &mut GlobalState, ctx: &mut Context) -> GameResult<()> {

            // //Stretch to cover whole screen
            // let background_param = graphics::DrawParam{
            //         scale: [crate::SCREEN_WIDTH  as f32 / self.assets.gradient.width()  as f32,
            //                 crate::SCREEN_HEIGHT as f32 / self.assets.gradient.height() as f32 ].into(),
            //         ..
            //         Default::default()
            // };

            // graphics::draw(ctx, &self.assets.gradient,   background_param)?;
            // self.tilemap.draw(ctx, self.camera.into())?;
            // // self.player.draw(ctx, self.camera.into())?;
            // self.player.draw(ctx, ggez::graphics::DrawParam::default())?;


            // let text = ggez::graphics::Text::new(format!("FPS: {}", ggez::timer::fps(ctx)));
            // ggez::graphics::draw(ctx, &text, ggez::graphics::DrawParam::default())
        
            unimplemented!()
        }

        fn input(&mut self, gameworld: &mut GlobalState, event: Input, started: bool) {
            unimplemented!()
        }
        fn name(&self) -> &str {
            unimplemented!()
        }

    // fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
    //     match keycode {
    //         KeyCode::Up     => { self.input.up     = true;}
    //         KeyCode::Down   => { self.input.down   = true;}
    //         KeyCode::Left   => { self.input.left   = true;}
    //         KeyCode::Right  => { self.input.right  = true;}
    //         KeyCode::Space  => { self.input.space  = true;}
    //         KeyCode::Escape => { self.input.escape = true;}
    //         _ => (),
    //     }
    // }

    // fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
    //     match keycode {
    //         KeyCode::Up     => { self.input.up     = false;}
    //         KeyCode::Down   => { self.input.down   = false;}
    //         KeyCode::Left   => { self.input.left   = false;}
    //         KeyCode::Right  => { self.input.right  = false;}
    //         KeyCode::Space  => { self.input.space  = false;}
    //         KeyCode::Escape => { self.input.escape = false;}
    //         _ => (),
    //     }
    // }
}