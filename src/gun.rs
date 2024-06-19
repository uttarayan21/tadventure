use macroquad::prelude::*;

use crate::{draw::Drawable, movement::*, tick::Ticker};

/// The shot made by a player or an enemy
#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    movement: Movement,
    /// The damage the bullet does
    damage: u32,
}

impl Bullet {
    fn draw(&self) {
        draw_circle(self.movement.pos.x, self.movement.pos.y, 4.0, GREEN);
    }
}

impl Ticker for Bullet {
    fn tick(&mut self) {
        self.movement.tick();
    }
}

impl Drawable for Bullet {
    fn draw(&self) {
        self.draw();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GunType {
    ShotGun,
    Pistol,
    Sniper,
}

/// This is basically an abstraction of a bullet_factory lets say
#[derive(Debug, Clone)]
pub struct Gun {
    pub pos: Vec2,
    pub damage: u32,
    pub direction: Vec2,
    pub type_: GunType,
    pub bullets: Vec<Bullet>,
}

impl Gun {
    pub fn new(pos: Vec2, damage: u32, direction: Vec2, type_: GunType) -> Self {
        Self {
            pos,
            damage,
            direction,
            type_,
            bullets: Vec::new(),
        }
    }
    pub fn shoot(&mut self) {
        let bullet = Bullet {
            movement: Movement {
                pos: self.pos,
                direction: self.direction,
                type_: MovementType::FASTER,
            },
            damage: self.damage,
        };
        self.bullets.push(bullet);
    }
}

impl Drawable for Gun {
    fn draw(&self) {
        self.bullets.draw()
    }
}

impl Ticker for Gun {
    fn tick(&mut self) {
        self.bullets.tick();
    }
}
