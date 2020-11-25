use crate::{
    buffer::{fragment_buffer::fragment::polygon::Polygon, Cell, Fragment},
    fragment::{marker_line, Bounds, Circle, Marker, MarkerLine},
    util, Direction, Point,
};
use ncollide2d::{
    math::Isometry,
    query::point_internal::point_query::PointQuery,
    shape::{Segment, Shape},
};
use std::{cmp::Ordering, fmt};

use crate::fragment::Arc;
use sauron::{html::attributes::*, svg, svg::attributes::*, Node};

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub is_broken: bool,
}

impl Line {
    /// creates a new line and reorder the points swapping the end points if necessary
    /// such that the start is the most top-left and end point is the most bottom-right
    pub(in crate) fn new(start: Point, end: Point, is_broken: bool) -> Self {
        let mut line = Line {
            start,
            end,
            is_broken,
        };
        line.sort_reorder_end_points();
        line
    }

    /// creates a new line, but don't reorder the points
    pub(in crate) fn new_noswap(
        start: Point,
        end: Point,
        is_broken: bool,
    ) -> Self {
        Line {
            start,
            end,
            is_broken,
        }
    }

    /// reorder the end points swap end points such that
    /// start < end
    pub(in crate) fn sort_reorder_end_points(&mut self) {
        if self.start > self.end {
            self.swap()
        }
    }

    fn swap(&mut self) {
        let tmp_start = self.start;
        self.start = self.end;
        self.end = tmp_start;
    }

    /// does this line can completely cover line a b?
    pub(in crate) fn overlaps(&self, a: Point, b: Point) -> bool {
        let segment = Segment::new(*self.start, *self.end);
        let identity = &Isometry::identity();
        segment.contains_point(identity, &a)
            && segment.contains_point(identity, &b)
    }

    fn contains_point(&self, p: Point) -> bool {
        let segment = Segment::new(*self.start, *self.end);
        let identity = &Isometry::identity();
        segment.contains_point(identity, &p)
    }

    fn touching_line(&self, other: &Self) -> bool {
        self.contains_point(other.start) || self.contains_point(other.end)
    }

    fn octant(&self) -> u8 {
        let mut dx = self.end.x - self.start.x;
        let mut dy = -(self.end.y * 2.0 - self.start.y * 2.0);

        let mut octant = 0;

        if dy < 0.0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0.0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2
        }

