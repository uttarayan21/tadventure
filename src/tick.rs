use core::time::Duration;

use crate::draw::Drawable;

pub trait Ticker {
    fn tick(&mut self);
}

impl<T: Ticker> Ticker for Box<T> {
    fn tick(&mut self) {
        self.as_mut().tick();
    }
}

impl<T: Ticker> Ticker for Vec<T> {
    fn tick(&mut self) {
        self.iter_mut().for_each(|item| item.tick());
    }
}

impl<T: Ticker> Ticker for Option<T> {
    fn tick(&mut self) {
        if let Some(item) = self {
            item.tick();
        }
    }
}

pub struct Tick<T> {
    pub ticker: T,
    pub time: Duration,
}

pub trait TickEvery: Sized + Ticker {
    fn every(self, time: Duration) -> Tick<Self>;
}

impl TickEvery for crate::world::World {
    fn every(self, time: Duration) -> Tick<Self> {
        Tick { ticker: self, time }
    }
}

impl Ticker for Tick<crate::world::World> {
    fn tick(&mut self) {
        if self.ticker.since_last_tick >= self.time.as_secs_f32() {
            self.ticker.tick();
            self.ticker.since_last_tick = 0.0;
        }
    }
}

impl<T: Drawable> Drawable for Tick<T> {
    fn draw(&self) {
        self.ticker.draw()
    }
}
