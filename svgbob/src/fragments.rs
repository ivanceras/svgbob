use self::Fragment::{Arc, ArrowLine, Line, DashedLine, CircleStartLine, CircleOpenLine, BigCircleOpenLine, OpenCircle, SolidCircle, StartArrowLine};

use point_block::PointBlock;


/// These are non-final drawing elements
/// Lines most likely fall on the collinear line
/// arc most likely be changed

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum Fragment {
    Line(PointBlock, PointBlock),
    CircleStartLine(PointBlock, PointBlock),
    CircleOpenLine(PointBlock, PointBlock),
    BigCircleOpenLine(PointBlock, PointBlock),
    DashedLine(PointBlock, PointBlock),
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
pub fn circle_start_line(p1: &PointBlock, p2: &PointBlock) -> Fragment {
    CircleStartLine(p1.clone(), p2.clone())
}
pub fn circle_open_line(p1: &PointBlock, p2: &PointBlock) -> Fragment {
    CircleOpenLine(p1.clone(), p2.clone())
}
pub fn big_circle_open_line(p1: &PointBlock, p2: &PointBlock) -> Fragment {
    BigCircleOpenLine(p1.clone(), p2.clone())
}
pub fn dashed_line(p1: &PointBlock, p2: &PointBlock) -> Fragment {
    DashedLine(p1.clone(), p2.clone())
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

