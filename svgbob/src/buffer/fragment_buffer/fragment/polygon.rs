use crate::Direction;
use crate::{
    fragment::{marker_line::Marker, Bounds},
    Cell, Point,
};
use nalgebra::Point2;
use parry2d::shape::{shape::Shape, Polyline};
use sauron::{
    html::attributes::*,
    svg::{attributes::*, *},
    Node,
};
use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum PolygonTag {
    //    ^
    //     \
    ArrowTopLeft,
    //    ^
    //    |
    ArrowTop,
    //     ^
    //    /
    ArrowTopRight,
    //   <----
    ArrowLeft,
    //   ---->
    ArrowRight,
    //    /
    //   V
    ArrowBottomLeft,
    //     |
    //     V
    ArrowBottom,
    //     \
    //      V
    ArrowBottomRight,
    //  #
    //   \
    DiamondBullet,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Point>,
    pub is_filled: bool,
    /// tag is added in order to not keep detecting the shape
    /// of the polygon to arrows/diamond
    pub tags: Vec<PolygonTag>,
}

impl PolygonTag {
    pub(crate) fn get_marker(&self) -> Marker {
        match self {
            PolygonTag::ArrowTopLeft
            | PolygonTag::ArrowTop
            | PolygonTag::ArrowTopRight
            | PolygonTag::ArrowLeft
            | PolygonTag::ArrowRight
            | PolygonTag::ArrowBottomLeft
            | PolygonTag::ArrowBottom
            | PolygonTag::ArrowBottomRight => Marker::Arrow,
            PolygonTag::DiamondBullet => Marker::Diamond,
        }
    }

    pub(crate) fn direction(&self) -> Option<Direction> {
        match self {
            PolygonTag::ArrowTopLeft => Some(Direction::TopLeft),
            PolygonTag::ArrowTop => Some(Direction::Top),
            PolygonTag::ArrowTopRight => Some(Direction::TopRight),
            PolygonTag::ArrowLeft => Some(Direction::Left),
            PolygonTag::ArrowRight => Some(Direction::Right),
            PolygonTag::ArrowBottomLeft => Some(Direction::BottomLeft),
            PolygonTag::ArrowBottom => Some(Direction::Bottom),
            PolygonTag::ArrowBottomRight => Some(Direction::BottomRight),
            PolygonTag::DiamondBullet => None,
        }
    }

    pub(crate) fn matched_direction(&self, arg: Direction) -> bool {
        if let Some(direction) = self.direction() {
            direction == arg
        } else {
            // DiamondBullet just match any direction
            true
        }
    }
}

impl Polygon {
    pub(in crate) fn new(
        points: Vec<Point>,
        is_filled: bool,
        tags: Vec<PolygonTag>,
    ) -> Self {
        Polygon {
            points,
            is_filled,
            tags,
        }
    }

    pub(in crate) fn absolute_position(&self, cell: Cell) -> Self {
        let points: Vec<Point> = self
            .points
            .iter()
            .map(|p| cell.absolute_position(*p))
            .collect();
        Polygon {
            points,
            is_filled: self.is_filled,
            tags: self.tags.clone(),
        }
    }

    pub(crate) fn matched_direction(&self, direction: Direction) -> bool {
        self.tags.iter().any(|tag| tag.matched_direction(direction))
    }

    fn is_diamond(&self) -> bool {
        self.tags.len() == 1
            && self
                .tags
                .iter()
                .all(|tag| *tag == PolygonTag::DiamondBullet)
    }

    /// returns Diamond marker if the tags is a DiamondBullet
    /// otherwise if it is an Arrow direction, then return Arrow.
    pub(crate) fn get_marker(&self) -> Option<Marker> {
        if !self.tags.is_empty() {
            if self.is_diamond() {
                Some(Marker::Diamond)
            } else if self
                .tags
                .iter()
                .all(|tag| tag.get_marker() == Marker::Arrow)
            {
                Some(Marker::Arrow)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(in crate) fn scale(&self, scale: f32) -> Self {
        let points: Vec<Point> =
            self.points.iter().map(|p| p.scale(scale)).collect();
        Polygon {
            points,
            is_filled: self.is_filled,
            tags: self.tags.clone(),
        }
    }

    fn first(&self) -> Point {
        self.points[0]
    }

    fn last(&self) -> Point {
        let n = self.points.len();
        self.points[n - 1]
    }

    pub(crate) fn center(&self) -> Point {
        let points: Vec<Point2<f32>> =
            self.points.iter().map(|p| **p).collect();
        (parry2d::utils::center(&points)).into()
    }
}

impl Bounds for Polygon {
    fn bounds(&self) -> (Point, Point) {
        let pl: Polyline = self.clone().into();
        let aabb = pl.local_aabb();
        (Point::from(*aabb.mins), Point::from(*aabb.maxs))
    }
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "P {}",
            self.points
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Into<Polyline> for Polygon {
    fn into(self) -> Polyline {
        let points: Vec<Point2<f32>> =
            self.points.iter().map(|p| **p).collect();
        Polyline::new(points, None)
    }
}

impl<MSG> Into<Node<MSG>> for Polygon {
    fn into(self) -> Node<MSG> {
        polygon(
            [
                points(
                    self.points
                        .iter()
                        .map(|p| format!("{},{}", p.x, p.y))
                        .collect::<Vec<String>>()
                        .join(" "),
                ),
                classes_flag([
                    ("filled", self.is_filled),
                    ("nofill", !self.is_filled),
                ]),
            ],
            [],
        )
    }
}

impl Eq for Polygon {}

/// This is needed since this struct contains f32 which rust doesn't provide Eq implementation
impl Ord for Polygon {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.points == other.points {
            Ordering::Equal
        } else {
            self.first()
                .cmp(&other.first())
                .then(self.last().cmp(&other.last()))
                .then(self.is_filled.cmp(&other.is_filled))
                .then(self.points.len().cmp(&other.points.len()))
        }
    }
}

impl PartialOrd for Polygon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Polygon {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
