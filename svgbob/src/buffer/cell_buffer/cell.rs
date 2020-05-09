use crate::{util, Point};
use ncollide2d::{
    bounding_volume::AABB,
    math::Isometry,
    query::{proximity, Proximity},
    shape::{Polyline, Segment},
};
use std::{cmp, cmp::Ordering, fmt};

mod cell_grid;

pub use cell_grid::CellGrid;

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

/// A single element in the terminal that
/// can fit 1 character.
/// Describe the exact location of a point/subcell in a grid.
#[derive(Debug, PartialEq, Hash, PartialOrd, Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Eq for Cell {}
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

macro_rules! cell_grid {
    ($($a:ident),*) => {
        /// The point at sepcific cell grid of this cell
        $(pub fn $a(&self) -> Point {
            self.top_left_most() + CellGrid::$a()
        })*
    }
}

impl Cell {
    cell_grid!(
        a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y
    );

    pub fn new(x: i32, y: i32) -> Self {
        Cell { x, y }
    }

    /// returns true if the other cell is at: top_left, top, top_right, left, right, bottom_left,
    /// bottom, bottom_right of self
    pub fn is_adjacent(&self, other: &Self) -> bool {
        (other.x - self.x).abs() <= 1 && (other.y - self.y).abs() <= 1
    }

    /// Derive which cell this points falls into and snap the point closes to any
    /// intersection in the cell grid.
    /// FIXME: need to find a way to snap a group of point that lies in boundaries to
    /// snap together to a common cell.
    pub fn snap_point(point: Point) -> (Self, Point) {
        let x = (point.x / Self::width()).floor();
        let y = (point.y / Self::height()).floor();
        let cell = Self::new(x as i32, y as i32);
        let snap = Self::snap(cell.localize_point(point));
        (cell, snap)
    }

    pub fn snap_group(points: &[Point]) -> Self {
        let snaps: Vec<(Self, Point)> = points
            .iter()
            .map(|point| Self::snap_point(*point))
            .collect();
        let (cells, _snap_points): (Vec<Self>, Vec<Point>) = snaps.into_iter().unzip();
        let min_cell: Self = cells.into_iter().min().expect("should have a min cell");
        min_cell
    }

    pub(in crate) fn absolute_position(&self, point: Point) -> Point {
        self.top_left_most() + point
    }

    /// The point at the top right of this cell
    pub fn top_left_most(&self) -> Point {
        let px = self.x as f32 * CellGrid::width();
        let py = self.y as f32 * CellGrid::height();
        Point::new(px, py)
    }

    pub fn bottom_right_most(&self) -> Point {
        let px = (self.x + 1) as f32 * CellGrid::width();
        let py = (self.y + 1) as f32 * CellGrid::height();
        Point::new(px, py)
    }

    /// turn point into relative distance from the top-left of this cell
    /// by simply deducting the point p with this cell's top_left_most point
    pub fn localize_point(&self, point: Point) -> Point {
        point - self.top_left_most()
    }

    pub fn localize_cell(&self, cell: Cell) -> Cell {
        Cell::new(cell.x - self.x, cell.y - self.y)
    }

    /// the bounding box of this cell
    #[inline]
    fn bounding_box(&self) -> AABB<f32> {
        let start = Point::new(
            self.x as f32 * Self::width(),
            self.y as f32 * Self::height(),
        );
        let end = Point::new(
            (self.x + 1) as f32 * Self::width(),
            (self.y + 1) as f32 * Self::height(),
        );
        AABB::new(*start, *end)
    }

    /// Convert the bounding box aabb to polyline segment
    /// the dots from top-left, top-right, bottom-right, bottom-left then closing to top-left
    /// The polyline is then used to testing for intersection with the line segment
    fn polyline(&self) -> Polyline<f32> {
        let aabb = self.bounding_box();
        let min = aabb.mins();
        let max = aabb.maxs();
        let x1 = min.x;
        let y1 = min.y;
        let x2 = max.x;
        let y2 = max.y;
        let c1 = Point::new(x1, y1); // top-left
        let c2 = Point::new(x2, y1); // top-right
        let c3 = Point::new(x2, y2); // bottom-right
        let c4 = Point::new(x1, y2); // bottom-left
        Polyline::new(vec![*c1, *c2, *c3, *c4, *c1], None)
    }

    pub fn width() -> f32 {
        CellGrid::width()
    }

    pub fn height() -> f32 {
        CellGrid::height()
    }

    pub fn unit(l: i32) -> f32 {
        CellGrid::unit_x() * l as f32
    }

    /// test whether this cell is intersected with the line segment
    /// with point `start` and `end`
    pub fn is_intersected(&self, start: Point, end: Point) -> bool {
        let pl = self.polyline();
        let segment = Segment::new(*start, *end);
        let prox = proximity(
            &Isometry::identity(),
            &pl,
            &Isometry::identity(),
            &segment,
            0.0,
        );
        prox == Proximity::Intersecting
    }

