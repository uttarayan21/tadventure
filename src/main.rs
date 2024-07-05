#![allow(dead_code)]
use std::time::Duration;

use draw::Drawable as _;
use macroquad::prelude::*;
use tick::{TickEvery, Ticker as _};
mod cursor;
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
    let mut cursor = cursor::Cursor::default();
    let mut world = world::World::new();
    // .every(Duration::from_millis(200));

    loop {
        cursor.tick();
        world.tick();

        cursor.draw();
        world.draw();
        world.next_frame();

        next_frame().await
    }
}
