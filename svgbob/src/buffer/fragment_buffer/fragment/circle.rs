use crate::{fragment::Bounds, util, Cell, Point};
use nalgebra::Point2;
use parry2d::shape::ConvexPolygon;
use parry2d::shape::Polyline;
use std::{cmp::Ordering, fmt};

use sauron::{
    html::attributes::*,
    svg::{attributes::*, *},
    Node,
};

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f32,
    pub center: Point,
    pub is_filled: bool,
}

impl Circle {
    pub(in crate) fn new(center: Point, radius: f32, is_filled: bool) -> Self {
        Circle {
            center,
            radius,
            is_filled,
        }
    }

    /// the top most point of this circle for sorting.
    /// center.y - radius
    fn top_left_bound(&self) -> Point {
        Point::new(self.center.x - self.radius, self.center.y - self.radius)
    }
    fn top_right_bound(&self) -> Point {
        Point::new(self.center.x + self.radius, self.center.y - self.radius)
    }

    fn bottom_right_bound(&self) -> Point {
        Point::new(self.center.x + self.radius, self.center.y + self.radius)
    }

    fn bottom_left_bound(&self) -> Point {
        Point::new(self.center.x - self.radius, self.center.y + self.radius)
    }

    /// offset the circles parameter from the arg cell
    pub(in crate) fn absolute_position(&self, cell: Cell) -> Self {
        Circle {
            center: cell.absolute_position(self.center),
            ..*self
        }
    }

    pub fn scale(&self, scale: f32) -> Self {
        Circle {
            center: self.center.scale(scale),
            radius: self.radius * scale,
            ..*self
        }
    }
}

impl Bounds for Circle {
    fn bounds(&self) -> (Point, Point) {
        (self.top_left_bound(), self.bottom_right_bound())
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "C {} {}", self.center, self.radius)
    }
}

impl<MSG> Into<Node<MSG>> for Circle {
    fn into(self) -> Node<MSG> {
        circle(
            vec![
                cx(self.center.x),
                cy(self.center.y),
                r(self.radius),
                classes_flag([
                    ("filled", self.is_filled),
                    ("nofill", !self.is_filled),
                ]),
            ],
            vec![],
        )
    }
}

impl Eq for Circle {}

///This is needed since circle contains radius which is an f32 which rust doesn't provide trait
///implementation for Eq
impl Ord for Circle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.mins()
            .cmp(&other.mins())
            .then(self.maxs().cmp(&other.maxs()))
            .then(util::ord(self.radius, other.radius))
            .then(self.is_filled.cmp(&other.is_filled))
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Into<Polyline> for Circle {
    fn into(self) -> Polyline {
        let points: Vec<Point2<f32>> = extract_circle_points(self.radius, 64)
            .into_iter()
            .map(|p| Point2::new(p.x + self.center.x, p.y + self.center.y))
            .collect();

        Polyline::new(points, None)
    }
}

impl Into<ConvexPolygon> for Circle {
    fn into(self) -> ConvexPolygon {
        let points: Vec<Point2<f32>> = extract_circle_points(self.radius, 64)
            .into_iter()
            .map(|p| Point2::new(p.x + self.center.x, p.y + self.center.y))
            .collect();

        ConvexPolygon::from_convex_polyline(points)
            .expect("must create a convex polygon")
    }
}

fn extract_circle_points(radius: f32, nsubdivs: u32) -> Vec<Point> {
    let two_pi = std::f32::consts::TAU;
    let dtheta = two_pi / nsubdivs as f32;
    push_xy_arc(radius, nsubdivs, dtheta)
}

/// Pushes a discretized counterclockwise circle to a buffer.
/// The circle is contained on the plane spanned by the `x` and `y` axis.
fn push_xy_arc(radius: f32, nsubdiv: u32, dtheta: f32) -> Vec<Point> {
    let mut out: Vec<Point> = vec![];
    let mut curr_theta: f32 = 0.0;

    for _ in 0..nsubdiv {
        let x = curr_theta.cos() * radius;
        let y = curr_theta.sin() * radius;
        out.push(Point::new(x, y));

        curr_theta = curr_theta + dtheta;
    }
    out
}