    /// check if this cell is bounded by the lower bound and upper bound
    pub fn is_bounded(&self, bound1: Cell, bound2: Cell) -> bool {
        let (lower_bound, upper_bound) = rearrange_bound(bound1, bound2);
        self.x >= lower_bound.x
            && self.y >= lower_bound.y
            && self.x <= upper_bound.x
            && self.y <= upper_bound.y
    }

    /// snap a point closest to any of the intersection of this cellgrid
    #[inline]
    fn snap_xy(x: f32, y: f32) -> Point {
        let tx = (x * 4.0).round() / 4.0;
        let ty = (y * 8.0).round() / 8.0;
        Point::new(tx, ty)
    }

    /// snap point to a closest intersection of this cellgrid
    #[inline]
    pub fn snap(p: Point) -> Point {
        Self::snap_xy(p.x, p.y)
    }

    /// clip a line segment within the bounding box of this cell
    fn clip_line(&self, start: Point, end: Point) -> Option<(Point, Point)> {
        let aabb = self.bounding_box();
        util::clip_line(&aabb, start, end)
    }

    pub fn clip_line_snap(&self, start: Point, end: Point) -> Option<(Point, Point)> {
        self.clip_line(start, end)
            .map(|(s, e)| (Self::snap(s), Self::snap(e)))
    }

    /// clip line then localize the points and snap to the nearest cell grid intersection
    pub fn clip_line_localize(&self, start: Point, end: Point) -> Option<(Point, Point)> {
        self.clip_line_snap(start, end)
            .map(|(s, e)| (self.localize_point(s), self.localize_point(e)))
    }

