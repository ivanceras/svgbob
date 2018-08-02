use self::Direction::{Top,Bottom,Left,Right};
use point_block::PointBlock;
use block::{Block::{self,A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y}};
/// a location in the grid
/// relative to the focused char
/// go to direction and how many steps to get there
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct Location(pub Vec<(Direction, usize)>);

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

impl Location {
    pub fn go(direction: Direction) -> Self {
        Self::jump(direction, 1)
    }

    // this location
    // TODO: hacky fix these
    pub fn this()-> Self{
        Self::jump(Right, 0)
    }

    pub fn jump(direction: Direction, step: usize) -> Self {
        Location(vec![(direction, step)])
    }


    fn jump_to(&mut self, direction: Direction, step: usize) {
        self.0.push((direction, step));
    }

    fn go_jump(&self, direction: Direction, step: usize) -> Self {
        let mut loc = self.clone();
        loc.jump_to(direction, step);
        loc
    }

    pub fn go_top(&self, step: usize) -> Self {
        self.go_jump(Top, step)
    }
    pub fn go_left(&self, step: usize) -> Self {
        self.go_jump(Left, step)
    }
    pub fn go_bottom(&self, step: usize) -> Self {
        self.go_jump(Bottom, step)
    }
    pub fn go_right(&self, step: usize) -> Self {
        self.go_jump(Right, step)
    }

    pub fn top(&self) -> Self {
        self.go_top(1)
    }
    pub fn bottom(&self) -> Self {
        self.go_bottom(1)
    }
    pub fn left(&self) -> Self {
        self.go_left(1)
    }
    pub fn right(&self) -> Self {
        self.go_right(1)
    }

    pub fn a(&self) -> PointBlock {
        self.block(A)
    }

    pub fn b(&self) -> PointBlock {
        self.block(B)
    }
    pub fn c(&self) -> PointBlock {
        self.block(C)
    }
    pub fn d(&self) -> PointBlock {
        self.block(D)
    }
    pub fn e(&self) -> PointBlock {
        self.block(E)
    }
    pub fn f(&self) -> PointBlock {
        self.block(F)
    }
    pub fn g(&self) -> PointBlock {
        self.block(G)
    }
    pub fn h(&self) -> PointBlock {
        self.block(H)
    }
    pub fn i(&self) -> PointBlock {
        self.block(I)
    }
    pub fn j(&self) -> PointBlock {
        self.block(J)
    }
    pub fn k(&self) -> PointBlock {
        self.block(K)
    }
    pub fn l(&self) -> PointBlock {
        self.block(L)
    }
    pub fn m(&self) -> PointBlock {
        self.block(M)
    }
    pub fn n(&self) -> PointBlock {
        self.block(N)
    }
    pub fn o(&self) -> PointBlock {
        self.block(O)
    }
    pub fn p(&self) -> PointBlock {
        self.block(P)
    }
    pub fn q(&self) -> PointBlock {
        self.block(Q)
    }
    pub fn r(&self) -> PointBlock {
        self.block(R)
    }
    pub fn s(&self) -> PointBlock {
        self.block(S)
    }
    pub fn t(&self) -> PointBlock {
        self.block(T)
    }
    pub fn u(&self) -> PointBlock {
        self.block(U)
    }
    pub fn v(&self) -> PointBlock {
        self.block(V)
    }
    pub fn w(&self) -> PointBlock {
        self.block(W)
    }
    pub fn x(&self) -> PointBlock {
        self.block(X)
    }
    pub fn y(&self) -> PointBlock {
        self.block(Y)
    }

    pub fn block(&self, block: Block) -> PointBlock {
        PointBlock {
            location: Some(self.clone()),
            block: block,
            adjust_x: 0.0,
            adjust_y: 0.0
        }
    }
}

