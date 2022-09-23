use crate::Merge;
use crate::{buffer::Cell, map::unicode_map::FRAGMENTS_UNICODE, Point};
pub use crate::{Property, Settings, Signal};
pub use arc::Arc;
pub use circle::Circle;
pub use line::Line;
pub use marker_line::{Marker, MarkerLine};
use parry2d::bounding_volume::BoundingVolume;
use parry2d::query::PointQuery;
use parry2d::shape::ConvexPolygon;
use parry2d::{
    bounding_volume::AABB,
    math::Isometry,
    query::intersection_test,
    shape::{Polyline, Segment, Shape},
};
pub use polygon::{Polygon, PolygonTag};
pub use rect::Rect;
use sauron::Node;
use std::{cmp::Ordering, fmt};
pub use text::{CellText, Text};

mod arc;
mod circle;
mod line;
mod marker_line;
mod polygon;
mod rect;
mod text;

/// ```ignore
///      0 1 2 3 4           B C D
///     0┌─┬─┬─┬─┐        A┌─┬─┬─┬─┐E
///     1├─┼─┼─┼─┤         │ │ │ │ │
///     2├─┼─┼─┼─┤        F├─G─H─I─┤J
///     3├─┼─┼─┼─┤         │ │ │ │ │
///     4├─┼─┼─┼─┤        K├─L─M─N─┤O
///     5├─┼─┼─┼─┤         │ │ │ │ │
///     6├─┼─┼─┼─┤        P├─Q─R─S─┤T
///     7├─┼─┼─┼─┤         │ │ │ │ │
///     8└─┴─┴─┴─┘        U└─┴─┴─┴─┘Y
/// ```                      V W X

#[derive(Debug, Clone)]
pub enum Fragment {
    Line(Line),
    MarkerLine(MarkerLine),
    Circle(Circle),
    Arc(Arc),
    Polygon(Polygon),
    Rect(Rect),
    // cell base
    CellText(CellText),
    // point base
    Text(Text),
}

/// get the boundary of a fragment
/// this is used for sorting the fragments
/// in a consistent sorted order
pub trait Bounds {
    fn bounds(&self) -> (Point, Point);

    fn mins(&self) -> Point {
        self.bounds().0
    }

    fn maxs(&self) -> Point {
        self.bounds().1
    }
}

impl Fragment {
    /// get the character that matches the shape present on this cell
    pub fn match_unicode(fragments: &Vec<Self>) -> Option<char> {
        let mut sorted_shapes = fragments.clone();
        sorted_shapes.sort();
        //assert!(sorted_shapes.is_sorted());
        FRAGMENTS_UNICODE.get(&sorted_shapes).copied()
    }

    /// check to see if this fragment is a line and that line
    /// can completely overlap line a b
    /// TODO: only checking for solid, also expose API for broken
    pub(crate) fn line_overlap(&self, a: Point, b: Point) -> bool {
        match self {
            Fragment::Line(line) => line.overlaps(a, b),
            _ => false,
        }
    }

    /// check if any of the fragment end point is touching p
    pub(crate) fn has_endpoint(&self, p: Point) -> bool {
        match self {
            Fragment::Line(line) => line.has_endpoint(p),
            Fragment::Arc(arc) => arc.has_endpoint(p),
            _ => false,
        }
    }

    /// check to see if this fragment is an arc
    /// overlaps from point a to b
    pub(crate) fn arcs_to(&self, a: Point, b: Point) -> bool {
        match self {
            Fragment::Arc(arc) => arc.arcs_to(a, b),
            _ => false,
        }
    }

    /// check whether the other fragment is can be fit in this fragment
    pub(crate) fn can_fit(&self, other: &Self) -> bool {
        let (tl, br) = self.bounds();
        let (other_tl, other_br) = other.bounds();
        tl.x <= other_tl.x
            && tl.y <= other_tl.y
            && br.x >= other_br.x
            && br.y >= other_br.y
    }

    /// are lines axis align and parallel
    pub(crate) fn is_aabb_parallel(&self, other: &Self) -> bool {
        match (self, other) {
            (Fragment::Line(line), Fragment::Line(other)) => {
                line.is_aabb_parallel(other)
            }
            (_, _) => false,
        }
    }

    #[allow(unused)]
    pub(crate) fn is_aabb_perpendicular(&self, other: &Self) -> bool {
        match (self, other) {
            (Fragment::Line(line), Fragment::Line(other)) => {
                line.is_aabb_perpendicular(other)
            }
            (_, _) => false,
        }
    }

