#![allow(dead_code)]
use std::time::Duration;

use draw::Drawable as _;
use macroquad::prelude::*;
use tick::{TickEvery, Ticker as _};
mod draw;
mod enemy;
mod entity;
mod gun;
mod movement;
mod player;
mod tick;
mod world;

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
    cursor.handle_mouse();
    let mut world = world::World::new().every(Duration::from_millis(5));

    loop {
        cursor.draw();
        world.draw();
        world.tick();
        world.next_frame();

        next_frame().await
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
