use macroquad::prelude::*;

pub struct Character {
    pos: Vec2,
    pointing: Vec2,
    velocity: Vec2,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Balls".to_owned(),
        fullscreen: true,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut character = Character::new(
        Vec2::new(100.0, 100.0),
        Vec2::new(0.0, 0.0),
        vec2(0f32, 0f32),
    );
    let mut cursor = Cursor::default();

    loop {
        character.handle_inputs();
        character.draw();
        cursor.handle_mouse();
        cursor.draw();
        next_frame().await
    }
}

impl Character {
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
        self.move_with_velocity();
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

/// The basic enemy data
/// This condaions just he enemy position
/// and the enemy class
/// The enemy just moves towards the player constantly if there's nothing in the way.
pub struct Enemy {
    /// The current position of the enemy
    pos: Vec2,
    /// The class of the enemy
    class: EnemyClass,
    /// Health of the enemy
    health: u32,
    /// Velocity of the enemy moving towards the player // This is normalized to 1
    velocity: Vec2,
}

pub enum EnemyClass {
    Melee,
    Ranged,
    Boss,
}

/// The world struct
/// This contains the player, the enemies, and the center of the world
pub struct World {
    player: Character,
    enemies: Vec<Enemy>,
    center: Vec2,
}

pub fn move_with_velocity(pos: &mut Vec2, velocity: Vec2) {
    *pos += velocity;
}