    /// The cell at the top left of this cell
    #[inline]
    pub fn top_left(&self) -> Self {
        Cell {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    #[inline]
    pub fn top(&self) -> Self {
        Cell {
            x: self.x,
            y: self.y - 1,
        }
    }

    #[inline]
    pub fn top_right(&self) -> Self {
        Cell {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    /// The cell at the left of this cell
    #[inline]
    pub fn left(&self) -> Self {
        Cell {
            x: self.x - 1,
            y: self.y,
        }
    }

    #[inline]
    pub fn right(&self) -> Self {
        Cell {
            x: self.x + 1,
            y: self.y,
        }
    }

    #[inline]
    pub fn bottom_left(&self) -> Self {
        Cell {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    #[inline]
    pub fn bottom(&self) -> Self {
        Cell {
            x: self.x,
            y: self.y + 1,
        }
    }

    #[inline]
    pub fn bottom_right(&self) -> Self {
        Cell {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

/// rearrange the bound of 2 cells
pub fn rearrange_bound(bound1: Cell, bound2: Cell) -> (Cell, Cell) {
    let min_x = cmp::min(bound1.x, bound2.x);
    let min_y = cmp::min(bound1.y, bound2.y);
    let max_x = cmp::max(bound1.x, bound2.x);
    let max_y = cmp::max(bound1.y, bound2.y);
    (Cell::new(min_x, min_y), Cell::new(max_x, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_right() {
        let cell = Cell::new(2, 0);
        assert_eq!(cell.bottom_right(), Cell::new(3, 1));
    }

    #[test]
    fn test_adjacent() {
        let cell = Cell::new(4, 4);
        let top = Cell::new(4, 3);
        let top2 = Cell::new(4, 2);
        assert!(cell.is_adjacent(&top));
        assert!(top.is_adjacent(&cell));
        assert!(cell.is_adjacent(&cell.top_left()));
        assert!(cell.is_adjacent(&cell.top()));
        assert!(cell.is_adjacent(&cell.top_right()));
        assert!(cell.is_adjacent(&cell.left()));
        assert!(cell.is_adjacent(&cell.right()));
        assert!(cell.is_adjacent(&cell.bottom_left()));
        assert!(cell.is_adjacent(&cell.bottom()));
        assert!(cell.is_adjacent(&cell.bottom_right()));

        assert!(!cell.is_adjacent(&top2));
        assert!(!cell.is_adjacent(&cell.top_left().top()));
        assert!(cell.is_adjacent(&cell.top().left()));
        assert!(!cell.is_adjacent(&cell.top_right().right()));
        assert!(!cell.is_adjacent(&cell.left().left()));
        assert!(!cell.is_adjacent(&cell.right().right()));
        assert!(!cell.is_adjacent(&cell.bottom_left().bottom_left()));
        assert!(!cell.is_adjacent(&cell.bottom().bottom_right()));
        assert!(!cell.is_adjacent(&cell.bottom_right().right()));
    }

    #[test]
    fn test_location() {
        let cell = Cell::new(5, 5);
        assert_eq!(cell.left(), Cell::new(4, 5));
        assert_eq!(cell.right(), Cell::new(6, 5));
        assert_eq!(cell.top(), Cell::new(5, 4));
        assert_eq!(cell.bottom(), Cell::new(5, 6));
        assert_eq!(cell.top_left(), Cell::new(4, 4));
        assert_eq!(cell.top_right(), Cell::new(6, 4));
        assert_eq!(cell.bottom_left(), Cell::new(4, 6));
        assert_eq!(cell.bottom_right(), Cell::new(6, 6));
    }

    #[test]
    fn cell_from_snap_group_point() {
        let p1 = Point::new(11.0, 11.0);
        let p2 = Point::new(10.0, 11.0);
        let (cell1, snap1) = Cell::snap_point(p1);
        assert_eq!(Cell::new(11, 5), cell1);
        assert_eq!(snap1, Point::new(0.0, 1.0));

        let (cell2, _snap2) = Cell::snap_point(p2);
        assert_eq!(Cell::new(10, 5), cell2);

        let cell_group = Cell::snap_group(&[p1, p2]);
        assert_eq!(Cell::new(10, 5), cell_group);
        let local1 = cell_group.localize_point(p1);
        let local2 = cell_group.localize_point(p2);
        println!("local1: {:#?}", local1);
        println!("local2: {:#?}", local2);
        assert_eq!(local1, Point::new(1.0, 1.0));
        assert_eq!(local2, Point::new(0.0, 1.0));
    }

    #[test]
    fn cell_from_point() {
        let (cell, snap) = Cell::snap_point(Point::new(10.0, 11.0));
        assert_eq!(Cell::new(10, 5), cell);
        assert_eq!(snap, Point::new(0.0, 1.0));
    }

    #[test]
    fn test_localize() {
        let cell = Cell::new(3, 2);
        let m = cell.m();
        assert_eq!(m, Point::new(3.5, 5.0));
        let local_m = cell.localize_point(m);
        assert_eq!(local_m, Point::new(0.5, 1.0));
    }

    #[test]
    fn test_locations() {
        assert_eq!(Cell::new(0, 0).top_left_most(), Point::new(0.0, 0.0));
        assert_eq!(Cell::new(0, 0).a(), Point::new(0.0, 0.0));
        assert_eq!(Cell::new(0, 0).y(), Point::new(1.0, 2.0));
        assert_eq!(Cell::new(1, 0).y(), Point::new(2.0, 2.0));
        assert_eq!(Cell::new(1, 1).m(), Point::new(1.5, 3.0));
        assert_eq!(Cell::new(9, 9).o(), Point::new(10.0, 19.0));
    }

    #[test]
    fn text_proximity() {
        assert_eq!(Cell::snap_xy(0.1, 0.05), CellGrid::a());
        assert_eq!(Cell::snap_xy(1.1, 2.05), CellGrid::y());
    }

    #[test]
    fn test_aabb() {
        assert_eq!(
            AABB::new(*Point::new(0.0, 0.0), *Point::new(1.0, 2.0)),
            Cell::new(0, 0).bounding_box()
        );
    }

    #[test]
    fn test_clip_line() {
        assert_eq!(
            Cell::new(0, 0).clip_line_snap(Point::new(-0.01, -0.01), Point::new(1.01, 2.01)),
            Some((CellGrid::a(), CellGrid::y()))
        );

        assert_eq!(
            Cell::new(0, 0).clip_line_snap(Point::new(0.0, 0.0), Point::new(1.00, 0.0)),
            Some((CellGrid::a(), CellGrid::e()))
        );

        let clipped = Cell::new(0, 0).clip_line_snap(Point::new(0.0, 1.0), Point::new(1.0, 1.0));
        assert_eq!(clipped, Some((CellGrid::k(), CellGrid::o())));

        let clipped =
            Cell::new(0, 0).clip_line_snap(Point::new(-0.01, 1.01), Point::new(1.01, 0.95));
        assert_eq!(clipped, Some((CellGrid::k(), CellGrid::o())));

        assert_eq!(
            Cell::new(0, 0).clip_line_snap(Point::new(0.0, 2.0), Point::new(1.0, 2.0)),
            Some((CellGrid::u(), CellGrid::y()))
        );

        assert_eq!(
            Cell::new(0, 0).clip_line_snap(Point::new(0.0, 0.0), Point::new(0.0, 2.0)),
            Some((CellGrid::a(), CellGrid::u()))
        );

        assert_eq!(
            Cell::new(0, 0).clip_line_snap(Point::new(1.0, 0.0), Point::new(1.0, 2.0)),
            Some((CellGrid::e(), CellGrid::y()))
        );

        assert_eq!(
            Cell::new(0, 0).clip_line_snap(Point::new(1.0, 0.0), Point::new(0.0, 0.0)),
            Some((CellGrid::e(), CellGrid::a()))
        );

        assert_eq!(
            Cell::new(0, 0).clip_line_snap(Point::new(0.5, 1.0), Point::new(1.0, 1.0)),
            Some((CellGrid::m(), CellGrid::o()))
        );

        assert_eq!(
            Cell::new(0, 0).clip_line_snap(Point::new(0.25, 1.0), Point::new(1.0, 1.0)),
            Some((CellGrid::l(), CellGrid::o()))
        );
    }
}
