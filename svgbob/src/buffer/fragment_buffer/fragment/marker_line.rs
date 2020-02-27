use crate::{
    fragment::{marker_line, Bounds, Cell, Fragment, Line, Polygon},
    Point,
};
use sauron::{html::attributes::class, Node};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Marker {
    //   -->
    Arrow,
    //  ---|>
    ClearArrow,
    //  ----*
    Circle,
    // -----#
    Square,
    //   \
    //    \
    //     #
    Diamond,
    // -----o
    OpenCircle,
    // -----O
    BigOpenCircle,
}

impl Marker {
    fn is_arrow(&self) -> bool {
        match self {
            Marker::Arrow | Marker::ClearArrow => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarkerLine {
    pub line: Line,
    pub start_marker: Option<Marker>,
    pub end_marker: Option<Marker>,
}

impl MarkerLine {
    pub fn new(
        a: Point,
        b: Point,
        is_broken: bool,
        start_marker: Option<Marker>,
        end_marker: Option<Marker>,
    ) -> Self {
        MarkerLine {
            line: Line::new_noswap(a, b, is_broken),
            start_marker,
            end_marker,
        }
    }

    pub fn absolute_position(&self, cell: Cell) -> Self {
        MarkerLine {
            line: self.line.absolute_position(cell),
            start_marker: self.start_marker.clone(),
            end_marker: self.end_marker.clone(),
        }
    }

    pub fn align(&self) -> Self {
        MarkerLine {
            line: self.line.align(),
            start_marker: self.start_marker.clone(),
            end_marker: self.end_marker.clone(),
        }
    }

    pub fn scale(&self, scale: f32) -> Self {
        MarkerLine {
            line: self.line.scale(scale),
            start_marker: self.start_marker.clone(),
            end_marker: self.end_marker.clone(),
        }
    }

    pub(crate) fn can_merge_polygon(&self, polygon: &Polygon) -> bool {
        let poly_center = polygon.center();
        let distance_end_center = self.line.end.distance(&poly_center);
        let distance_start_center = self.line.start.distance(&poly_center);

        let threshold_length = self.line.heading().threshold_length();
        let is_close_start_point = distance_start_center < threshold_length;
        let is_close_end_point = distance_end_center < threshold_length;

        let can_connect_start = is_close_start_point && self.start_marker.is_none();
        let can_connect_end = is_close_end_point && self.end_marker.is_none();

        let is_same_direction = polygon
            .tags
            .iter()
            .any(|tag| tag.matched_direction(self.line.heading()));

        is_same_direction && (can_connect_start || can_connect_end)
    }

    /// merge this marker line to the polygon
    pub(crate) fn merge_polygon(&self, polygon: &Polygon) -> Option<Fragment> {
        if self.can_merge_polygon(polygon) {
            let marker = polygon.tags.get(0).map(|tag| tag.get_marker());
            let direction = polygon.tags.get(0).map(|tag| tag.direction());
            let poly_center = polygon.center();
            let distance_end_center = self.line.end.distance(&poly_center);
            let distance_start_center = self.line.start.distance(&poly_center);

            let threshold_length = self.line.heading().threshold_length();
            let is_close_start_point = distance_start_center < threshold_length;
            let is_close_end_point = distance_end_center < threshold_length;

            let start_marker = if is_close_start_point && self.start_marker.is_none() {
                marker.clone()
            } else {
                self.start_marker.clone()
            };
            let end_marker = if is_close_end_point && self.end_marker.is_none() {
                marker
            } else {
                self.end_marker.clone()
            };

            let extended_line = if is_close_start_point {
                self.line.extend_start(threshold_length / 2.0)
            } else if is_close_end_point {
                self.line.extend(threshold_length / 2.0)
            } else {
                panic!("There is no endpoint close to the polygon");
            };

            Some(marker_line(
                extended_line.start,
                extended_line.end,
                extended_line.is_broken,
                start_marker,
                end_marker,
            ))
        } else {
            None
        }
    }
}

impl Bounds for MarkerLine {
    fn bounds(&self) -> (Point, Point) {
        self.line.bounds()
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Marker::Arrow => write!(f, "arrow"),
            Marker::ClearArrow => write!(f, "clear_arrow"),
            Marker::Circle => write!(f, "circle"),
            Marker::Square => write!(f, "square"),
            Marker::Diamond => write!(f, "diamond"),
            Marker::OpenCircle => write!(f, "open_circle"),
            Marker::BigOpenCircle => write!(f, "big_open_circle"),
        }
    }
}

impl fmt::Display for MarkerLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {:?} {:?}",
            self.line, self.start_marker, self.end_marker
        )
    }
}

impl Into<Node<()>> for MarkerLine {
    fn into(self) -> Node<()> {
        let mut node: Node<()> = self.line.into();
        let mut classes = vec![];
        if let Some(start_marker) = self.start_marker {
            classes.push(class(format!("start_marked_{}", start_marker)));
        }
        if let Some(end_marker) = self.end_marker {
            classes.push(class(format!("end_marked_{}", end_marker)));
        }
        node.add_attributes(classes)
    }
}
