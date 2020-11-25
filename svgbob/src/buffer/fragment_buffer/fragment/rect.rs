use crate::{fragment::Bounds, util, Cell, Point};
use std::fmt;

use ncollide2d::shape::{Segment, Shape};
use sauron::{
    html::{attributes::*},
    svg::{attributes::*, *},
    Node,
};
use std::cmp::Ordering;

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
    pub(in crate) fn new(start: Point, end: Point, is_filled: bool, is_broken: bool) -> Self {
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

    pub(in crate) fn rounded_new(
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
    pub(in crate) fn sort_reorder_end_points(&mut self) {
        if self.start > self.end {
            let tmp_start = self.start;
            self.start = self.end;
            self.end = tmp_start;
        }
    }

    /// recompute the rect with start and end point offset by the cell
    /// location
    pub(in crate) fn absolute_position(&self, cell: Cell) -> Self {
        Rect {
            start: cell.absolute_position(self.start),
            end: cell.absolute_position(self.end),
            ..*self
        }
    }

    pub(in crate) fn scale(&self, scale: f32) -> Self {
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
}

impl Bounds for Rect {
    fn bounds(&self) -> (Point, Point) {
        let aabb = Segment::new(*self.start, *self.end).local_aabb();
        (Point::from(*aabb.mins()), Point::from(*aabb.maxs()))
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "R {} {}", self.start, self.end)
    }
}

impl<MSG> Into<Node<MSG>> for Rect {
    fn into(self) -> Node<MSG> {
        rect(
            vec![
                x(self.start.x),
                y(self.start.y),
                width(self.width()),
                height(self.height()),
                classes_flag([
                    ("broken", self.is_broken),
                    ("solid", !self.is_broken),
                    ("filled", self.is_filled),
                    ("nofill", !self.is_filled),
                ]),
                if let Some(radius) = self.radius {
                    rx(radius)
                } else {
                    rx(0)
                },
            ],
            vec![],
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
