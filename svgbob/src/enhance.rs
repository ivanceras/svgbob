use focus_char::FocusChar;
use fragments::Fragment;
use location::Location;
use location::Direction::{Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight};
use block::Block::{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y};
use point_block::PointBlock;
use fragments::{self, line, arc, arrow_line, dashed_line};

pub trait Enhance {
    fn enhance(&self) -> (Vec<Fragment>, Vec<Location>);
}

impl<'g> Enhance for FocusChar<'g> {
    fn enhance(&self) -> (Vec<Fragment>, Vec<Location>) {
        let mut elm = vec![];
        let mut consumed = vec![];
        let a = &PointBlock::block(A);
        let _b = &PointBlock::block(B);
        let c = &PointBlock::block(C);
        let _d = &PointBlock::block(D);
        let e = &PointBlock::block(E);
        let _f = &PointBlock::block(F);
        let _g = &PointBlock::block(G);
        let _h = &PointBlock::block(H);
        let _i = &PointBlock::block(I);
        let _j = &PointBlock::block(J);
        let k = &PointBlock::block(K);
        let l = &PointBlock::block(L);
        let _m = &PointBlock::block(M);
        let n = &PointBlock::block(N);
        let o = &PointBlock::block(O);
        let _p = &PointBlock::block(P);
        let _q = &PointBlock::block(Q);
        let _r = &PointBlock::block(R);
        let _s = &PointBlock::block(S);
        let _t = &PointBlock::block(T);
        let u = &PointBlock::block(U);
        let _v = &PointBlock::block(V);
        let w = &PointBlock::block(W);
        let _x = &PointBlock::block(X);
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
        else if self.any("`'") {
            // for circuitries
            //  +     +    \
            //   `>    '>   `>
            if self.top_left().any("+\\") && self.right().is('>') {
                elm.push(fragments::arrow_line(&top_left().m(), &right().f()));
                consumed.push(right());
                if self.top_left().is('\\') {
                    consumed.push(top_left());
                }
            }
            // for circuitries
            //     +    /
            //   <'   <'
            if self.top_right().any("+/") && self.left().is('<') {
                elm.push(fragments::arrow_line(&top_right().m(), &left().j()));
                consumed.push(left());
                if self.top_right().is('/') {
                    consumed.push(top_right());
                }
            }
            // For diamon rectanle
            //     .
            //    '
            if self.top_right().any(".,") {
                elm.push(fragments::line(c, &top_right().m()));
                consumed.push(top_right());
            }
            //   .
            //    '
            if self.top_left().any(".,") {
                elm.push(fragments::line(c, &top_left().m()));
                consumed.push(top_left());
            }
            //   .'
            if self.left().any(".,") {
                elm.push(fragments::line(c, &left().m()));
                consumed.push(left());
            }
            //   '.
            if self.right().any(".,") {
                elm.push(fragments::line(c, &right().m()));
                consumed.push(right());
            }
        } else if self.any(".,") {
            // for circuitries
            //   <.    <,
            //     +     \
            if self.bottom_right().any("+\\") && self.left().is('<') {
                elm.push(fragments::arrow_line(&bottom_right().m(), &left().t()));
                consumed.push(left());
                if self.bottom_right().is('\\') {
                    consumed.push(bottom_right());
                }
            }
            // for circuitries
            //   .>    ,>   ,>
            //  +     +    /
            if self.bottom_left().any("+/") && self.right().is('>') {
                elm.push(fragments::arrow_line(&bottom_left().m(), &right().p()));
                consumed.push(right());
                if self.bottom_left().is('/') {
                    consumed.push(bottom_left());
                }
            }
        }
        // transistor complimentary enhancement
        else if self.is('|') {
            //    |    |
            //    <    >
            if self.bottom().any("><") {
                elm.push(line(c, &bottom().m()));
            }
            //    <    >
            //    |    |
            if self.top().any("><") {
                elm.push(line(w, &top().m()));
            }
            //    _
            //   |
            if self.top_right().is('_') {
                elm.extend(vec![line(c,w),line(c, e)]);
            }
            //    _
            //     |
            if self.top_left().is('_') {
                elm.extend(vec![line(c,w),line(a,c)]);
            }
        } else if self.is('/') {
            //      >
            //     /
            if self.top_right().is('>') {
                elm.push(line(u, &top_right().m()));
            }
            //    /
            //   <
            if self.bottom_left().is('<') {
                elm.push(line(e, &bottom_left().m()));
            }
        } else if self.is('\\') {
            //      \
            //       >
            if self.bottom_right().is('>') {
                elm.push(line(a, &bottom_right().m()));
            }
            //    <
            //     \
            if self.top_left().is('<') {
                elm.push(line(y, &top_left().m()));
            }
        }
        // circuitries jump
        //    |
        //   -(-
        //    |
        //
        else if self.is('(') && self.top().can_strongly_connect(&W)
            && self.bottom().can_strongly_connect(&C)
            && self.left().can_strongly_connect(&O)
            && self.right().can_strongly_connect(&K)
        {
            elm.extend(vec![arc(c, w, 5), line(k, o)]);
            //consumed.push(this());
        }
        // circuitries jump
        //    |
        //   -)-
        //    |
        //
        else if self.is(')') && self.top().can_strongly_connect(&W)
            && self.bottom().can_strongly_connect(&C)
            && self.left().can_strongly_connect(&O)
            && self.right().can_strongly_connect(&K)
        {
            elm.extend(vec![arc(w, c, 5), line(k, o)]);
            //consumed.push(this());
        }

        (elm, consumed)
    }
}
