use crate::{draw::Drawable, enemy::Enemy, gun::Bullet, player::Player, tick::Tick};

pub trait EntityTrait {
    fn as_tick(&mut self) -> &mut dyn Tick;
    fn as_draw(&self) -> &dyn Drawable;
}

impl<T: Tick + Drawable> EntityTrait for T {
    fn as_tick(&mut self) -> &mut dyn Tick {
        self
    }

    fn as_draw(&self) -> &dyn Drawable {
        self
    }
}

impl Drawable for Box<dyn EntityTrait> {
    fn draw(&self) {
        self.as_draw().draw();
    }
}

impl Tick for Box<dyn EntityTrait> {
    fn tick(&mut self) {
        self.as_tick().tick();
    }
}

// impl EntityTrait for Entity {
//     fn as_tick(&mut self) -> &mut dyn Tick {
//         match self {
//             Entity::Player(player) => player,
//             Entity::Enemies(enemy) => enemy,
//             Entity::Bullet(bullet) => bullet,
//             Entity::Custom(entity) => entity.as_tick(),
//         }
//     }
//
//     fn as_draw(&self) -> &dyn Drawable {
//         match self {
//             Entity::Player(player) => player,
//             Entity::Enemies(enemy) => enemy,
//             Entity::Bullet(bullet) => bullet,
//             Entity::Custom(entity) => entity.as_draw(),
//         }
//     }
// }

pub enum Entity {
    Player(Player),
    Enemies(Enemy),
    Bullet(Bullet),
    Custom(Box<dyn EntityTrait>),
}

impl core::fmt::Debug for Entity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Entity::Player(player) => write!(f, "Entity::Player({:?})", player),
            Entity::Enemies(enemy) => write!(f, "Entity::Enemies({:?})", enemy),
            Entity::Bullet(bullet) => write!(f, "Entity::Bullet({:?})", bullet),
            Entity::Custom(_) => write!(f, "Entity::Custom(...)"),
        }
    }
}

impl Drawable for Entity {
    fn draw(&self) {
        match self {
            Entity::Player(player) => player.draw(),
            Entity::Enemies(enemy) => enemy.draw(),
            Entity::Bullet(bullet) => bullet.draw(),
            Entity::Custom(drawable) => drawable.draw(),
        }
    }
}

impl Tick for Entity {
    fn tick(&mut self) {
        match self {
            Entity::Player(player) => player.tick(),
            // Entity::Enemies(enemy) => enemy.tick(),
            // Entity::Bullet(bullet) => bullet.tick(),
            Entity::Custom(tick) => tick.tick(),
            _ => (),
        }
    }
}
