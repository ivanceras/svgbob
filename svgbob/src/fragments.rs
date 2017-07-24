use self::Block::{ A,B,C,D,E, F,G,H,I,J, K,L,M,N,O, P,Q,R,S,T, U,V,W,X,Y };

use self::Fragment::{ Line, ArrowLine, StartArrowLine, Arc };
use self::Direction::{ Top, Bottom, Left, Right, TopLeft, TopRight, BottomLeft, BottomRight, };

use Element;
use line;

/// exact location of point
/// relative to the Character Block
/// The block is divided in to 5x5 small blocks
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Block{
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y
}


/// These are non-final drawing elements
/// Lines most likely fall on the collinear line
/// arc most likely be changed
#[derive(Debug)]
pub enum Fragment{
    Line(Block, Block),
    ArrowLine(Block, Block),
    StartArrowLine(Block, Block), // the arrow is at the start marker
    Arc(Block, Block, i32),//i32 is the multiplier to 1/4 of textwidth
}







/// 8 directions which a character can connect to
///   \|/
///   -+-
///   /|\
/// Block is when the connection is in a certain specific point
/// in the character block that is not part of the 
/// 8 regular connection point
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Direction{
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Block(Block),
}

impl Direction{

    ///
    /// convert direction relative to the center to block location
    ///
    pub fn to_block(&self) -> Block {
        match *self{
            TopLeft => A,
            Top => C,
            TopRight => E,
            Left => K,
            Right => O,
            BottomLeft => U,
            Bottom => W,
            BottomRight => Y,
            Direction::Block(ref block) => block.clone()
        }
    }
}


/// return a fragment line
/// connecting the middle to the block
/// at the specified direction
pub fn line_to(dir: Direction) -> Fragment {
    Line(M, dir.to_block())
}


pub fn line_from(dir: Direction) -> Fragment {
    Line(dir.to_block(), M)
}


pub fn middle_line() -> Fragment {
    Line(K, O)
}

pub fn center_line() -> Fragment {
    Line(C,W)
}

