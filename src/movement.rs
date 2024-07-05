use macroquad::prelude::*;

use crate::tick::Ticker;
#[derive(Debug, Clone, Copy)]
pub enum MovementType {
    Speed(f32),
    Acceleration(f32),
}

impl MovementType {
    pub const FAST: Self = Self::Speed(5.0);
    pub const MEDIUM: Self = Self::Speed(3.0);
    pub const SLOW: Self = Self::Speed(1.0);
    pub const STOP: Self = Self::Speed(0.0);
    pub const FASTER: Self = Self::Acceleration(1.0);
    pub const SLOWER: Self = Self::Acceleration(-1.0);
}

#[derive(Debug, Clone, Copy)]
pub struct Movement {
    pub pos: Vec2,
    pub direction: Vec2, // Direction of the movement normalized to 1
    pub type_: MovementType,
}

impl Movement {
    fn tick(&mut self) {
        match self.type_ {
            MovementType::Speed(speed) => {
                self.pos += self.direction * speed;
            }
            MovementType::Acceleration(acceleration) => {
                self.direction *= acceleration;
                self.pos += self.direction;
            }
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }
}

impl Ticker for Movement {
    fn tick(&mut self) {
        self.tick();
    }
}
