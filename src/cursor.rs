use crate::{draw::Drawable, tick::Ticker};
use macroquad::prelude::*;

#[derive(Default)]
pub struct Cursor {
    pos: Vec2,
}

impl Cursor {
    pub fn handle_mouse(&mut self) {
        let mouse_pos = mouse_position();
        self.pos = vec2(mouse_pos.0, mouse_pos.1);
    }

    pub fn draw(&self) {
        draw_triangle(
            self.pos,
            self.pos + vec2(10.0, 14.0),
            self.pos + vec2(-2.0, 16.0),
            RED,
        );
    }
}

impl Drawable for Cursor {
    fn draw(&self) {
        self.draw();
    }
}

impl Ticker for Cursor {
    fn tick(&mut self) {
        self.handle_mouse();
    }
}
