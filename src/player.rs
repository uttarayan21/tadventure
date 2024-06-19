use macroquad::prelude::*;

use crate::{draw::Drawable, tick::Tick};

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub pointing: Vec2,
    pub direction: Vec2,
    pub velocity: Vec2,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        let mouse_pos = mouse_position();
        let pointing = vec2(mouse_pos.0, mouse_pos.1);
        let direction = (pos - pointing).normalize();
        Self {
            pos,
            pointing,
            direction,
            velocity: vec2(0.0, 0.0),
        }
    }

    fn handle_inputs(&mut self) {
        let mouse_pos = mouse_position();
        self.pointing = vec2(mouse_pos.0, mouse_pos.1);
        self.direction = (self.pointing - self.pos).normalize();
        let distance = self.pos.distance(self.pointing);
        self.velocity = self.direction * distance / 15f32;
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 16.0, BLUE);
    }

    fn tick(&mut self) {
        self.pos += self.velocity;
    }
}

impl Drawable for Player {
    fn draw(&self) {
        self.draw();
    }
}

impl Tick for Player {
    fn tick(&mut self) {
        self.handle_inputs();
        self.tick();
    }
}
