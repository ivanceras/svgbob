use block::Block;
use location::{Direction, Location};
use std::cmp::Ordering;

/// An exact point in the grid
/// relative to the focused char
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PointBlock {
    pub location: Option<Location>,
    pub block: Block,
    pub adjust_x: f32,
    pub adjust_y: f32,
}

impl Ord for PointBlock {
    fn cmp(&self, other: &PointBlock) -> Ordering {
        self.location.cmp(&other.location)
    }
}

impl Eq for PointBlock {}

impl PointBlock {
    pub fn block(block: Block) -> Self {
        PointBlock {
            location: None,
            block: block,
            adjust_x: 0.0,
            adjust_y: 0.0,
        }
    }

    pub fn go(direction: Direction, step: usize, block: Block) -> Self {
        PointBlock {
            location: Some(Location::jump(direction, step)),
            block: block,
            adjust_x: 0.0,
            adjust_y: 0.0,
        }
    }

    pub fn adjust(&self, x: f32, y: f32) -> Self {
        let mut pb = self.clone();
        pb.adjust_x = pb.adjust_x + x;
        pb.adjust_y = pb.adjust_y + y;
        pb
    }
}
