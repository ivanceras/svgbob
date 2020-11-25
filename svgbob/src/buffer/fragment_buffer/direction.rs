use crate::buffer::CellGrid;
use crate::fragment::PolygonTag;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Direction {
    /// return the opposite direction of self
    pub(crate) fn opposite(&self) -> Self {
        match self {
            Direction::TopLeft => Direction::BottomRight,
            Direction::Top => Direction::Bottom,
            Direction::TopRight => Direction::BottomLeft,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::BottomLeft => Direction::TopRight,
            Direction::Bottom => Direction::Top,
            Direction::BottomRight => Direction::TopLeft,
        }
    }
    /*
    pub(crate) fn any_along_side(&self, tags: &[PolygonTag]) -> bool {
        tags.iter().any(|tag| self.is_along_side(tag))
    }
    /// diamon matches alongside for everything.
    pub(crate) fn is_along_side(&self, tag: &PolygonTag) -> bool {
        if *tag == PolygonTag::DiamondBullet {
            return true;
        }
        match self {
            Direction::TopLeft | Direction::BottomRight => match tag {
                PolygonTag::ArrowTopLeft
                | PolygonTag::ArrowBottomRight
                | PolygonTag::ArrowTop
                | PolygonTag::ArrowBottom => true,
                _ => false,
            },
            Direction::Top | Direction::Bottom => match tag {
                PolygonTag::ArrowTop | PolygonTag::ArrowBottom => true,
                _ => false,
            },
            Direction::TopRight | Direction::BottomLeft => match tag {
                PolygonTag::ArrowTopRight
                | PolygonTag::ArrowBottomLeft
                | PolygonTag::ArrowTop
                | PolygonTag::ArrowBottom => true,
                _ => false,
            },
            Direction::Left | Direction::Right => match tag {
                PolygonTag::ArrowLeft | PolygonTag::ArrowRight => true,
                _ => false,
            },
        }
    }
    */

    /// calculate the threshold length which is the basis
    /// if the arrow and the line is connected
    pub(crate) fn threshold_length(&self) -> f32 {
        match self {
            Direction::TopLeft
            | Direction::TopRight
            | Direction::BottomLeft
            | Direction::BottomRight => CellGrid::diagonal_length(),
            Direction::Left | Direction::Right => CellGrid::width(),
            Direction::Top | Direction::Bottom => CellGrid::height(),
        }
    }
}
