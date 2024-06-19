
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

use crate::draw::Drawable;
use crate::entity::Entity;
use crate::player::Player;
use crate::tick::Ticker;

/// The world struct
/// This contains the player, the enemies, and the center of the world
#[derive(Debug, Default)]
pub struct World {
    player: Player,
    center: Vec2,
    size: Vec2,
    tick: u64,
    entities: Vec<Entity>,
    frame_time: f32, // The rolling average frame time for the last 5 frames
    pub since_last_tick: f32,
}

impl World {
    pub fn new() -> Self {
        let size = screen_size();
        let size = vec2(size.0, size.1);
        let center = size / 2.0;
        Self {
            player: Player::new(center),
            center,
            size,
            ..Default::default()
        }
    }

    fn tick(&mut self) {
        self.player.tick();
        self.entities.tick();
    }

    pub fn next_frame(&mut self) {
        let current_frame_time = get_frame_time();
        if self.frame_time == 0.0 {
            self.frame_time = current_frame_time;
        } else {
            self.frame_time = (self.frame_time * 4.0 + current_frame_time) / 5.0;
        }
        self.since_last_tick += current_frame_time;
    }
}

impl Drawable for World {
    fn draw(&self) {
        self.player.draw();
        self.entities.draw();
    }
}

impl Ticker for World {
    fn tick(&mut self) {
        self.tick()
    }
}
