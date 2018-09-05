use focus_char::FocusChar;
use fragments::Fragment;
use location::Location;
use location::Direction::{Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight};
use block::Block::{A, C, E, F, J, K, M, O, P, Q, S, T, U, W, Y};
use point_block::PointBlock;
use fragments::{line, arc, arrow_line, open_circle};

pub trait Enhance {
    fn enhance(&self) -> (Vec<Fragment>, Vec<Location>);
}

impl<'g> Enhance for FocusChar<'g> {

    fn enhance(&self) -> (Vec<Fragment>, Vec<Location>) {
        let mut elm = vec![];
        let mut consumed = vec![];
        let a = &PointBlock::block(A);
        //let _b = &PointBlock::block(B);
        let c = &PointBlock::block(C);
        //let _d = &PointBlock::block(D);
        let e = &PointBlock::block(E);
        let f = &PointBlock::block(F);
        //let _g = &PointBlock::block(G);
        //let _h = &PointBlock::block(H);
        //let _i = &PointBlock::block(I);
        let j = &PointBlock::block(J);
        let k = &PointBlock::block(K);
        //let _l = &PointBlock::block(L);
        let m = &PointBlock::block(M);
        //let _n = &PointBlock::block(N);
        let o = &PointBlock::block(O);
        let p = &PointBlock::block(P);
        let q = &PointBlock::block(Q);
        //let r = &PointBlock::block(R);
        let s = &PointBlock::block(S);
        let t = &PointBlock::block(T);
        let u = &PointBlock::block(U);
        //let _v = &PointBlock::block(V);
        let w = &PointBlock::block(W);
        //let _x = &PointBlock::block(X);
        let y = &PointBlock::block(Y);

        let this = || Location::this();
        let top = || Location::go(Top);
        let bottom = || Location::go(Bottom);
        let left = || Location::go(Left);
        let right = || Location::go(Right);
        let top_left = || Location::go(TopLeft);
        let top_right = || Location::go(TopRight);
        let bottom_left = || Location::go(BottomLeft);
        let bottom_right = || Location::go(BottomRight);
        //let top2 = || Location::jump(Top,2);

        let top_left2 = || top().go_left(2);
        let top_right2 = || top().go_right(2);
        let bottom_right2 = || bottom().go_right(2); 
        let bottom_left2 = || bottom().go_left(2); 
        //let top2_right = || top2().right();

        // _ underscore
        if self.is('_') {
            //   _|
            if self.right().any("|[") {
                elm.push(line(u, &right().w()));
            }
            //    |_
            if self.left().any("|]") {
                elm.push(line(y, &left().w()));
            }
        }
        if self.any("`'") {
            // for circuitries
            //  +     + 
            //   `>    '> 
            if self.top_left().is('+') && self.right().is('>') {
                elm.push(arrow_line(&top_left().m(), &right().f()));
                consumed.extend(vec![this(), right()]);
            }
            //      \
            //       `>
            if self.top_left().is('\\') && self.right().is('>') {
                elm.push(arrow_line(&top_left().c(), &right().f()));
                consumed.extend(vec![this(), right(), top_left()]);
            }
            // 
            //        +
            //      <'
            if self.top_right().is('+') && self.left().is('<') {
                elm.push(arrow_line(&top_right().m(), &left().j()));
                consumed.extend(vec![this(), left()]);
            }
            // 
            //        /
            //      <'
            if self.top_right().is('/') && self.left().is('<') {
                elm.push(arrow_line(&top_right().c(), &left().j()));
                consumed.extend(vec![this(), left(),top_right()]);
            }
            // For diamond rectangle
            //     .
            //    '
            if self.top_right().any(".,") {
                elm.push(line(c, &top_right().m()));
                consumed.extend(vec![this(), top_right()]);
            }
            //   .
            //    '
            if self.top_left().any(".,") {
                elm.push(line(c, &top_left().m()));
                consumed.extend(vec![this(), top_left()]);
            }
            //   .'
            if self.left().any(".,") {
                elm.push(line(c, &left().m()));
                consumed.extend(vec![this(),left()]);
            }
            //   '.
            if self.right().any(".,") {
                elm.push(line(c, &right().m()));
                consumed.extend(vec![this(),right()]);
            }
        }
        if self.any(".,") {
            // for circuitries
            //   <.  <, 
            //     +   +
            if self.bottom_right().is('+') && self.left().is('<') {
                elm.push(arrow_line(&bottom_right().m(), &left().t()));
                consumed.extend(vec![this(),left()]);
            }
            //       <,   <.
            //         \    \
            if self.bottom_right().is('\\') && self.left().is('<') {
                elm.push(arrow_line(&bottom_right().w(), &left().t()));
                consumed.extend(vec![this(),left(), bottom_right()]);
            }
            // 
            //   .>    ,> 
            //  +     +    
            if self.bottom_left().is('+') && self.right().is('>') {
                elm.push(arrow_line(&bottom_left().m(), &right().p()));
                consumed.extend(vec![this(),right()]);
            }
            // 
            //       ,>
            //      /
            if self.bottom_left().is('/') && self.right().is('>') {
                elm.push(arrow_line(&bottom_left().w(), &right().p()));
                consumed.extend(vec![this(),right(), bottom_left()]);
            }
        }
        // transistor complimentary enhancement
        if self.is('|') {
            //    |    |
            //    <    >
            if self.bottom().any("><") {
                elm.push(line(c, &bottom().m()));
                consumed.push(this());
            }
            //    <    >
            //    |    |
            if self.top().any("><") {
                elm.push(line(w, &top().m()));
                consumed.push(this());
            }
            //    _
            //   |
            if self.top_right().is('_') {
                elm.extend(vec![line(c,w),line(c, e)]);
                consumed.push(this());
            }
            //    _
            //     |
            if self.top_left().is('_') {
                elm.extend(vec![line(c,w),line(a,c)]);
                consumed.push(this());
            }
        } 
        if self.is('/') {
            //      >
            //     /
            if self.top_right().is('>') {
                elm.push(line(u, &top_right().m()));
                consumed.push(this());
            }
            //    /
            //   <
            if self.bottom_left().is('<') {
                elm.push(line(e, &bottom_left().m()));
                consumed.push(this());
            }
        } 
        if self.is('\\') {
            //      \
            //       >
            if self.bottom_right().is('>') {
                elm.push(line(a, &bottom_right().m()));
                consumed.push(this());
            }
            //    <
            //     \
            if self.top_left().is('<') {
                elm.push(line(y, &top_left().m()));
                consumed.push(this());
            }
        }

        if self.any("vV"){
            //     `.
            //       V
            if self.top_left().is('.') && self.top().in_left(2).is('`'){
                elm.push(arrow_line(&top_left2().c(), j));
                consumed.push(this())
            }
            //    .'
            //   V
            if self.top_right().is('.') && self.top().in_right(2).is('\''){
                elm.push(arrow_line(&top_right2().c(), f));
                consumed.push(this())
            }
        }
        if self.is('^'){
            //  ^
            //   `.
            if self.bottom_right().is('`') && self.bottom().in_right(2).is('.'){
                elm.push(arrow_line(&bottom_right2().t(), m));
                consumed.push(this());
            }
            //    ^
            //  .'
            if self.bottom_left().is('\'') && self.bottom().in_left(2).is('.') {
                elm.push(arrow_line(&bottom_left2().p(), m));
                consumed.push(this());
            }
        }
        // circuitries jump
        //    |
        //   -(-
        //    |
        //
       if self.is('(') && self.top().can_strongly_connect(&W)
            && self.bottom().can_strongly_connect(&C)
            && self.left().can_strongly_connect(&O)
            && self.right().can_strongly_connect(&K)
        {
            elm.extend(vec![arc(c, w, 5), line(k, o)]);
            consumed.push(this());
        }
        // circuitries jump
        //    |
        //   -)-
        //    |
        //
        if self.is(')') && self.top().can_strongly_connect(&W)
            && self.bottom().can_strongly_connect(&C)
            && self.left().can_strongly_connect(&O)
            && self.right().can_strongly_connect(&K)
        {
            elm.extend(vec![arc(w, c, 5), line(k, o)]);
            consumed.push(this());
        }
        // railroad diagram
        // _◞_
        if self.is('◞') && self.left().is('_') && self.right().is('_'){
            elm.extend(vec![line(u,y)]);
        }
        // railroad diagram
        // _◟_
        if self.is('◟') && self.left().is('_') && self.right().is('_'){
            elm.extend(vec![line(u,y)]);
        }
        // railroad diagram
        //
        // -╯-  -╰-  -╭-  -╮-
        //
        if self.any("╯╮╰╭") && self.left().is('-') && self.right().is('-'){
            elm.extend(vec![line(k,o)]);
        }
        // |    |
        // ╰    ╯
        // |    |
        if self.any("╰╯") && self.top().is('|') && self.bottom().is('|'){
            elm.extend(vec![line(c,w)]);
        }
        // railroad start
        // O_
        if self.is('O') && self.right().is('_'){
            elm.extend(vec![open_circle(m,3), arc(t,&right().y(),4)]);
            consumed.extend(vec![this(), right()]);
        }
        // railroad end
        // _O
        if self.is('O') && self.left().is('_'){
            elm.extend(vec![open_circle(m,3), arc(&left().u(), p,4)]);
            consumed.extend(vec![this(), left()]);
        }
        // railroad start
        // o_
        if self.is('o') && self.right().is('_'){
            elm.extend(vec![open_circle(m,2), 
                       arc(s,&right().w(),4), 
                       line(&right().w(), &right().y())]);
            consumed.extend(vec![this(), right()]);
        }
        // railroad end
        // _o
        if self.is('o') && self.left().is('_'){
            elm.extend(vec![open_circle(m,2), 
                       arc(&left().w(), q, 4), 
                       line(&left().w(), &left().u())]);
            consumed.extend(vec![this(), left()]);
        }
        (elm, consumed)
    }
}
