#![allow(dead_code)]
use macroquad::prelude::*;
use miniquad::window::screen_size;

#[derive(Debug, Default, Clone)]
pub struct Player {
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
    let mut cursor = Cursor::default();
    let mut world = World::new();

    loop {
        world.handle_inputs();
        world.tick();
        world.draw();

        cursor.handle_mouse();
        cursor.draw();
        next_frame().await
    }
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
    /// Health of the enemy
    health: u32,
    /// Velocity of the enemy moving towards the player // This is normalized to 1
    velocity: Vec2,
}

impl Enemy {
    pub fn new(pos: Vec2, health: u32, velocity: Vec2) -> Self {
        Self {
            pos,
            health,
            velocity,
        }
    }
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 16.0, RED);
    }
}

/// The shot made by a player or an enemy
pub struct Bullet {
    /// The position of the bullet
    pos: Vec2,
    /// The bullet travels at constant speed
    speed: f32,
    // velocity: Vec2,
    damage: u32,
}

/// The world struct
/// This contains the player, the enemies, and the center of the world
pub struct World {
    player: Player,
    enemies: Vec<Enemy>,
    center: Vec2,
    size: Vec2,
}

impl World {
    pub fn new() -> Self {
        let (x, y) = screen_size();
        let center = vec2(x / 2., y / 2.);

        Self {
            player: Player::new(center, center, center),
            enemies: Vec::default(),
            center,
            size: vec2(x, y),
        }
    }
    pub fn handle_inputs(&mut self) {
        self.player.handle_inputs();
    }
    pub fn draw(&self) {
        self.player.draw();
        self.enemies.iter().for_each(|enemy| enemy.draw());
    }

    pub fn spawn_enemy(&mut self) {
        let enemy = Enemy {
            pos: random_vec2_in_bounds(self.size),
            health: 100,
            velocity: vec2(0., 0.),
        };
        self.enemies.push(enemy);
    }

    pub fn tick(&mut self) {
        self.player.move_with_velocity();
        let enemies_count = self.enemies.len();
        if enemies_count < 10 && (macroquad::time::get_time() / (2 * enemies_count) as f64) > 1.0 {
            self.spawn_enemy();
        }
        for enemy in self.enemies.iter_mut() {
            enemy.velocity = (self.player.pos - enemy.pos).normalize();
            enemy.pos += enemy.velocity;
        }
    }
}

fn random_vec2_in_bounds(bounds: Vec2) -> Vec2 {
    vec2(
        rand::gen_range(0.0, bounds.x),
        rand::gen_range(0.0, bounds.y),
    )
}
