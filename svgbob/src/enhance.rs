use fragments::Block;
use fragments::Block::{
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y
};
use properties::Location;
use properties::PointBlock;
use patterns::FocusChar;
use fragments::Direction;

use fragments::Fragment;
use fragments::{
    line,
    arrow_line,
    start_arrow_line,
    arc,
    open_circle,
    solid_circle,
};

use fragments::Direction::{
    Top,Bottom,
    Left,Right,
    TopLeft,TopRight,
    BottomLeft,BottomRight
};

pub trait Enhance{
    
    fn enhance(&self) -> (Vec<Fragment>, Vec<Location>);
}


impl <'g>Enhance for FocusChar<'g>{
    ///
    /// Enhancement use case where it is hard to incorporate into the default characteristic
    /// examples: 
    ///      +    \           _   _ 
    ///       `>   `>        |_   _|
    ///                     
    ///    .-.               
    ///   (   )
    ///    `-'
    ///
    /// without checking the characteristic, this will enchance the character
    /// based on the surrounding neighbors
    /// returns the fragments, consumed location
    fn enhance(&self) -> (Vec<Fragment>, Vec<Location>) {
        use fragments::Direction::*;
        use fragments;
        let a = &PointBlock::block(A);
        let b = &PointBlock::block(B);
        let c = &PointBlock::block(C);
        let d = &PointBlock::block(D);
        let e = &PointBlock::block(E);
        let f = &PointBlock::block(F);
        let g = &PointBlock::block(G);
        let h = &PointBlock::block(H);
        let i = &PointBlock::block(I);
        let j = &PointBlock::block(J);
        let k = &PointBlock::block(K);
        let l = &PointBlock::block(L);
        let m = &PointBlock::block(M);
        let n = &PointBlock::block(N);
        let o = &PointBlock::block(O);
        let p = &PointBlock::block(P);
        let q = &PointBlock::block(Q);
        let r = &PointBlock::block(R);
        let s = &PointBlock::block(S);
        let t = &PointBlock::block(T);
        let u = &PointBlock::block(U);
        let v = &PointBlock::block(V);
        let w = &PointBlock::block(W);
        let x = &PointBlock::block(X);
        let y = &PointBlock::block(Y);
        
        let top = || Location::go(Top);
        let bottom = || Location::go(Bottom);
        let left = || Location::go(Left);
        let right = || Location::go(Right);
        let top_left = || Location::go(TopLeft);
        let top_right = || Location::go(TopRight);
        let bottom_left = || Location::go(BottomLeft);
        let bottom_right = || Location::go(BottomRight);

        let enable_aggressive_underscore = true;

        let mut elm = vec![];
        let mut consumed = vec![];
        if self.is('|'){
            //    |    |
            //    <    >
            if self.bottom().any("></\\"){
                elm.push(line(c, &bottom().m()));
            }
            //    <    >
            //    |    |
            if self.top().any("></\\"){
                elm.push(line(w, &top().m()));
            }
        }
        else if self.is('/'){
            //      >
            //     /
            if self.top_right().is('>'){
                elm.push(line(u, &top_right().m()));
            }
            //    /
            //   <
            if self.bottom_left().is('<'){
                elm.push(line(e, &bottom_left().m()));
            }
        }
        else if self.is('\\'){
            //      \
            //       >
            if self.bottom_right().is('>'){
                elm.push(line(a, &bottom_right().m()));
            }
            //    <
            //     \
            if self.top_left().is('<'){
                elm.push(line(y, &top_left().m()));
            }
        }
        else if self.any("`'"){
            // for circuitries
            //  +     +    \
            //   `>    '>   `>
            if self.top_left().any("+\\") 
                && self.right().is('>'){
                elm.push(fragments::arrow_line(
                    &top_left().m(), &right().f()));
                consumed.push(right());
                if self.top_left().is('\\'){
                    consumed.push(top_left());
                }
            } 
            // for circuitries
            //     +    /
            //   <'   <'
            if self.top_right().any("+/")
                && self.left().is('<'){
                elm.push(fragments::arrow_line(
                    &top_right().m(), &left().j()));
                consumed.push(left());
                if self.top_right().is('/'){
                    consumed.push(top_right());
                }
            }
            //     .  
            //    '   
            if self.top_right().any(".,"){
                elm.push(fragments::line(c, &top_right().m()));
                consumed.push(top_right());
            }
            //   .  
            //    ' 
            if self.top_left().any(".,"){
                elm.push(fragments::line(c, &top_left().m()));
                consumed.push(top_left());
            }
            //   .'
            if self.left().any(".,"){
                elm.push(fragments::line(c, &left().m()));
                consumed.push(left());
            }
            //   '.
            if self.right().any(".,"){
                elm.push(fragments::line(c, &right().m()));
                consumed.push(right());
            }
        }
        else if self.any(".,"){
            // for circuitries
            //   <.    <,
            //     +     \
            if self.bottom_right().any("+\\")
                && self.left().is('<'){
                elm.push(fragments::arrow_line(
                    &bottom_right().m(), &left().t()));
                consumed.push(left());
                if self.bottom_right().is('\\'){
                    consumed.push(bottom_right());
                }
            }
            // for circuitries
            //   .>    ,>   ,>
            //  +     +    /
            if self.bottom_left().any("+/") 
                && self.right().is('>'){
                elm.push(fragments::arrow_line(
                    &bottom_left().m(), &right().p()));
                consumed.push(right());
                if self.bottom_left().is('/'){
                    consumed.push(bottom_left());
                }
            } 
        }
        else if self.is('_'){
            //   _|
            if self.right().any("|["){
                elm.push(fragments::line(u, &right().w()));
            }
            //    |_
            if self.left().any("|]"){
                elm.push(fragments::line(y, &left().w()));
            }
            //    _
            //   |
            if self.bottom_left().any("|]"){
                elm.push(fragments::line(y, &left().w()));
            }
            //    _
            //     |
            if self.bottom_right().any("|["){
                elm.push(fragments::line(u, &right().w()));
            }
            if enable_aggressive_underscore{
                //    /_
                if self.left().is('/'){
                    elm.push(fragments::line(y, &left().u()));
                }
                if self.right().is('\\'){
                    //     _\
                    elm.push(fragments::line(u, &right().y()));
                }
            }
        }
        else if self.is('-'){
            //   -|   -/   -\   x-  X-
            if self.right().any("|/\\xX<"){
               elm.push(fragments::line(k, &right().m())); 
            }
            //  |-   /-    /-   x-  X-
            if self.left().any("|/\\xX>"){
                elm.push(fragments::line(o, &left().m()));
            }
            //  -O  the O has radius of 3 units, so this line
            //  must only run from k to n only
            if self.right().is('O'){
                elm.push(fragments::line(k, n));
            }
            //   O-
            if self.left().is('O'){
                elm.push(fragments::line(o,l));
            }
            // circuit jump
            //    |
            //  _.-._
            //    |
            if self.top().can_strongly_connect(&W)
                && self.bottom().can_strongly_connect(&C)
                && self.left().is('.')
                && self.in_left(2).is('_')
                && self.right().is('.')
                && self.in_right(2).is('_'){

                elm.extend(vec![
                    line(&left().u(), &left().w()),
                    arc(&right().w(), &left().w(),5),
                    line(&right().w(),&right().y()),
                    line(c,w),
                ]);
                consumed.extend(vec![
                    left(), right()
                ]);
            }
        }
        // circuitries jump
        //    |
        //   -(-
        //    |
        //
        else if self.is('(')
            && self.top().can_strongly_connect(&W)
            && self.bottom().can_strongly_connect(&C)
            && self.left().can_strongly_connect(&O)
            && self.right().can_strongly_connect(&K){

            elm.extend(vec![
                arc(c, w, 5),
                line(k,o),
            ]);
        }
        // circuitries jump
        //    |
        //   -)-
        //    |
        //
        else if self.is(')')
            && self.top().can_strongly_connect(&W)
            && self.bottom().can_strongly_connect(&C)
            && self.left().can_strongly_connect(&O)
            && self.right().can_strongly_connect(&K){

            elm.extend(vec![
                arc(w, c, 5),
                line(k,o),
            ]);
        }
        //     |
        //    -~-
        //     |
        else if self.is('~')
            && self.top().can_strongly_connect(&W)
            && self.bottom().can_strongly_connect(&C)
            && self.left().can_strongly_connect(&O)
            && self.right().can_strongly_connect(&K){
                elm.extend(vec![
                    line(&left().k(), &left().m()),
                    arc(&right().m(), &left().m(),5),
                    line(&right().o(), &right().m()),
                    line(c, &bottom().w()),
                ]);
                consumed.extend(vec![
                    left(),
                    right(),
                ]);
            }
        (elm, consumed)
    }
}
