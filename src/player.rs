use macroquad::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub pointing: Vec2,
    pub velocity: Vec2,
}

impl Player {
    pub fn new(pos: Vec2, pointing: Vec2, velocity: Vec2) -> Self {
        Self {
            pos,
            pointing,
            velocity,
        }
    }

    pub fn handle_inputs(&mut self) {
        self.handle_mouse();
        let distance = self.pos.distance(self.pointing);
        // increase the velocity scaled to the distance
        self.velocity = (self.pointing - self.pos).normalize() * distance / 15f32;
    }

    pub fn handle_mouse(&mut self) {
        let mouse_pos = mouse_position();
        self.pointing = vec2(mouse_pos.0, mouse_pos.1);
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 16.0, BLUE);
    }

    pub fn move_with_velocity(&mut self) {
        self.pos += self.velocity;
    }
}