        if dx < dy {
            octant += 1
        }
        octant
    }

    //phi = atan(m1) - atan(m2);
    fn angle_rad(&self) -> f32 {
        let m1 = self.slope();
        0.0 - m1.atan()
    }

    fn angle_deg(&self) -> f32 {
        self.angle_rad().to_degrees()
    }

    fn slope(&self) -> f32 {
        (2.0 * self.end.y as f32 - 2.0 * self.start.y as f32)
            / (self.end.x as f32 - self.start.x as f32)
    }

    fn full_angle(&self) -> f32 {
        let angle = self.angle_deg().abs();
        match self.octant() {
            0..=1 => angle,
            2..=3 => 180.0 - angle,
            4..=5 => 180.0 + angle,
            6..=7 => 360.0 - angle,
            _ => angle,
        }
    }

    /// round angle closest to
    ///
    /// slash line is not really 60% but 63.435
    /// 63.435	0	63.435
    ///         180	116.565
    ///         180	243.435
    ///         360	296.565
    ///
    ///
    fn line_angle(&self) -> f32 {
        let angle = self.full_angle().round() as i32;
        match angle {
            0..=10 => 0.0,
            11..=50 => 63.435, //45.0,
            51..=80 => 63.435,
            81..=100 => 90.0,
            101..=130 => 116.565,
            131..=170 => 116.565, //135.0,
            171..=190 => 180.0,
            191..=230 => 243.435, //225.0,
            231..=260 => 243.435,
            261..=280 => 270.0,
            281..=310 => 296.565,
            311..=350 => 296.565, //315.0,
            351..=360 => 0.0,
            _ => 0.0,
        }
    }

    /// 0	0
    /// 45	45
    /// 63.435	63
    /// 90	90
    /// 116.565	117
    /// 135	135
    /// 180	180
    /// 225	225
    /// 243.435	243
    /// 270	270
    /// 296.565	297
    /// 315	315
    /// 360	360
    pub(crate) fn heading(&self) -> Direction {
        match self.line_angle().round() as i32 {
            0 => Direction::Right,
            45 => Direction::TopRight,
            63 => Direction::TopRight,
            90 => Direction::Top,
            117 => Direction::TopLeft,
            135 => Direction::TopLeft,
            180 => Direction::Left,
            225 => Direction::BottomLeft,
            243 => Direction::BottomLeft,
            270 => Direction::Bottom,
            297 => Direction::BottomRight,
            315 => Direction::BottomRight,
            _ => unreachable!(),
        }
    }

    /*
    /// if this line is colliean with the marker line and the
    pub(crate) fn can_merge_marker_line(&self, mline: &MarkerLine) -> bool {
        if self.can_merge(&mline.line) {
            // if there is no marker at the start
            if mline.start_marker.is_none() {
                self.end == mline.line.start || self.start == mline.line.start
            }
            // if there is no marker at the end
            else if mline.end_marker.is_none() {
                self.end == mline.line.end || self.start == mline.line.end
            } else {
                false
            }
        } else {
            false
        }
    }
    */

    #[allow(unused)]
    pub(crate) fn merge_marker_line(
        &self,
        mline: &MarkerLine,
    ) -> Option<Fragment> {
        if mline.start_marker.is_none() {
            if self.end == mline.line.start {
                Some(marker_line(
                    self.start,
                    mline.line.end,
                    mline.line.is_broken,
                    None,
                    mline.end_marker.clone(),
                ))
            } else if self.start == mline.line.start {
                Some(marker_line(
                    self.end,
                    mline.line.end,
                    mline.line.is_broken,
                    None,
                    mline.end_marker.clone(),
                ))
            } else {
                None
            }
        } else if mline.end_marker.is_none() {
            if self.end == mline.line.end {
                println!("success 3");
                Some(marker_line(
                    self.start,
                    mline.line.start,
                    mline.line.is_broken,
                    mline.start_marker.clone(),
                    None,
                ))
            } else if self.start == mline.line.end {
                println!("success 4");
                Some(marker_line(
                    self.end,
                    mline.line.start,
                    mline.line.is_broken,
                    mline.start_marker.clone(),
                    None,
                ))
            } else {
                None
            }
        } else {
            panic!("marker line should have at least one marker");
        }
    }

    pub(crate) fn is_touching_circle(&self, circle: &Circle) -> bool {
        let center = circle.center;
        let distance_end_center = self.end.distance(&center);
        let distance_start_center = self.start.distance(&center);

        let _threshold_length = self.heading().threshold_length();
        let is_close_start_point = distance_start_center < (circle.radius);
        let is_close_end_point = distance_end_center < (circle.radius);
        is_close_start_point || is_close_end_point
    }

    /// considering lines are sorted based on their start and end points
    /// the line should be touching each other
    /// and are collinear ( lies on the same line) can be test by computing the triangle area which
    /// should be equal to 0.
    /// therefore can merge
    pub(in crate) fn can_merge(&self, other: &Self) -> bool {
        self.is_touching(other)
            && util::is_collinear(&self.start, &self.end, &other.start)
            && util::is_collinear(&self.start, &self.end, &other.end)
    }

    /// check if this line and the other can merge
    /// returns None if it can not merge
    /// the merged line used the starting_point of self and the end_point of other
    pub(in crate) fn merge(&self, other: &Self) -> Option<Self> {
        if self.can_merge(other) {
            let start = std::cmp::min(self.start, other.start);
            let end = std::cmp::max(self.end, other.end);
            // when one of them is broken line, then everything will be broken line
            Some(Line::new(start, end, self.is_broken || other.is_broken))
        } else {
            None
        }
    }

    /// merge this line to the marker line
    pub(crate) fn merge_line_polygon(
        &self,
        polygon: &Polygon,
    ) -> Option<Fragment> {
        let poly_center = polygon.center();
        let distance_end_center = self.end.distance(&poly_center);
        let distance_start_center = self.start.distance(&poly_center);

        let line_heading = self.heading();

        let threshold_length = line_heading.threshold_length();
        let is_close_start_point = distance_start_center < threshold_length;
        let is_close_end_point = distance_end_center < threshold_length;

        let is_same_direction = polygon.matched_direction(line_heading);

        let is_opposite_direction =
            polygon.matched_direction(line_heading.opposite());

        let can_merge = (is_same_direction || is_opposite_direction)
            && (is_close_start_point || is_close_end_point);

        if can_merge {
            let new_line = if is_close_end_point {
                Line::new_noswap(self.start, self.end, self.is_broken)
            } else if is_close_start_point {
                // if close to the start, swap the end points of the line
                Line::new_noswap(self.end, self.start, self.is_broken)
            } else {
                panic!("There is no endpoint of the line is that close to the arrow");
            };
            let extended_line = new_line.extend(threshold_length);

            Some(marker_line(
                extended_line.start,
                extended_line.end,
                extended_line.is_broken,
                None,
                polygon.get_marker(),
            ))
        } else {
            None
        }
    }

    pub(crate) fn merge_circle(&self, circle: &Circle) -> Option<Fragment> {
        let distance_end_center = self.end.distance(&circle.center);
        let distance_start_center = self.start.distance(&circle.center);

        let threshold_length = self.heading().threshold_length();
        let is_close_start_point =
            distance_start_center <= threshold_length * 0.75;
        let is_close_end_point = distance_end_center <= threshold_length * 0.75;

        let can_merge = circle.radius <= Cell::unit(3)
            && (is_close_start_point || is_close_end_point);

        if can_merge {
            let marker = if circle.is_filled {
                Some(Marker::Circle)
            } else if circle.radius >= Cell::unit(2) {
                Some(Marker::BigOpenCircle)
            } else {
                Some(Marker::OpenCircle)
            };
            let new_line = if is_close_end_point {
                Line::new_noswap(self.start, circle.center, self.is_broken)
            } else if is_close_start_point {
                // if close to the start, swap the end points of the line
                Line::new_noswap(self.end, circle.center, self.is_broken)
            } else {
                panic!("There is no endpoint of the line is that close to the arrow");
            };

            let marker_line = marker_line(
                new_line.start,
                new_line.end,
                new_line.is_broken,
                None,
                marker,
            );
            Some(marker_line)
        } else {
            None
        }
    }

    /// check to see if any of the line endpoints is touching.
    /// this will be used to group lines together
    /// This does not check if the lines are intersecting
    pub(in crate) fn is_touching(&self, other: &Self) -> bool {
        self.touching_line(other) || other.touching_line(self)
    }

    pub(in crate) fn has_endpoint(&self, p: Point) -> bool {
        self.start == p || self.end == p
    }

    pub(in crate) fn is_touching_arc(&self, other: &Arc) -> bool {
        self.start == other.start
            || self.end == other.end
            || self.start == other.end
            || self.end == other.start
    }

    /// check if this a horizontal line
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    /// check if this is a vertical line
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    /// check if 2 lines are parallel in axis align box
    /// for the purpose of promoting lines into rect
    pub(in crate) fn is_aabb_parallel(&self, other: &Self) -> bool {
        (self.is_horizontal()
            && other.is_horizontal()
            && self.start.x == other.start.x
            && self.end.x == other.end.x)
            || (self.is_vertical()
                && other.is_vertical()
                && self.start.y == other.start.y
                && self.end.y == other.end.y)
    }

    /// check if 2 lines are perpendicular in axis align box only
    /// for the purpose of promoting lines into rect
    pub(in crate) fn is_aabb_perpendicular(&self, other: &Self) -> bool {
        (self.is_horizontal() && other.is_vertical())
            || (self.is_vertical() && other.is_horizontal())
    }

    /// check if 2 lines are touching and aabb perpendicular at the same time
    pub(in crate) fn is_touching_aabb_perpendicular(
        &self,
        other: &Self,
    ) -> bool {
        self.is_touching(other) && self.is_aabb_perpendicular(other)
    }

    /// recompute the line with start and end point offset by the cell
    /// location
    pub(in crate) fn absolute_position(&self, cell: Cell) -> Self {
        Line {
            start: cell.absolute_position(self.start),
            end: cell.absolute_position(self.end),
            is_broken: self.is_broken,
        }
    }

    pub(in crate) fn scale(&self, scale: f32) -> Self {
        Line {
            start: self.start.scale(scale),
            end: self.end.scale(scale),
            is_broken: self.is_broken,
        }
    }

    pub(crate) fn is_broken(&self) -> bool {
        self.is_broken
    }

    pub(crate) fn localize(&self, cell: Cell) -> Self {
        Line {
            start: cell.localize_point(self.start),
            end: cell.localize_point(self.end),
            is_broken: self.is_broken,
        }
    }

    pub(crate) fn align(&self) -> Self {
        Line {
            start: self.start.align(),
            end: self.end.align(),
            is_broken: self.is_broken,
        }
    }

    /// extend the line by a length added to the end point
    /// https://stackoverflow.com/questions/7740507/extend-a-line-segment-a-specific-distance#7741655
    pub fn extend(&self, length: f32) -> Self {
        let d = self.start.distance(&self.end);
        let cx = self.end.x + (self.end.x - self.start.x) / d * length;
        let cy = self.end.y + (self.end.y - self.start.y) / d * length;
        Line::new_noswap(self.start, Point::new(cx, cy), self.is_broken)
    }

    /// extend but on the oposite direction
    /// TODO: This implementation is hacky
    pub fn extend_start(&self, length: f32) -> Self {
        let mut tmp_line = self.clone();
        tmp_line.swap();
        let mut new_line = tmp_line.extend(length);
        new_line.swap();
        new_line
    }
}

