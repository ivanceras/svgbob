
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
        Ordering::Equal
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

pub fn is_on_line(point: &Point, start: &Point, end: &Point) -> bool {
    ((point.x - start.x) / (end.x - start.x)) == ((point.y - start.y) / (end.y - start.y))
}
// Given three colinear points p, q, r, the function checks if
// point q lies on line segment 'pr'
pub fn on_segment(q: &Point, p:&Point, r:&Point)-> bool {
    q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) &&
            q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
}

// 3 points are collinear when the area of the triangle connecting them is 0;
pub fn collinear(a: &Point, b: &Point, c: &Point) -> bool {
    a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y) == 0.0
}

pub fn distance(a: &Point, b: &Point) -> f32{
    ((b.x - a.x ).powi(2) + (b.y - a.y).powi(2)).sqrt()
}
