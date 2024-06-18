use macroquad::prelude::*;

/// The shot made by a player or an enemy
#[derive(Debug, Default)]
pub struct Bullet {
    /// The position of the bullet
    pos: Vec2,
    /// The bullet travels at constant speed
    speed: f32,
    /// The damage the bullet does
    damage: u32,
    /// Source of the bullet
    source: Vec2,
}

impl Bullet {
    pub fn new(pos: Vec2, speed: f32, damage: u32, source: Vec2) -> Self {
        Self {
            pos,
            speed,
            damage,
            source,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 4.0, GREEN);
    }

    /// Every tick the bullet moves in the direction it was shot from source by speed
    pub fn tick(&mut self) {
        self.pos += self.source.normalize() * self.speed;
    }

    /// Spawns a bullet to the mouse position from source
    pub fn shoot(source: Vec2) -> Self {
        let mouse = mouse_position();
        let mouse = vec2(mouse.0, mouse.1);
        Self {
            pos: (source - mouse).normalize(),
            speed: 10.0,
            damage: 10,
            source,
        }
    }
}
