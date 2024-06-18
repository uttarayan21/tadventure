use macroquad::prelude::*;
use miniquad::window::screen_size;

use crate::enemy::Enemy;
use crate::gun::Bullet;
use crate::player::Player;

/// The world struct
/// This contains the player, the enemies, and the center of the world
#[derive(Debug, Default)]
pub struct World {
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    center: Vec2,
    size: Vec2,
    tick: u64,
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
            ..Default::default()
        }
    }
    pub fn handle_inputs(&mut self) {
        self.player.handle_inputs();
    }

    pub fn draw(&self) {
        self.player.draw();
        self.enemies.iter().for_each(|enemy| enemy.draw());
        self.bullets.iter().for_each(|bullet| bullet.draw());
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
        self.tick += 1;
        self.player.move_with_velocity();
        let enemies_count = self.enemies.len();
        if enemies_count < 10 && (macroquad::time::get_time() / (2 * enemies_count) as f64) > 1.0 {
            self.spawn_enemy();
        }
        for enemy in self.enemies.iter_mut() {
            enemy.velocity = (self.player.pos - enemy.pos).normalize();
            enemy.pos += enemy.velocity;
        }
        for bullet in self.bullets.iter_mut() {
            bullet.tick();
        }
        if self.tick % 12 == 0 {
            self.bullets.push(Bullet::shoot(self.player.pos));
        }
    }
}

fn random_vec2_in_bounds(bounds: Vec2) -> Vec2 {
    vec2(
        rand::gen_range(0.0, bounds.x),
        rand::gen_range(0.0, bounds.y),
    )
}
