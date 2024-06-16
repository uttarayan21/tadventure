use macroquad::prelude::*;

pub struct Character {
    pos: Vec2,
    pointing: Vec2,
    velocity: Vec2,
}

// static BACKGROUND_TEXTURE: Lazy<Texture2D> =
//     Lazy::new(|| Texture2D::from_file_with_format(include_bytes!("../assets/grass.png"), None));
fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
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
        clear_background(GREEN);
        // draw_texture(&BACKGROUND_TEXTURE, 0.0, 0.0, WHITE);
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
        self.pos = self.pos.move_towards(self.pointing, distance / 15f32);
    }

    pub fn handle_mouse(&mut self) {
        let mouse_pos = mouse_position();
        self.pointing = vec2(mouse_pos.0, mouse_pos.1);
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 16.0, BLUE);
        draw_line(
            self.pos.x,
            self.pos.y,
            self.pointing.x,
            self.pointing.y,
            5.0,
            RED,
        );
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

pub struct Enemy {
    pos: Vec2,
}
