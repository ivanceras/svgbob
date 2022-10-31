use crate::{fragment::Bounds, util, Cell, Point};
use parry2d::shape::{ConvexPolygon, Polyline, Segment, Shape};
use sauron::{
    html::attributes::*,
    svg::{attributes::*, *},
    Node,
};
use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone)]
pub struct Rect {
    pub start: Point,
    pub end: Point,
    pub is_filled: bool,
    pub radius: Option<f32>,
    //TODO:Make this as enum
    pub is_broken: bool,
}

impl Rect {
    /// creates a new rect and reorder the points swapping the end points if necessary
    /// such that the start is the most top-left and end point is the most bottom-right
    pub(crate) fn new(
        start: Point,
        end: Point,
        is_filled: bool,
        is_broken: bool,
    ) -> Self {
        let mut rect = Rect {
            start,
            end,
            is_filled,
            radius: None,
            is_broken,
        };
        rect.sort_reorder_end_points();
        rect
    }

    pub(crate) fn rounded_new(
        start: Point,
        end: Point,
        is_filled: bool,
        radius: f32,
        is_broken: bool,
    ) -> Self {
        let mut rect = Rect {
            start,
            end,
            is_filled,
            radius: Some(radius),
            is_broken,
        };
        rect.sort_reorder_end_points();
        rect
    }

    /// reorder the end points swap end points such that
    /// start < end
    pub(crate) fn sort_reorder_end_points(&mut self) {
        if self.start > self.end {
            std::mem::swap(&mut self.start, &mut self.end);
        }
    }

    /// recompute the rect with start and end point offset by the cell
    /// location
    pub(crate) fn absolute_position(&self, cell: Cell) -> Self {
        Rect {
            start: cell.absolute_position(self.start),
            end: cell.absolute_position(self.end),
            ..*self
        }
    }

    pub(crate) fn scale(&self, scale: f32) -> Self {
        Rect {
            start: self.start.scale(scale),
            end: self.end.scale(scale),
            radius: self.radius.map(|r| r * scale),
            ..*self
        }
    }

    pub(crate) fn width(&self) -> f32 {
        self.end.x - self.start.x
    }

    pub(crate) fn height(&self) -> f32 {
        self.end.y - self.start.y
    }

    pub(crate) fn is_broken(&self) -> bool {
        self.is_broken
    }

    pub fn is_rounded(&self) -> bool {
        if let Some(ref r) = &self.radius {
            *r > 0.0
        } else {
            false
        }
    }
}

impl Bounds for Rect {
    fn bounds(&self) -> (Point, Point) {
        let aabb = Segment::new(*self.start, *self.end).local_aabb();
        (Point::from(*aabb.mins), Point::from(*aabb.maxs))
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R {} {}", self.start, self.end)
    }
}

impl From<Rect> for Polyline {
    fn from(rect: Rect) -> Polyline {
        Polyline::new(
            vec![
                *rect.start,
                *Point::new(rect.end.x, rect.start.y),
                *rect.end,
                *Point::new(rect.start.x, rect.end.y),
                *rect.start,
            ],
            None,
        )
    }
}

impl From<Rect> for ConvexPolygon {
    fn from(rect: Rect) -> ConvexPolygon {
        ConvexPolygon::from_convex_polyline(vec![
            *rect.start,
            *Point::new(rect.end.x, rect.start.y),
            *rect.end,
            *Point::new(rect.start.x, rect.end.y),
        ])
        .expect("must create a convex polygon")
    }
}

impl<MSG> From<Rect> for Node<MSG> {
    fn from(r: Rect) -> Node<MSG> {
        rect(
            [
                x(r.start.x),
                y(r.start.y),
                width(r.width()),
                height(r.height()),
                classes_flag([
                    ("broken", r.is_broken),
                    ("solid", !r.is_broken),
                    ("filled", r.is_filled),
                    ("nofill", !r.is_filled),
                ]),
                if let Some(radius) = r.radius {
                    rx(radius)
                } else {
                    rx(0)
                },
            ],
            [],
        )
    }
}

impl Eq for Rect {}

impl Ord for Rect {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then(self.end.cmp(&other.end))
            .then(self.is_filled.cmp(&other.is_filled))
            .then(util::opt_ord(self.radius, other.radius))
            .then(self.is_broken.cmp(&other.is_broken))
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
