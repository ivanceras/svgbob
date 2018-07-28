use self::Fragment::{Arc, ArrowLine, Line, OpenCircle, SolidCircle, StartArrowLine};

use properties::PointBlock;
use std::cmp::Ordering;

/// exact location of point
/// relative to the Character Block
/// The block is divided in to 5x5 small blocks
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
pub enum Block {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
}

/// These are non-final drawing elements
/// Lines most likely fall on the collinear line
/// arc most likely be changed

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum Fragment {
    Line(PointBlock, PointBlock),
    ArrowLine(PointBlock, PointBlock),
    StartArrowLine(PointBlock, PointBlock), // the arrow is at the start marker
    Arc(PointBlock, PointBlock, i32),       //i32 is the multiplier to 1/4 of textwidth
    OpenCircle(PointBlock, i32),
    SolidCircle(PointBlock, i32),
    Text(String),
}

pub fn line(p1: &PointBlock, p2: &PointBlock) -> Fragment {
    Line(p1.clone(), p2.clone())
}
pub fn arrow_line(p1: &PointBlock, p2: &PointBlock) -> Fragment {
    ArrowLine(p1.clone(), p2.clone())
}
pub fn start_arrow_line(p1: &PointBlock, p2: &PointBlock) -> Fragment {
    StartArrowLine(p1.clone(), p2.clone())
}
pub fn arc(s: &PointBlock, e: &PointBlock, r: i32) -> Fragment {
    Arc(s.clone(), e.clone(), r)
}
pub fn open_circle(c: &PointBlock, r: i32) -> Fragment {
    OpenCircle(c.clone(), r)
}
pub fn solid_circle(c: &PointBlock, r: i32) -> Fragment {
    SolidCircle(c.clone(), r)
}

/// 8 directions which a character can connect to
///   \|/
///   -+-
///   /|\

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
