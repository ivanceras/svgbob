use crate::util;
use nalgebra::{Point2, Vector2};
use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Deref, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Point(Point2<f32>);

impl Deref for Point {
    type Target = Point2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point(Point2::new(x, y))
    }

    /// convert a point to vector
    #[inline]
    pub fn to_vector(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }

    /// scale point
    pub fn scale(&self, scale: f32) -> Self {
        Point::new(self.x * scale, self.y * scale)
    }

    pub fn normalize(&self) -> Point {
        let vector = self.to_vector().normalize();
        Point::new(vector.x, vector.y)
    }

    pub fn distance(&self, other: &Self) -> f32 {
        nalgebra::distance(&self.0, &other.0)
    }

    /// align x to 0.5
    /// align y to odd number
    pub fn align(&self) -> Self {
        let x = self.x.round() + 0.5;
        let y = if self.y.round().rem_euclid(2.0) == 0.0 {
            self.y.round() + 1.0
        } else {
            self.y.round()
        };
        Point::new(x, y)
    }
}

impl From<Point2<f32>> for Point {
    fn from(point: Point2<f32>) -> Self {
        Point(point)
    }
}

impl From<Point2<i32>> for Point {
    fn from(point: Point2<i32>) -> Self {
        Point::new(point.x as f32, point.y as f32)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        util::ord(self.y, other.y).then(util::ord(self.x, other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_align() {
        let p = Point::new(1.25, 2.0);
        let p2 = p.align();
        assert_eq!(p2, Point::new(1.5, 3.0));
    }
}
