use crate::{buffer::Cell, fragment::Bounds, util, Point};
use parry2d::shape::{Segment, Shape};
use sauron::{
    html::attributes::*,
    svg::{attributes::*, *},
    Node,
};
use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone)]
pub struct Arc {
    pub start: Point,
    pub end: Point,
    pub radius: f32,
    major_flag: bool,
    pub sweep_flag: bool,
    rotation_flag: bool,
}

impl Arc {
    /// create an arc from start to end with a radius
    /// direction is counter clock wise
    pub(crate) fn new(start: Point, end: Point, radius: f32) -> Self {
        let mut arc = Arc {
            start,
            end,
            radius,
            /// always false, since arcs are mostly in minor arc
            major_flag: false,
            sweep_flag: false,
            rotation_flag: false,
        };
        arc.sort_reorder_end_points();
        arc
    }

    pub(crate) fn major(start: Point, end: Point, radius: f32) -> Self {
        let mut arc = Arc {
            start,
            end,
            radius,
            major_flag: true,
            sweep_flag: false,
            rotation_flag: false,
        };
        arc.sort_reorder_end_points();
        arc
    }

    /// check if this arcs to point a, b
    /// disregarding radius
    pub(crate) fn arcs_to(&self, a: Point, b: Point) -> bool {
        let arc = Arc::new(a, b, 1.0);
        self.start == arc.start
            && self.end == arc.end
            && self.sweep_flag == arc.sweep_flag
    }

    pub(crate) fn new_with_sweep(
        start: Point,
        end: Point,
        radius: f32,
        sweep_flag: bool,
    ) -> Self {
        let mut arc = Arc {
            start,
            end,
            radius,
            /// always false, since arcs are mostly in minor arc
            major_flag: false,
            sweep_flag,
            rotation_flag: false,
        };
        arc.sort_reorder_end_points();
        arc
    }

    pub(crate) fn absolute_position(&self, cell: Cell) -> Self {
        Arc {
            start: cell.absolute_position(self.start),
            end: cell.absolute_position(self.end),
            ..*self
        }
    }

    /// reverse the order of points and also set the flag to true, to
    /// make the rotation clockwise
    pub(crate) fn sort_reorder_end_points(&mut self) {
        if self.start > self.end {
            std::mem::swap(&mut self.start, &mut self.end);
            self.sweep_flag = !self.sweep_flag;
        }
    }

    pub fn scale(&self, scale: f32) -> Self {
        Arc {
            start: self.start.scale(scale),
            end: self.end.scale(scale),
            radius: self.radius * scale,
            ..*self
        }
    }

    /// check to see of this arc is touching the other arc
    pub(crate) fn is_touching(&self, other: &Self) -> bool {
        self.start == other.start
            || self.end == other.end
            || self.start == other.end
            || self.end == other.start
    }

    pub fn has_endpoint(&self, p: Point) -> bool {
        self.start == p || self.end == p
    }

    /// calculate the center point this arc
    pub fn center(&self) -> Point {
        let start = self.start;
        let end = self.end;
        let q = start.distance(&end);
        let y3 = (start.y + end.y) / 2.0;
        let x3 = (start.x + end.x) / 2.0;

        let rr_q22 = (self.radius.powf(2.0) - (q / 2.0).powf(2.0)).sqrt();

        let base_x = rr_q22 * (start.y - end.y) / q;
        let base_y = rr_q22 * (end.x - start.x) / q;

        if self.sweep_flag {
            let cx = x3 + base_x;
            let cy = y3 + base_y;
            Point::new(cx, cy)
        } else {
            let cx = x3 - base_x;
            let cy = y3 - base_y;
            Point::new(cx, cy)
        }
    }

    /// check to see if the arc is aabb right angle
    /// that is the center x and y coordinate is alinged to both of the end points
    /// This will be used for checking if group of fragments can be a rounded rect
    pub fn is_aabb_right_angle_arc(&self) -> bool {
        let center = self.center();
        (center.x == self.start.x && center.y == self.end.y)
            || (center.x == self.end.x && center.y == self.start.y)
    }
}

impl Bounds for Arc {
    fn bounds(&self) -> (Point, Point) {
        let aabb = Segment::new(*self.start, *self.end).local_aabb();
        (Point::from(*aabb.mins), Point::from(*aabb.maxs))
    }
}

impl fmt::Display for Arc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A {} {} {} -> {} {} {}",
            self.start,
            self.end,
            self.radius,
            self.rotation_flag as u8,
            self.major_flag as u8,
            self.sweep_flag as u8,
        )
    }
}

impl<MSG> From<Arc> for Node<MSG> {
    fn from(arc: Arc) -> Node<MSG> {
        let dv = format!(
            "M {},{} A {},{} {},{},{} {},{}",
            arc.start.x,
            arc.start.y,
            arc.radius,
            arc.radius,
            arc.rotation_flag as u8,
            arc.major_flag as u8,
            arc.sweep_flag as u8,
            arc.end.x,
            arc.end.y
        );
        path(vec![d(dv), class("nofill")], vec![])
    }
}

impl Eq for Arc {}

impl Ord for Arc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then(self.end.cmp(&other.end))
            .then(util::ord(self.radius, other.radius))
            .then(self.rotation_flag.cmp(&other.rotation_flag))
            .then(self.major_flag.cmp(&other.major_flag))
            .then(self.sweep_flag.cmp(&other.sweep_flag))
    }
}

impl PartialOrd for Arc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Arc {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::CellGrid;

    #[test]
    fn test_arc_centers() {
        let e = CellGrid::e();
        let y = CellGrid::y();
        let o = CellGrid::o();
        let arc = Arc::new(e, y, 1.0);
        assert_eq!(o, arc.center());
        assert!(!arc.is_aabb_right_angle_arc());
    }

    #[test]
    fn test_arc_ke_center_a() {
        let a = CellGrid::a();
        let e = CellGrid::e();
        let k = CellGrid::k();
        let _o = CellGrid::o();
        let arc = Arc::new(k, e, 1.0);
        // 1st, up, ltr, swapped, steepup
        assert_eq!(a, arc.center());
        assert!(arc.is_aabb_right_angle_arc());
    }

    #[test]
    fn test_arc_ao_center_e() {
        let a = CellGrid::a();
        let e = CellGrid::e();
        let o = CellGrid::o();
        let _k = CellGrid::k();
        let arc = Arc::new(a, o, 1.0);
        assert_eq!(e, arc.center());
        assert!(arc.is_aabb_right_angle_arc());
    }

    #[test]
    fn test_arc_or_center_t() {
        let o = CellGrid::o();
        let r = CellGrid::r();
        let _m = CellGrid::m();
        let t = CellGrid::t();
        let arc = Arc::new(o, r, 0.5);
        assert_eq!(t, arc.center());
        assert!(arc.is_aabb_right_angle_arc());
    }

    #[test]
    fn test_arc_kr_center_p() {
        let k = CellGrid::k();
        let r = CellGrid::r();
        let _m = CellGrid::m();
        let p = CellGrid::p();
        let arc = Arc::new(r, k, 0.5);
        // 1st, down, ltr, swapped, slowdescent
        assert_eq!(p, arc.center());
        assert!(arc.is_aabb_right_angle_arc());
    }

    #[test]
    fn test_distance() {
        let a = CellGrid::a();
        let e = CellGrid::e();
        let m = CellGrid::m();
        let n = CellGrid::n();
        assert_eq!(1.0, a.distance(&e));
        assert_eq!(0.25, m.distance(&n));
    }
}
