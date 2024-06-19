pub trait Drawable {
    fn draw(&self);
}

impl<T: Drawable> Drawable for Box<T> {
    fn draw(&self) {
        self.as_ref().draw();
    }
}

impl<T: Drawable> Drawable for Vec<T> {
    fn draw(&self) {
        self.iter().for_each(|item| item.draw());
    }
}

impl<T: Drawable> Drawable for Option<T> {
    fn draw(&self) {
        if let Some(item) = self {
            item.draw();
        }
    }
}

impl<T: Drawable> Drawable for &T {
    fn draw(&self) {
        (*self).draw();
    }
}

// impl<T: Drawable, I: Iterator<Item = T>> Drawable for I {
//     fn draw(&self) {
//         self.for_each(|item| item.draw());
//     }
// }
