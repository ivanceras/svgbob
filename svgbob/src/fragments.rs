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

impl Block{

    pub fn connects_to(&self) -> Vec<(Direction, Block)> {
        match *self{
            A => vec![
                (Top,U),
                (Left,E),
                (TopLeft,Y)
                ],
            B => vec![(Top,V)],
            C => vec![(Top,W)],
            D => vec![(Top,X)],
            E => vec![
                (Top,Y),
                (Right,A),
                (TopRight,U)
            ],
            F => vec![(Left,J)],
            G => vec![],
            H => vec![],
            I => vec![],
            J => vec![(Right,F)],
            K => vec![(Left,O)],
            L => vec![],
            M => vec![],
            N => vec![],
            O => vec![(Right,K)],
            P => vec![(Left,T)],
            Q => vec![],
            R => vec![],
            S => vec![],
            T => vec![(Right,P)],
            U => vec![
                (Left,Y),
                (Bottom,A),
                (BottomLeft,E)
                ],
            V => vec![(Bottom,B)],
            W => vec![(Bottom,C)],
            X => vec![(Bottom,D)],
            Y => vec![
                (Right,U),
                (Bottom,E),
                (BottomRight,A)
                ],
        }
    }

}


/// These are non-final drawing elements
/// Lines most likely fall on the collinear line
/// arc most likely be changed
#[derive(Debug)]
#[derive(Clone)]
pub enum Fragment{
    Line(Block, Block),
    ArrowLine(Block, Block),
    StartArrowLine(Block, Block), // the arrow is at the start marker
    Arc(Block, Block, i32),//i32 is the multiplier to 1/4 of textwidth
    OpenCircle(Block, i32),
    SolidCircle(Block, i32),
    Text(String),
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
}

/// a location in the grid
/// relative to the focused char
/// go to direction and how many steps to get there
pub struct Location(Vec<(Direction,usize)>);

impl Location{
    fn go(direction: Direction) -> Self{
        Self::jump(direction, 1)
    }

    fn jump(direction: Direction, step: usize) -> Self {
        Location(vec![(direction, step)])
    }

    fn go_to(&mut self, direction: Direction) {
        self.jump_to(direction, 1);
    }

    fn jump_to(&mut self, direction: Direction, step: usize){
        self.0.push((direction, step));
    }
}

/// An exact point in the grid
/// relative to the focused char
struct PointBlock{
    location: Option<Location>,
    block: Block,
    adjust: f32,
}

impl PointBlock{
    fn block(block: Block) -> Self {
        PointBlock{
            location: None,
            block: block,
            adjust: 0.0,
        }
    }

    fn go(direction: Direction, step: usize, block: Block) -> Self {
        PointBlock{
            location: Some(Location::jump(direction, step)),
            block: block,
            adjust: 0.0,
        }
    }

    fn adjust(&mut self, adjust: f32){
        self.adjust += adjust;
    }
}

