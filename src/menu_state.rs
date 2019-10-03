use crate::application::State;
use crate::controls::Controls;

use ggez::graphics::{Rect, Text};
use ggez::graphics::Font;
use ggez::GameResult;
use ggez::Context;

use ggez::nalgebra as na;

use std::time::Duration;


struct Button {
    rect: Rect,
    text: Text,
}

impl Button {
    fn new(string: String, rect: Rect, ctx: &mut Context, font: &Font) -> Button {
        Button {
            rect: rect,
            text: Text::new(ctx, string.as_str(), font).unwrap(),
        }
    }

    fn draw(&self, selected: bool, ctx: &mut Context) {
         if selected {
            ggez::graphics::rectangle(ctx, ggez::graphics::DrawMode::Fill, self.rect);
         } else {
            ggez::graphics::rectangle(ctx, ggez::graphics::DrawMode::Line(3.0), self.rect);
         }
         
         ggez::graphics::draw(ctx, &self.text, na::Point2::new(self.rect.x, self.rect.y), 0.0);
    }
}

pub struct MenuState {
    buttons: Vec<Button>,
    selected: usize,
    font: Font,
}

impl MenuState {
    pub fn new(ctx: &mut Context) -> MenuState {
        let width  = crate::SCREEN_WIDTH  as i32;
        let height = crate::SCREEN_HEIGHT as i32;
        let font = Font::new(ctx, "/font.ttf", 24).unwrap();
        let h = 50;
        let start = Button::new(String::from("start"), Rect::new_i32(width/3, height/3, width/3, h), ctx, &font);
        let setting = Button::new(String::from("settings"), Rect::new_i32(width/3, height/3+h, width/3, h), ctx, &font);
        let quit = Button::new(String::from("quit"), Rect::new_i32(width/3, height/3+h+h, width/3, h), ctx, &font);


        MenuState {
            buttons: vec![start, setting, quit],
            selected: 0,
            font: font,
        }
    }
}

impl State for MenuState {
    fn update(&mut self, input: Controls, dt: Duration) -> GameResult<()> {
        
        
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut count = 0;
        for button in &self.buttons {
            if self.selected == count {
                button.draw(true,ctx);
            } else {
                button.draw(false,ctx);
            }
            count += 1;
        }
        Ok(())
    }

    fn should_pop(&self) -> bool {
        unimplemented!()
    }

    fn push(&self) -> Option<Box<State>> {
        unimplemented!()
    }
}