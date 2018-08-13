
use std::cmp::Ordering;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl Ord for Point{
    fn cmp(&self, other:&Point) -> Ordering{
        if let Some(order) = self.partial_cmp(other){
            return order
        }
        Ordering::Less
    }
}
impl Eq for Point{
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
    pub fn adjust(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
}

// 3 points are collinear when the area of the triangle connecting them is 0;
pub fn collinear(a: &Point, b: &Point, c: &Point) -> bool {
    a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y) == 0.0
}

pub fn distance(a: &Point, b: &Point) -> f32{
    ((b.x - a.x ).powi(2) + (b.y - a.y).powi(2)).sqrt()
}
