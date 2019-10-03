use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use std::time::Duration;

use crate::application::State;
use crate::assets::Assets;
use crate::camera::Camera;
use crate::collider;
use crate::controls::Controls;
use crate::player::Player;
use crate::tilemap::TileMap;

pub struct MainState {
    assets: Assets,
    camera: Camera,
    player: Player,
    tilemap: TileMap,
    should_pop: bool,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {

        let assets = Assets::load(ctx)?;
        let spritebatch = graphics::spritebatch::SpriteBatch::new(assets.ground.clone());
        
        let s = MainState{
            assets: assets,
            camera: Camera{rect: graphics::Rect::new_i32(0, 0, crate::CAMERA_DIMENSIONS.0, crate::CAMERA_DIMENSIONS.1)},
            player: Player::new(0.0, 0.0),
            tilemap: TileMap::load_test(spritebatch),
            should_pop: false,
        };
        Ok(s)
    }
}

impl State for MainState {

    fn update(&mut self, input: Controls, dt: Duration) -> GameResult<()> {
             
            let dt = dt.as_millis() as f32 / 1000.0;
            
        // input
            match (input.right, input.left) {
                (true,false) => {self.player.move_right()},
                (false,true) => {self.player.move_left() },
                _ => (),
            };

            if input.space || input.up {
                self.player.jump();
            };

            if input.escape {
                self.should_pop = true;
            };
            
        // update
            self.player.velocity.y += crate::GRAVITY_Y;
            self.player.update(dt);

            self.player.velocity.x *= crate::DRAG_COEFFICIENT.0;

            // if self.player.velocity.y.is_sign_positive() {self.player.velocity.y *= crate::DRAG_COEFFICIENT.1;}
            self.player.velocity.y *= crate::DRAG_COEFFICIENT.1;

            collider::collide(&mut self.player, &self.tilemap);

            self.camera.update(&self.player);

            Ok(())
        }

        fn draw(&self, ctx: &mut Context) -> GameResult<()> {

            //Stretch to cover whole screen
            let background_param = graphics::DrawParam{
                    scale: na::Point2::new(crate::SCREEN_WIDTH  as f32 / self.assets.gradient.width()  as f32 ,
                                           crate::SCREEN_HEIGHT as f32 / self.assets.gradient.height() as f32 ),
                    ..
                    Default::default()
            };

            graphics::draw_ex(ctx, &self.assets.gradient,   background_param)?;
            ggez::graphics::set_blend_mode(ctx, ggez::graphics::BlendMode::Alpha)?;
            self.tilemap.draw(ctx, self.camera.into())?;
            self.player.draw(ctx, self.camera.into())?;

            println!("{}, {}", self.player.rect.x, self.player.rect.y);


            Ok(())
        }

        fn should_pop(&self) -> bool {
            unimplemented!()
        }

        fn push(&self) -> Option<Box<State>> {
            None
        }
}