pub struct Vec2<T> {
    pub x: T,
    pub y: T
}
#[allow(dead_code)]
impl<T> Vec2<T> {
    pub fn assign(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}