    /// check if this fragment is touching the other fragment
    /// therefore can be in a group together
    pub(crate) fn is_contacting(&self, other: &Self) -> bool {
        match self {
            Fragment::Line(line) => match other {
                Fragment::Line(other) => line.is_touching(other),
                Fragment::Arc(other_arc) => line.is_touching_arc(other_arc),
                Fragment::Polygon(polygon) => {
                    line.merge_line_polygon(polygon).is_some()
                }
                Fragment::Circle(circle) => line.is_touching_circle(circle),
                _ => false,
            },
            Fragment::Polygon(polygon) => match other {
                Fragment::Line(other) => {
                    other.merge_line_polygon(polygon).is_some()
                }
                _ => false,
            },
            Fragment::Arc(arc) => match other {
                Fragment::Arc(other_arc) => arc.is_touching(other_arc),
                Fragment::Line(other_line) => other_line.is_touching_arc(arc),
                _ => false,
            },
            Fragment::Circle(circle) => match other {
                Fragment::Line(other) => other.is_touching_circle(circle),
                _ => false,
            },
            Fragment::CellText(ctext) => match other {
                Fragment::CellText(other_ctext) => {
                    ctext.is_contacting(other_ctext)
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub fn hit(&self, start: Point, end: Point) -> bool {
        self.is_intersecting(&AABB::new(*start, *end))
    }

    /// check if this fragment is intersecting with this bounding box
    /// Note: if intersection logic requires testing the solid shape inside the polygon
    /// use the ConvexPolygon of each shape instead of Polyline
    pub fn is_intersecting(&self, bbox: &AABB) -> bool {
        let points = vec![
            *Point::new(bbox.mins.x, bbox.mins.y),
            *Point::new(bbox.maxs.x, bbox.mins.y),
            *Point::new(bbox.maxs.x, bbox.maxs.y),
            *Point::new(bbox.mins.x, bbox.maxs.y),
        ];
        let bbox = Polyline::new(points, None);
        let identity = Isometry::identity();
        match self {
            Fragment::Line(line) => {
                let segment: Segment = line.clone().into();
                let res =
                    intersection_test(&identity, &segment, &identity, &bbox)
                        .expect("must pass");
                println!("res: {}", res);
                res
            }
            Fragment::Rect(rect) => {
                let polyline: Polyline = rect.clone().into();
                intersection_test(&identity, &polyline, &identity, &bbox)
                    .expect("must pass")
            }
            Fragment::Circle(circle) => {
                let polyline: Polyline = circle.clone().into();
                intersection_test(&identity, &polyline, &identity, &bbox)
                    .expect("must pass")
            }
            _ => false,
        }
    }

    /// check if this fragment can be contain in the specified bounding box `bbox`
    pub fn is_inside(&self, bbox: &AABB) -> bool {
        let (start, end) = self.bounds();
        let frag_bound = AABB::new(*start, *end);
        bbox.contains(&frag_bound)
    }

    /// recompute the end points of this fragment
    /// offset by the cell location
    pub fn absolute_position(&self, cell: Cell) -> Self {
        match self {
            Fragment::Line(line) => {
                Fragment::Line(line.absolute_position(cell))
            }
            Fragment::MarkerLine(marker_line) => {
                Fragment::MarkerLine(marker_line.absolute_position(cell))
            }
            Fragment::Circle(circle) => {
                Fragment::Circle(circle.absolute_position(cell))
            }
            Fragment::Arc(arc) => Fragment::Arc(arc.absolute_position(cell)),
            Fragment::Polygon(polygon) => {
                Fragment::Polygon(polygon.absolute_position(cell))
            }
            Fragment::Rect(rect) => {
                Fragment::Rect(rect.absolute_position(cell))
            }
            Fragment::Text(text) => {
                Fragment::Text(text.absolute_position(cell))
            }
            Fragment::CellText(ctext) => {
                Fragment::CellText(ctext.absolute_position(cell))
            }
        }
    }

    /// enlarge or shrink this fragment at scale
    pub fn scale(&self, scale: f32) -> Self {
        match self {
            Fragment::Line(line) => Fragment::Line(line.scale(scale)),
            Fragment::MarkerLine(marker_line) => {
                Fragment::MarkerLine(marker_line.scale(scale))
            }
            Fragment::Circle(circle) => Fragment::Circle(circle.scale(scale)),
            Fragment::Arc(arc) => Fragment::Arc(arc.scale(scale)),
            Fragment::Polygon(polygon) => {
                Fragment::Polygon(polygon.scale(scale))
            }
            Fragment::Rect(rect) => Fragment::Rect(rect.scale(scale)),
            Fragment::Text(text) => Fragment::Text(text.scale(scale)),
            // the CellText is converted into text fragment first, then scaled
            Fragment::CellText(ctext) => {
                Fragment::Text(Into::<Text>::into(ctext.clone()).scale(scale))
            }
        }
    }

    pub fn align(&self) -> Self {
        match self {
            Fragment::Line(line) => Fragment::Line(line.align()),
            Fragment::MarkerLine(marker_line) => {
                Fragment::MarkerLine(marker_line.align())
            }
            Fragment::Circle(circle) => Fragment::Circle(circle.clone()),
            Fragment::Arc(arc) => Fragment::Arc(arc.clone()),
            Fragment::Polygon(polygon) => Fragment::Polygon(polygon.clone()),
            Fragment::Rect(rect) => Fragment::Rect(rect.clone()),
            Fragment::Text(text) => Fragment::Text(text.clone()),
            // the CellText is converted into text fragment first, then scaled
            Fragment::CellText(ctext) => {
                Fragment::Text(Into::<Text>::into(ctext.clone()))
            }
        }
    }

    /// rank for additional cmp comparison
    /// so as not match different shapes if their lower bounds
    /// and upper bounds matched
    fn rank(&self) -> u8 {
        match self {
            Fragment::Line(_) => 10,
            Fragment::MarkerLine(_) => 20,
            Fragment::Circle(_) => 30,
            Fragment::Arc(_) => 40,
            Fragment::Polygon { .. } => 50,
            Fragment::Rect(_) => 60,
            Fragment::Text(_) => 70,
            Fragment::CellText(_) => 80,
        }
    }

    pub fn as_line(&self) -> Option<&Line> {
        match self {
            Fragment::Line(ref line) => Some(line),
            _ => None,
        }
    }

    pub fn as_rect(&self) -> Option<&Rect> {
        match self {
            Fragment::Rect(ref rect) => Some(rect),
            _ => None,
        }
    }

    pub fn as_polygon(&self) -> Option<&Polygon> {
        match self {
            Fragment::Polygon(polygon) => Some(polygon),
            _ => None,
        }
    }

    pub fn as_arc(&self) -> Option<&Arc> {
        match self {
            Fragment::Arc(ref arc) => Some(arc),
            _ => None,
        }
    }

    pub fn as_cell_text(&self) -> Option<&CellText> {
        match self {
            Fragment::CellText(ref ctext) => Some(ctext),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&Text> {
        match self {
            Fragment::Text(ref text) => Some(text),
            _ => None,
        }
    }

    /// if this is a cell text and is wrapped in braces then it is a css
    /// tag for the container
    pub fn as_css_tag(&self) -> Vec<String> {
        let input_text: Option<&str> =
            if let Some(cell_text) = self.as_cell_text() {
                Some(&cell_text.content)
            } else if let Some(text) = self.as_text() {
                Some(&text.text)
            } else {
                None
            };

        if let Some(input_text) = input_text {
            if let Ok(tags) = crate::util::parser::parse_css_tag(input_text) {
                tags
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    pub fn as_circle(&self) -> Option<&Circle> {
        match self {
            Fragment::Circle(circle) => Some(circle),
            _ => None,
        }
    }

    pub fn is_circle(&self) -> bool {
        match self {
            Fragment::Circle(_) => true,
            _ => false,
        }
    }

    pub fn is_rect(&self) -> bool {
        match self {
            Fragment::Rect(_) => true,
            _ => false,
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            Fragment::Text(_) => true,
            _ => false,
        }
    }

    pub fn is_cell_text(&self) -> bool {
        match self {
            Fragment::CellText(_) => true,
            _ => false,
        }
    }

    pub fn is_broken(&self) -> bool {
        match self {
            Fragment::Line(line) => line.is_broken(),
            Fragment::Rect(rect) => rect.is_broken(),
            _ => false,
        }
    }
}

impl Merge for Fragment {
    /// merge this fragment to the other fragment if it is possible
    /// returns None if the fragment can not be merge
    fn merge(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            // line and line
            (Fragment::Line(line), Fragment::Line(other_line)) => {
                line.merge(other_line).map(Fragment::Line)
            }

            // line and polygon
            (Fragment::Line(line), Fragment::Polygon(polygon)) => {
                line.merge_line_polygon(polygon)
            }

            // polygon and line
            (Fragment::Polygon(polygon), Fragment::Line(line)) => {
                line.merge_line_polygon(polygon)
            }

            // line and marker_line
            (Fragment::Line(line), Fragment::MarkerLine(mline)) => {
                line.merge_marker_line(mline)
            }
            // marker_line and line
            (Fragment::MarkerLine(mline), Fragment::Line(line)) => {
                line.merge_marker_line(mline)
            }
            (Fragment::MarkerLine(mline), Fragment::Polygon(polygon)) => {
                mline.merge_polygon(polygon)
            }
            // line and circle
            (Fragment::Line(line), Fragment::Circle(circle)) => {
                line.merge_circle(circle)
            }

            // circle and line
            (Fragment::Circle(circle), Fragment::Line(line)) => {
                line.merge_circle(circle)
            }
            // cell_text and cell_text
            (Fragment::CellText(ctext), Fragment::CellText(other_ctext)) => {
                ctext.merge(other_ctext).map(Fragment::CellText)
            }
            _ => None,
        }
    }
}

impl Bounds for Fragment {
    fn bounds(&self) -> (Point, Point) {
        match self {
            Fragment::Line(line) => line.bounds(),
            //TODO: also add the bounds for both of the markers
            Fragment::MarkerLine(mline) => mline.bounds(),
            Fragment::Circle(circle) => circle.bounds(),
            Fragment::Arc(arc) => arc.bounds(),
            Fragment::Polygon(polygon) => polygon.bounds(),
            Fragment::Rect(rect) => rect.bounds(),
            Fragment::Text(text) => text.bounds(),
            Fragment::CellText(ctext) => ctext.bounds(),
        }
    }
}

impl<MSG> Into<Node<MSG>> for Fragment {
    fn into(self) -> Node<MSG> {
        match self {
            Fragment::Line(line) => line.into(),
            Fragment::MarkerLine(marker_line) => marker_line.into(),
            Fragment::Circle(circle) => circle.into(),
            Fragment::Arc(arc) => arc.into(),
            Fragment::Polygon(polygon) => polygon.into(),
            Fragment::Rect(rect) => rect.into(),
            Fragment::Text(text) => text.into(),
            Fragment::CellText(ctext) => ctext.into(),
        }
    }
}

impl fmt::Display for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fragment::Line(line) => write!(f, "{}", line),
            Fragment::MarkerLine(marker_line) => write!(f, "{}", marker_line),
            Fragment::Circle(circle) => write!(f, "{}", circle),
            Fragment::Arc(arc) => write!(f, "{}", arc),
            Fragment::Polygon(polygon) => write!(f, "{}", polygon),
            Fragment::Rect(rect) => write!(f, "{}", rect),
            Fragment::Text(text) => write!(f, "{}", text),
            Fragment::CellText(ctext) => write!(f, "{}", ctext),
        }
    }
}

pub fn line(a: Point, b: Point) -> Fragment {
    Fragment::Line(Line::new(a, b, false))
}

pub fn marker_line(
    a: Point,
    b: Point,
    is_broken: bool,
    start_marker: Option<Marker>,
    end_marker: Option<Marker>,
) -> Fragment {
    Fragment::MarkerLine(MarkerLine::new(
        a,
        b,
        is_broken,
        start_marker,
        end_marker,
    ))
}

pub fn broken_line(a: Point, b: Point) -> Fragment {
    Fragment::Line(Line::new(a, b, true))
}

pub fn circle(c: Point, r: f32, is_filled: bool) -> Fragment {
    Fragment::Circle(Circle::new(c, r, is_filled))
}

pub fn arc(a: Point, b: Point, r: f32) -> Fragment {
    Fragment::Arc(Arc::new(a, b, r))
}

pub fn arc_with_sweep(
    a: Point,
    b: Point,
    r: f32,
    sweep_flag: bool,
) -> Fragment {
    Fragment::Arc(Arc::new_with_sweep(a, b, r, sweep_flag))
}

pub fn polygon(
    points: Vec<Point>,
    is_filled: bool,
    tags: Vec<PolygonTag>,
) -> Fragment {
    Fragment::Polygon(Polygon::new(points, is_filled, tags))
}

pub fn rect(
    start: Point,
    end: Point,
    is_filled: bool,
    is_broken: bool,
) -> Fragment {
    Fragment::Rect(Rect::new(start, end, is_filled, is_broken))
}

pub fn rounded_rect(
    start: Point,
    end: Point,
    is_filled: bool,
    radius: f32,
    is_broken: bool,
) -> Fragment {
    Fragment::Rect(Rect::rounded_new(start, end, is_filled, radius, is_broken))
}

/// creates a cell text meant to be stored
/// in a cell of a fragment_buffer,
pub fn cell_text(ch: char) -> Fragment {
    Fragment::CellText(CellText::new(Cell::new(0, 0), ch.to_string()))
}

pub fn text(s: String) -> Fragment {
    Fragment::Text(Text::new(Point::new(0.0, 0.0), s))
}

pub fn lines_to_fragments(lines: Vec<Line>) -> Vec<Fragment> {
    lines.into_iter().map(Fragment::Line).collect()
}

impl From<Line> for Fragment {
    fn from(line: Line) -> Self {
        Fragment::Line(line)
    }
}

impl From<Rect> for Fragment {
    fn from(rect: Rect) -> Self {
        Fragment::Rect(rect)
    }
}

impl From<Text> for Fragment {
    fn from(text: Text) -> Self {
        Fragment::Text(text)
    }
}

impl Into<Fragment> for CellText {
    fn into(self) -> Fragment {
        Fragment::CellText(self)
    }
}

impl From<Circle> for Fragment {
    fn from(circle: Circle) -> Self {
        Fragment::Circle(circle)
    }
}

impl From<Arc> for Fragment {
    fn from(arc: Arc) -> Self {
        Fragment::Arc(arc)
    }
}

impl Eq for Fragment {}

impl Ord for Fragment {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Fragment::Line(line), Fragment::Line(other)) => line.cmp(other),
            (Fragment::Arc(arc), Fragment::Arc(other)) => arc.cmp(other),
            (Fragment::Circle(circle), Fragment::Circle(other)) => {
                circle.cmp(other)
            }
            (Fragment::Polygon(polygon), Fragment::Polygon(other_polygon)) => {
                polygon.cmp(other_polygon)
            } //Note: the tags are not compared here
            (Fragment::Rect(rect), Fragment::Rect(other)) => rect.cmp(other),
            (Fragment::Text(text), Fragment::Text(other)) => text.cmp(other),
            (Fragment::CellText(ctext), Fragment::CellText(other)) => {
                ctext.cmp(other)
            }
            _ => self
                .mins()
                .cmp(&other.mins())
                .then(self.maxs().cmp(&other.maxs()))
                .then(self.rank().cmp(&other.rank())),
        }
    }
}

impl PartialOrd for Fragment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Fragment {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::CellGrid;

    #[test]
    fn test_can_fit() {
        let rect1 =
            rect(Point::new(0.0, 0.0), Point::new(10.0, 10.0), false, false);
        let rect2 =
            rect(Point::new(1.0, 1.0), Point::new(9.0, 9.0), false, false);
        let text1 = Fragment::CellText(CellText::new(
            Cell::new(2, 2),
            "{doc}".to_string(),
        ));
        let text2 = Fragment::CellText(CellText::new(
            Cell::new(2, 2),
            "This is a hello world!".to_string(),
        ));
        assert!(rect1.can_fit(&rect2));
        assert!(rect1.can_fit(&text1));
        assert!(rect2.can_fit(&text1));
        assert!(!rect1.can_fit(&text2));
        assert!(!rect2.can_fit(&text2));
    }

    #[test]
    fn test_recursive_merge() {
        let k = CellGrid::k();
        let o = CellGrid::o();
        let c = CellGrid::c();
        let m = CellGrid::m();
        let w = CellGrid::w();
        let fragments1 = vec![line(k, m), line(m, o), line(c, m), line(m, w)];
        println!("before merged:");
        for frag in &fragments1 {
            println!("{}", frag);
        }
        let mut expected = vec![line(k, o), line(c, w)];
        expected.sort();
        let mut merged_fragments1 = Fragment::merge_recursive(fragments1);
        merged_fragments1.sort();
        assert_eq!(merged_fragments1.len(), 2);
        println!("after merged:");
        for frag in &merged_fragments1 {
            println!("{}", frag);
        }
        assert_eq!(merged_fragments1, expected);
    }

    #[test]
    fn test_can_merge() {
        let k = CellGrid::k();
        let l = CellGrid::l();
        let m = CellGrid::m();
        let n = CellGrid::n();
        let o = CellGrid::o();
        let j = CellGrid::j();

        assert!(line(k, m).merge(&line(m, o),).is_some()); // collinear and connected
        assert!(line(k, l).merge(&line(n, o),).is_none()); //collinear but not connected
        assert!(line(k, o).merge(&line(o, j),).is_none()); // connected but not collinear
    }

    #[test]
    fn merge_line_and_circle() {
        let a = CellGrid::a();
        let m = CellGrid::m();
        let y = CellGrid::y();

        let circle = circle(m, Cell::unit(2), false);

        let circle = circle.absolute_position(Cell::new(0, 0));
        let diagonal: Fragment = Line::new_noswap(a, y, false).into();
        let diagonal = diagonal.absolute_position(Cell::new(1, 1));

        println!("circle: {:#?}", circle);
        println!("diagonal: {:#?}", diagonal);

        let merged = circle.merge(&diagonal);

        let expected = marker_line(
            Point::new(2.0, 4.0),
            Point::new(0.5, 1.0),
            false,
            None,
            Some(Marker::BigOpenCircle),
        );
        assert_eq!(Some(expected), merged);
    }

    #[test]
    fn line_overlaps() {
        let line = Line::new(CellGrid::a(), CellGrid::b(), false);
        println!("line: {}", line);
        assert!(line.overlaps(CellGrid::a(), CellGrid::b()));
        assert!(!line.overlaps(CellGrid::d(), CellGrid::e()));
    }

    #[test]
    fn line_overlap2() {
        let ko = line(CellGrid::k(), CellGrid::o());
        assert!(ko.line_overlap(CellGrid::n(), CellGrid::o()));
        assert!(ko.line_overlap(CellGrid::n(), CellGrid::o()));
    }

    #[test]
    fn line_overlap3() {
        let km = line(CellGrid::k(), CellGrid::m());
        assert!(km.line_overlap(CellGrid::l(), CellGrid::m()));
        assert!(!km.line_overlap(CellGrid::n(), CellGrid::o()));
        assert!(!km.line_overlap(CellGrid::m(), CellGrid::o()));
        assert!(!km.line_overlap(CellGrid::l(), CellGrid::o()));
    }

    #[test]
    fn equal_lines() {
        assert_eq!(
            line(CellGrid::a(), CellGrid::y()),
            line(CellGrid::y(), CellGrid::a())
        );
        assert_eq!(
            line(CellGrid::k(), CellGrid::o()),
            line(CellGrid::o(), CellGrid::k())
        );
    }

    #[test]
    fn test_sort_lines() {
        let mut lines1 = vec![
            line(CellGrid::a(), CellGrid::e()),
            line(CellGrid::u(), CellGrid::y()),
            line(CellGrid::k(), CellGrid::o()),
        ];

        let sorted = vec![
            line(CellGrid::a(), CellGrid::e()),
            line(CellGrid::k(), CellGrid::o()),
            line(CellGrid::u(), CellGrid::y()),
        ];

        assert_ne!(lines1, sorted);
        lines1.sort();
        assert_eq!(lines1, sorted);
    }

    #[test]
    fn test_sort_lines2() {
        let mut lines1 = vec![
            line(CellGrid::m(), CellGrid::e()),
            line(CellGrid::a(), CellGrid::y()),
            line(CellGrid::k(), CellGrid::o()),
        ];

        let sorted = vec![
            line(CellGrid::a(), CellGrid::y()),
            line(CellGrid::m(), CellGrid::e()),
            line(CellGrid::k(), CellGrid::o()),
        ];
        assert_ne!(lines1, sorted);
        lines1.sort();
        assert_eq!(lines1, sorted);
    }

    #[test]
    fn test_hit() {
        let line1 = line(CellGrid::a(), CellGrid::y());
        assert!(line1.hit(CellGrid::g(), CellGrid::s()));

        let rect_gs = rect(CellGrid::g(), CellGrid::s(), false, false);
        assert!(!rect_gs.hit(CellGrid::a(), CellGrid::y()));

        let rect1 = rect(CellGrid::a(), CellGrid::y(), false, false);
        assert!(!rect1.hit(CellGrid::g(), CellGrid::s()));
    }
}
