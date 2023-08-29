use crate::{buffer::CellGrid, util, Cell};
use nalgebra::{coordinates::XY, Point2, Vector2};
use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Deref, Sub},
};

#[derive(Clone, Copy, Debug)]
pub struct Point(pub Point2<f32>);

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

    /// adjust x value by units specified
    pub fn adjust_x(&self, units: f32) -> Self {
        let t = units * CellGrid::unit_x();
        Self::new(self.x + t, self.y)
    }

    /// adjust y value by units specified
    pub fn adjust_y(&self, units: f32) -> Self {
        let t = units * CellGrid::unit_y();
        Self::new(self.x, self.y + t)
    }

    /// adjust both x and y value by units specified
    pub fn adjust(&self, units_x: f32, units_y: f32) -> Self {
        let t = units_x * CellGrid::unit_x();
        let u = units_y * CellGrid::unit_y();
        Self::new(self.x + t, self.y + u)
    }

    /// test if the point lie on an edge of a cell
    /// that is the fractional part is 0.0
    pub fn is_edge_x(&self) -> bool {
        self.x.fract() == 0.0
    }

    pub fn is_edge_y(&self) -> bool {
        (self.y / 2.0).fract() == 0.0
    }

    pub fn is_mid_x(&self) -> bool {
        self.x.fract() == 0.5
    }

    pub fn is_mid_y(&self) -> bool {
        (self.y / 2.0).fract() == 0.5
    }

    /// return the cell where this point fall to
    pub fn cell(&self) -> Cell {
        let (cell, _) = Cell::snap_point(*self);
        cell
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

impl From<XY<i32>> for Point {
    fn from(point: XY<i32>) -> Self {
        Point::new(point.x as f32, point.y as f32)
    }
}

impl From<XY<f32>> for Point {
    fn from(point: XY<f32>) -> Self {
        Point::new(point.x, point.y)
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
