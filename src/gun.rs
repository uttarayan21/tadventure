use macroquad::prelude::*;

use crate::movement::*;

/// The shot made by a player or an enemy
#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    movement: Movement,
    /// The damage the bullet does
    damage: u32,
}

impl Bullet {
    pub fn draw(&self) {
        draw_circle(self.movement.pos.x, self.movement.pos.y, 4.0, GREEN);
    }
}

pub struct Gun {
    pub direction: Vec2,
    pub bullets: Vec<Bullet>,
}

impl Gun {
    pub fn shoot(&mut self, pos: Vec2) {
        let bullet = Bullet {
            movement: Movement {
                pos,
                direction: self.direction,
                type_: MovementType::Speed(5.0),
            },
            damage: 10,
        };
        self.bullets.push(bullet);
    }
}
