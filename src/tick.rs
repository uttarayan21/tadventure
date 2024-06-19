pub trait Tick {
    fn tick(&mut self);
}

impl<T: Tick> Tick for Box<T> {
    fn tick(&mut self) {
        self.as_mut().tick();
    }
}

impl<T: Tick> Tick for Vec<T> {
    fn tick(&mut self) {
        self.iter_mut().for_each(|item| item.tick());
    }
}

impl<T: Tick> Tick for Option<T> {
    fn tick(&mut self) {
        if let Some(item) = self {
            item.tick();
        }
    }
}
