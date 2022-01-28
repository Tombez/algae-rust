#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}
impl Default for Vec2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
#[allow(dead_code)]
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self{ x, y }
    }
    pub fn assign(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    pub fn clone(&self) -> Self {
        Vec2{x: self.x, y: self.y}
    }
    pub fn dist_squared(&self, other: &Self) -> f32 {
        self.difference(other).length_squared()
    }
    pub fn product(&self, other: &Self) -> Self {
        *self.clone().multiply(other)
    }
    pub fn difference(&self, other: &Self) -> Self {
        *self.clone().subtract(other)
    }
    pub fn multiply(&mut self, other: &Self) -> &Self {
        self.x *= other.x;
        self.y *= other.y;
        self
    }
    pub fn subtract(&mut self, other: &Self) -> &Self {
        self.x -= other.x;
        self.y -= other.y;
        self
    }
    pub fn product_n(&self, n: f32) -> Self {
        Self {x: self.x * n, y: self.y * n}
    }
    pub fn quotient(&self, n: f32) -> Self {
        Self { x: self.x / n, y: self.y / n }
    }
    fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }
    fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