impl Bounds for Line {
    fn bounds(&self) -> (Point, Point) {
        let aabb = Segment::new(*self.start, *self.end).local_aabb();
        (Point::from(*aabb.mins()), Point::from(*aabb.maxs()))
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L {} {} {}", self.start, self.end, self.is_broken)
    }
}

impl<MSG> Into<Node<MSG>> for Line {
    fn into(self) -> Node<MSG> {
        svg::tags::line(
            vec![
                x1(self.start.x),
                y1(self.start.y),
                x2(self.end.x),
                y2(self.end.y),
                classes_flag([
                    ("broken", self.is_broken),
                    ("solid", !self.is_broken),
                ]),
            ],
            vec![],
        )
    }
}

impl Eq for Line {}

/// This is needed since this struct contains f32 which rust doesn't provide Eq implementation
impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then(self.end.cmp(&other.end))
            .then(self.is_broken.cmp(&other.is_broken))
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::fragment_buffer::fragment::polygon::PolygonTag;
    use crate::buffer::CellGrid;

    #[test]
    fn test_extend_line() {
        let line1 = Line::new_noswap(
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            false,
        );
        let extended = line1.extend(1.0);
        assert_eq!(
            extended,
            Line::new(Point::new(0.0, 0.0), Point::new(11.0, 0.0), false)
        );
        let extended2 = line1.extend(2.0);
        assert_eq!(
            extended2,
            Line::new(Point::new(0.0, 0.0), Point::new(12.0, 0.0), false)
        );
    }

    #[test]
    fn test_extend_line_start() {
        let line1 = Line::new_noswap(
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            false,
        );
        let extended = line1.extend_start(1.0);
        assert_eq!(
            extended,
            Line::new(Point::new(-1.0, 0.0), Point::new(10.0, 0.0), false)
        );
        let extended2 = line1.extend_start(2.0);
        assert_eq!(
            extended2,
            Line::new(Point::new(-2.0, 0.0), Point::new(10.0, 0.0), false)
        );
    }

    #[test]
    fn test_extend_line_vertical() {
        let line1 = Line::new_noswap(
            Point::new(0.0, 0.0),
            Point::new(0.0, 10.0),
            false,
        );
        let extended = line1.extend(1.0);
        assert_eq!(
            extended,
            Line::new(Point::new(0.0, 0.0), Point::new(0.0, 11.0), false)
        );
        let extended2 = line1.extend(2.0);
        assert_eq!(
            extended2,
            Line::new(Point::new(0.0, 0.0), Point::new(0.0, 12.0), false)
        );
    }

    #[test]
    fn line_merge() {
        let line1 =
            Line::new(Point::new(4.0, 0.0), Point::new(2.0, 4.0), false);
        let line2 =
            Line::new(Point::new(2.0, 4.0), Point::new(1.0, 6.0), false);
        assert!(line1.is_touching(&line2));
        assert!(line2.is_touching(&line1));
        assert!(util::is_collinear(&line1.start, &line1.end, &line2.start));
        assert!(util::is_collinear(&line2.start, &line2.end, &line1.end));
        let area1 = ncollide2d::utils::triangle_area(
            &line1.start,
            &line1.end,
            &line2.start,
        );
        println!("area1: {}", area1);
        let area2 = ncollide2d::utils::triangle_area(
            &line1.start,
            &line1.end,
            &line2.end,
        );
        println!("area2: {}", area2);
        assert!(line1.can_merge(&line2));
    }

    #[test]
    fn is_touching_arrow() {
        let m = CellGrid::m();
        let end = Cell::new(10, 0).o();
        let p1 = Cell::new(11, 0).f();
        let p2 = Cell::new(11, 0).o();
        let p3 = Cell::new(11, 0).p();

        let polygon =
            Polygon::new(vec![p1, p2, p3], false, vec![PolygonTag::ArrowRight]);

        let line = Line::new(m, end, false);
        assert!(line.merge_line_polygon(&polygon).is_some());
    }

    #[test]
    fn test_angle() {
        let m = CellGrid::m();
        let k = CellGrid::k();
        let c = CellGrid::c();
        let o = CellGrid::o();
        let e = CellGrid::e();
        let a = CellGrid::a();
        let y = CellGrid::y();
        let u = CellGrid::u();

        assert_eq!(0.0, 0.0f32.atan());

        let line = Line::new(c, m, false);
        assert_eq!(line.line_angle(), 270.0);

        let line2 = Line::new(m, o, false);
        assert_eq!(line2.line_angle(), 0.0);

        let line3 = Line::new(a, y, false);
        assert_eq!(line3.line_angle(), 296.565);

        let line4 = Line::new(k, o, false);
        assert_eq!(line4.line_angle(), 0.0);

        let line6 = Line::new(u, e, false);
        assert_eq!(line6.line_angle(), 243.435);

        let line5 = Line::new(e, u, false);
        assert_eq!(line5.line_angle(), 243.435);
    }

    #[test]
    fn test_bounds() {
        let d = CellGrid::d();
        let e = CellGrid::e();
        let line = Line::new(e, d, false);
        assert_eq!(line.bounds(), (d, e));
    }

    #[test]
    fn test_merge() {
        let a = CellGrid::a();
        let b = CellGrid::b();
        let c = CellGrid::c();
        let d = CellGrid::d();
        assert!(Line::new(a, b, false).can_merge(&Line::new(b, c, false)));
        assert!(Line::new(b, c, false).can_merge(&Line::new(b, c, false)));
        assert!(Line::new(b, c, false).can_merge(&Line::new(c, b, false)));
        assert!(Line::new(b, c, false).can_merge(&Line::new(c, d, false)));
    }

    #[test]
    fn test_merge_kmo() {
        let k = CellGrid::k();
        let m = CellGrid::m();
        let o = CellGrid::o();
        assert!(Line::new(k, m, false).can_merge(&Line::new(m, o, false)));
        assert_eq!(
            Some(Line::new(k, o, false)),
            Line::new(k, m, false).merge(&Line::new(m, o, false))
        );
    }

    #[test]
    fn test_merge_cmw() {
        let c = CellGrid::c();
        let m = CellGrid::m();
        let w = CellGrid::w();
        assert!(Line::new(c, m, false).can_merge(&Line::new(m, w, false)));
        assert_eq!(
            Some(Line::new(c, w, false)),
            Line::new(c, m, false).merge(&Line::new(m, w, false))
        );
    }
}
