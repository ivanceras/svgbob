use self::Fragment::{ Line, ArrowLine, StartArrowLine, Arc, OpenCircle, SolidCircle, Text };

use Element;
use properties::PointBlock;

/// exact location of point
/// relative to the Character Block
/// The block is divided in to 5x5 small blocks
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Block{
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y
}

/*
impl Block{

    fn connects_to(&self) -> Vec<(Direction, Block)> {
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
*/


/// These are non-final drawing elements
/// Lines most likely fall on the collinear line
/// arc most likely be changed
#[derive(Debug)]
#[derive(Clone)]
pub enum Fragment{
    Line(PointBlock, PointBlock),
    ArrowLine(PointBlock, PointBlock),
    StartArrowLine(PointBlock, PointBlock), // the arrow is at the start marker
    Arc(PointBlock, PointBlock, i32),//i32 is the multiplier to 1/4 of textwidth
    OpenCircle(PointBlock, i32),
    SolidCircle(PointBlock, i32),
    Text(String),
}

pub fn line(p1: &PointBlock, p2: &PointBlock) -> Fragment{
    Line(p1.clone(), p2.clone())
}
pub fn arrow_line(p1: &PointBlock, p2: &PointBlock) -> Fragment{
    ArrowLine(p1.clone(), p2.clone())
}
pub fn start_arrow_line(p1: &PointBlock, p2: &PointBlock) -> Fragment {
    StartArrowLine(p1.clone(), p2.clone())
}
pub fn arc(s: &PointBlock, e: &PointBlock, r: i32) -> Fragment{
    Arc(s.clone(), e.clone(), r)
}
pub fn open_circle(c: &PointBlock, r: i32) -> Fragment {
    OpenCircle(c.clone(), r)
}
pub fn solid_circle(c: &PointBlock, r: i32) -> Fragment {
    SolidCircle(c.clone(), r)
}
pub fn text(s:String) -> Fragment {
    Text(s)
}








/// 8 directions which a character can connect to
///   \|/
///   -+-
///   /|\
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


