use crate::{draw::Drawable, enemy::Enemy, gun::Bullet, player::Player};

pub enum Entity {
    Player(Player),
    Enemies(Enemy),
    Bullet(Bullet),
    Custom(Box<dyn Drawable>),
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
