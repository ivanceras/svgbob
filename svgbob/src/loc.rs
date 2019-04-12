use location::Direction::{Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight};
use location::Location;
use std::cmp::Ordering;

/// Location of Block relative to the Grid
/// This the equivalent to the cell cation in the grid
/// 0,0 is the top left most
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Loc {
    pub x: i32,
    pub y: i32,
}

impl Ord for Loc {
    fn cmp(&self, other: &Loc) -> Ordering {
        if let Some(order) = self.partial_cmp(other) {
            return order;
        }
        Ordering::Less
    }
}

impl Loc {
    pub fn new(x: i32, y: i32) -> Loc {
        Loc { x: x, y: y }
    }

    pub fn from_location(&self, location: &Location) -> Loc {
        let mut loc = self.clone();
        for &(ref direction, step) in &location.0 {
            for _ in 0..step {
                match *direction {
                    TopLeft => {
                        loc = loc.top().left();
                    }
                    Top => {
                        loc = loc.top();
                    }
                    TopRight => {
                        loc = loc.top().right();
                    }
                    Left => {
                        loc = loc.left();
                    }
                    Right => {
                        loc = loc.right();
                    }
                    BottomLeft => {
                        loc = loc.bottom().left();
                    }
                    Bottom => {
                        loc = loc.bottom();
                    }
                    BottomRight => {
                        loc = loc.bottom().right();
                    }
                };
            }
        }
        loc
    }

    pub fn top(&self) -> Loc {
        Loc {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn in_top(&self, n: i32) -> Loc {
        Loc {
            x: self.x,
            y: self.y - n,
        }
    }

    pub fn left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn in_left(&self, step: i32) -> Loc {
        Loc {
            x: self.x - step,
            y: self.y,
        }
    }
    pub fn bottom(&self) -> Loc {
        Loc {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn in_bottom(&self, n: i32) -> Loc {
        Loc {
            x: self.x,
            y: self.y + n,
        }
    }
    pub fn right(&self) -> Loc {
        Loc {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn in_right(&self, step: i32) -> Loc {
        Loc {
            x: self.x + step,
            y: self.y,
        }
    }

    pub fn top_left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    pub fn top_right(&self) -> Loc {
        Loc {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    pub fn bottom_left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    pub fn bottom_right(&self) -> Loc {
        Loc {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    /// make a lower and upper bound loc with
    /// ry units top + ry units bottom
    /// rx units left + rx units right
    pub fn get_range(&self, rx: i32, ry: i32) -> (Loc, Loc) {
        let loc1 = Loc::new(self.x - rx, self.y - ry);
        let loc2 = Loc::new(self.x + rx, self.y + ry);
        (loc1, loc2)
    }
}
