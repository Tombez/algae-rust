use crate::vec2::Vec2;

fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v > min { if v < max { v } else { max } } else { min }
}

pub trait Shape {
    fn overlaps(&self, aabb: &AABB) -> bool;
    fn contains(&self, point: &Vec2) -> bool;
    fn contained_by(&self, aabb: &AABB) -> bool;
}

pub struct AABB {
    pub pos: Vec2,
    pub size: Vec2
}
impl Shape for AABB {
    fn overlaps(&self, aabb: &AABB) -> bool {
        self.pos.x + self.size.x > aabb.pos.x
            && self.pos.x < aabb.pos.x + aabb.size.x
            && self.pos.y + self.size.y > aabb.pos.y
            && self.pos.y < aabb.pos.y + aabb.size.y
    }
    fn contains(&self, point: &Vec2) -> bool {
        point.x > self.pos.x
            && point.x < self.pos.x + self.size.x
            && point.y > self.pos.y
            && point.y < self.pos.y + self.size.y
    }
    fn contained_by(&self, aabb: &AABB) -> bool {
        self.pos.x >= aabb.pos.x
            && self.pos.x + self.size.x <= aabb.pos.x + aabb.size.x
            && self.pos.y >= aabb.pos.y
            && self.pos.y + self.size.y <= aabb.pos.y + aabb.size.y
    }
}

pub struct Circle {
    pub pos: Vec2,
    pub r: f32
}
impl Shape for Circle {
    fn overlaps(&self, aabb: &AABB) -> bool {
        let mut pos = self.pos.clone();
        pos.x = clamp(pos.x, aabb.pos.x, aabb.pos.x + aabb.size.x);
        pos.y = clamp(pos.y, aabb.pos.y, aabb.pos.y + aabb.size.y);
        self.contains(&pos)
    }
    fn contains(&self, point: &Vec2) -> bool {
        self.pos.dist_squared(point) <= self.r * self.r
    }
    fn contained_by(&self, aabb: &AABB) -> bool {
        self.pos.x - self.r >= aabb.pos.x
            && self.pos.x + self.r <= aabb.pos.x + aabb.size.x
            && self.pos.y - self.r >= aabb.pos.y
            && self.pos.y + self.r <= aabb.pos.y + aabb.size.y
    }
}


pub struct LooseQuadtreeNode<T: Shape> {
    aabb: AABB,
    children: Vec<Self>,
    items: Vec<T>
}
impl<T: Shape> LooseQuadtreeNode<T> {
    pub fn new(aabb: AABB) -> Self {
        Self { aabb, children: vec![], items: vec![] }
    }
    pub fn insert(&mut self, item: T, level: u8, max_items: u8) {
        if !self.children.is_empty() {
            for i in 0..4 {
                let child = &mut (*self.children)[i];
                if item.contained_by(&child.aabb) {
                    child.insert(item, level - 1, max_items);
                    break;
                }
            }
        } else {
            self.items.push(item);
            if level == 0 { return }
            if self.items.len() > max_items.into() {
                self.split(level, max_items)
            }
        }
    }
    fn split(&mut self, level: u8, max_items: u8) {
        let hs = self.aabb.size.quotient(2.0);
        let qs = hs.quotient(2.0);
        for i in 0..4 {
            let index = Vec2 { x: (i & 1) as f32, y: (i & 2) as f32};
            let pos = *self.aabb.pos.product(&index).subtract(&qs);
            let aabb = AABB { pos, size: hs.clone() };
            self.children.push(LooseQuadtreeNode::new(aabb));
        }
        let mut i: u8 = 0;
        let mut len = self.items.len();
        'outer: while (i as usize) < len {
            let item = &self.items[i as usize];
            for child in &mut self.children {
                if item.contained_by(&child.aabb) {
                    self.items.swap(i as usize, len - 1);
                    let item = self.items.pop().unwrap();
                    len -= 1;
                    child.insert(item, level - 1, max_items);
                    continue 'outer;
                }
            }
            i += 1
        }
    }
}
