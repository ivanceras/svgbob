use focus_char::FocusChar;
use fragments::Fragment;
use location::Location;
use location::Direction::{Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight};
use block::Block::{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y};
use point_block::PointBlock;
use fragments::{self, line, arc, arrow_line};

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

        let top = ||Location::go(Top);
        let bottom = ||Location::go(Bottom);
        let left = ||Location::go(Left);
        let right = ||Location::go(Right);
        let top_left = ||Location::go(TopLeft);
        let top_right = ||Location::go(TopRight);
        let bottom_left = ||Location::go(BottomLeft);
        let bottom_right = ||Location::go(BottomRight);

        if self.is('_') {
            //   _|
            if self.right().any("|[") {
                elm.push(fragments::line(u, &right().w()));
            }
            //    |_
            if self.left().any("|]") {
                elm.push(fragments::line(y, &left().w()));
            }
            //    _
            //   |
            if self.bottom_left().any("|]") {
                elm.push(fragments::line(y, &left().w()));
            }
            //    _
            //     |
            if self.bottom_right().any("|[") {
                elm.push(fragments::line(u, &right().w()));
            }
            //    /_
            if self.left().is('/') {
                elm.push(fragments::line(y, &left().u()));
            }
            if self.right().is('\\') {
                //     _\
                elm.push(fragments::line(u, &right().y()));
            }
        }
        (elm, consumed)
    }
}
