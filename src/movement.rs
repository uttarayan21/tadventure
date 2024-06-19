use macroquad::prelude::*;
#[derive(Debug, Clone, Copy)]
pub enum MovementType {
    Speed(f32),
    Acceleration(Vec2),
}

#[derive(Debug, Clone, Copy)]
pub struct Movement {
    pub pos: Vec2,
    pub direction: Vec2, // Direction of the movement normalized to 1
    pub type_: MovementType,
}

impl Movement {
    pub fn tick(&mut self) {
        match self.type_ {
            MovementType::Speed(speed) => {
                self.pos += self.direction * speed;
            }
            MovementType::Acceleration(acceleration) => {
                self.direction += acceleration;
                self.direction = self.direction.normalize();
                self.pos += self.direction;
            }
        }
    }
    pub fn pos(&self) -> Vec2 {
        self.pos
    }
}
