use crate::buffer::CellGrid;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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
