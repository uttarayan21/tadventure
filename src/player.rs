use macroquad::prelude::*;

use crate::{draw::Drawable, gun::Gun, tick::Tick};

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub pointing: Vec2,
    pub direction: Vec2,
    pub velocity: Vec2,
    pub gun: Option<Gun>,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        let mouse_pos = mouse_position();
        let pointing = vec2(mouse_pos.0, mouse_pos.1);
        let direction = (pos - pointing).normalize();
        let gun = Some(Gun::new(pos, 10, direction, crate::gun::GunType::Pistol));
        Self {
            pos,
            pointing,
            direction,
            velocity: vec2(0.0, 0.0),
            gun,
        }
    }

    fn handle_inputs(&mut self) {
        let mouse_pos = mouse_position();
        self.pointing = vec2(mouse_pos.0, mouse_pos.1);
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 16.0, BLUE);
        self.gun.draw();
    }

    fn tick(&mut self) {
        self.pos += self.velocity;
        self.gun.as_mut().map(|gun| {
            gun.pos = self.pos;
            gun.shoot();
            gun.tick();
        });
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
        self.direction = (self.pointing - self.pos).normalize();
        let distance = self.pos.distance(self.pointing);
        self.velocity = self.direction * distance / 15f32;
        self.tick();
    }
}
