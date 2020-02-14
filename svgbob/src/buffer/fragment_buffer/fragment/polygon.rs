use crate::{
    fragment::{
        marker_line::{Marker, MarkerLine},
        Bounds,
    },
    Cell, Point,
};
use nalgebra::Point2;
use ncollide2d::shape::{shape::Shape, Polyline};
use sauron::{
    svg::{attributes::*, *},
    Node,
};
use std::{cmp::Ordering, fmt, ops::Deref};

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
}

impl Polygon {
    pub(in crate) fn new(points: Vec<Point>, is_filled: bool, tags: Vec<PolygonTag>) -> Self {
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

    pub(in crate) fn scale(&self, scale: f32) -> Self {
        let points: Vec<Point> = self.points.iter().map(|p| p.scale(scale)).collect();
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
        let points: Vec<Point2<f32>> = self.points.iter().map(|p| **p).collect();
        (ncollide2d::utils::center(&points)).into()
    }
}

impl Bounds for Polygon {
    fn bounds(&self) -> (Point, Point) {
        let points: Vec<Point2<f32>> = self.points.iter().map(|p| **p).collect();
        let aabb = Polyline::new(points, None).local_aabb();
        (Point::from(*aabb.mins()), Point::from(*aabb.maxs()))
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

impl Into<Node<()>> for Polygon {
    fn into(self) -> Node<()> {
        polygon(
            vec![
                points(
                    self.points
                        .iter()
                        .map(|p| format!("{},{}", p.x, p.y))
                        .collect::<Vec<String>>()
                        .join(" "),
                ),
                classes_flag([("filled", self.is_filled), ("nofill", !self.is_filled)]),
            ],
            vec![],
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
