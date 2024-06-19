use macroquad::prelude::*;

use crate::draw::Drawable;
/// The basic enemy data
/// This condaions just he enemy position
/// and the enemy class
/// The enemy just moves towards the player constantly if there's nothing in the way.
#[derive(Debug, Default)]
pub struct Enemy {
    /// The current position of the enemy
    pub pos: Vec2,
    /// Health of the enemy
    pub health: u32,
    /// Velocity of the enemy moving towards the player // This is normalized to 1
    pub velocity: Vec2,
}

impl Enemy {
    pub fn new(pos: Vec2, health: u32, velocity: Vec2) -> Self {
        Self {
            pos,
            health,
            velocity,
        }
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 16.0, RED);
    }
}

impl Drawable for Enemy {
    fn draw(&self) {
        self.draw();
    }
}
