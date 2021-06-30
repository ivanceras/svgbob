use crate::Point;

///  4x8 - 32 subcells
///
///  with 5x9 = 45 intersections
///
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
pub struct CellGrid {}

impl CellGrid {
    /// get the point intersection at x, y
    #[inline]
    pub fn point(x: usize, y: usize) -> Point {
        let px = x as f32 * Self::unit_x();
        let py = y as f32 * Self::unit_y();
        Point::new(px, py)
    }

    #[inline]
    pub fn a() -> Point {
        Self::point(0, 0)
    }

    #[inline]
    pub fn b() -> Point {
        Self::point(1, 0)
    }

    #[inline]
    pub fn c() -> Point {
        Self::point(2, 0)
    }

    #[inline]
    pub fn d() -> Point {
        Self::point(3, 0)
    }

    #[inline]
    pub fn e() -> Point {
        Self::point(4, 0)
    }

    #[inline]
    pub fn f() -> Point {
        Self::point(0, 2)
    }

    #[inline]
    pub fn g() -> Point {
        Self::point(1, 2)
    }

    #[inline]
    pub fn h() -> Point {
        Self::point(2, 2)
    }

    #[inline]
    pub fn i() -> Point {
        Self::point(3, 2)
    }

    #[inline]
    pub fn j() -> Point {
        Self::point(4, 2)
    }

    #[inline]
    pub fn k() -> Point {
        Self::point(0, 4)
    }

    #[inline]
    pub fn l() -> Point {
        Self::point(1, 4)
    }

    #[inline]
    pub fn m() -> Point {
        Self::point(2, 4)
    }

    #[inline]
    pub fn n() -> Point {
        Self::point(3, 4)
    }

    #[inline]
    pub fn o() -> Point {
        Self::point(4, 4)
    }

    #[inline]
    pub fn p() -> Point {
        Self::point(0, 6)
    }

    #[inline]
    pub fn q() -> Point {
        Self::point(1, 6)
    }

    #[inline]
    pub fn r() -> Point {
        Self::point(2, 6)
    }

    #[inline]
    pub fn s() -> Point {
        Self::point(3, 6)
    }

    #[inline]
    pub fn t() -> Point {
        Self::point(4, 6)
    }

    #[inline]
    pub fn u() -> Point {
        Self::point(0, 8)
    }

    #[inline]
    pub fn v() -> Point {
        Self::point(1, 8)
    }

    #[inline]
    pub fn w() -> Point {
        Self::point(2, 8)
    }

    #[inline]
    pub fn x() -> Point {
        Self::point(3, 8)
    }

    #[inline]
    pub fn y() -> Point {
        Self::point(4, 8)
    }

    #[inline]
    pub fn width() -> f32 {
        1.0
    }

    #[inline]
    pub fn height() -> f32 {
        2.0
    }

    /// 0.25
    #[inline]
    pub fn unit_x() -> f32 {
        Self::width() / Self::horizontal_slices() as f32
    }

    /// 0.25
    #[inline]
    pub fn unit_y() -> f32 {
        Self::height() / Self::vertical_slices() as f32
    }

    #[inline]
    fn vertical_slices() -> usize {
        8
    }

    #[inline]
    fn horizontal_slices() -> usize {
        4
    }

    pub fn diagonal_length() -> f32 {
        let w = Self::width();
        let h = Self::height();
        (w * w + h * h).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        assert_eq!(Point::new(0.0, 0.0), CellGrid::a());
        assert_eq!(Point::new(1.0, 2.0), CellGrid::y());
        assert_eq!(Point::new(0.0, 1.0), CellGrid::k());
        assert_eq!(Point::new(1.0, 1.0), CellGrid::o());
    }

    #[test]
    fn test_diagonal_length() {
        assert_eq!(CellGrid::diagonal_length(), 2.236068);
    }
}
