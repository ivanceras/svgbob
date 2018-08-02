
use focus_char::FocusChar;
use fragments::Fragment;
use location::Location;
use location::Direction::{Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight};
use block::Block::{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y};
use point_block::PointBlock;
use fragments::{self, line, arc, open_circle, arrow_line, dashed_line};
use properties::Can::{ConnectTo,Is,IsStrongAll};

pub trait EnhanceCircle {
    fn enhance_circle(&self) -> (Vec<Fragment>, Vec<Location>);
}

impl<'g> EnhanceCircle for FocusChar<'g> {

    fn enhance_circle(&self) -> (Vec<Fragment>, Vec<Location>) {
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
        let m = &PointBlock::block(M);
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

        let top2 = || Location::jump(Top,2);
        let bottom2 = || Location::jump(Bottom,2);
        let left2 = || Location::jump(Left,2);
        let right2 = || Location::jump(Right,2);
        let top_right2 = || top().go_right(2);
        let top_left2 = || top().go_left(2);
        let bottom_right2 = || bottom().go_right(2);
        let bottom_left2 = || bottom().go_left(2);

        let top3 = || Location::jump(Top,3);
        let bottom3 = || Location::jump(Bottom,3);
        let left3 = || Location::jump(Left,3);
        let right3 = || Location::jump(Right,3);
        let top_right3 = || top().go_right(3);
        let top_left3 = || top().go_left(3);
        let bottom_right3 = || bottom().go_right(3);
        let bottom_left3 = || bottom().go_left(3);

        //  circle 4
        //   _
        //  (_)
        //
        if self.is('_') 
            && self.left().is('(') && self.right().is(')')
            && self.top().is('_'){
            elm.push(open_circle(m, 4));
            consumed.extend(vec![this(), left(), right(),top()]);
        }

        //  circle 6
        //   __
        //  (__)
        //
        if self.is('_') 
            && self.left().is('(') && self.in_right(2).is(')')
            && self.top().is('_') && self.top_right().is('_')
            && self.right().is('_'){
            elm.push(open_circle(m, 6));
            consumed.extend(vec![this(), left(), right2(), top(), top_right(), right()]);
        }

        // circle 8
        //       .-.
        //      ( + )
        //       '-'
        if self.in_left(2).is('(')
            && self.in_right(2).is(')')
            && self.top().is_satisfied(&IsStrongAll(vec![K,O]))
            && self.bottom().is_satisfied(&IsStrongAll(vec![K,O]))
            && self.bottom_left().any("`'")
            && self.bottom_right().is('\'')
            && self.top_left().any(".,")
            && self.top_right().is('.'){
            elm.push(open_circle(m,8));
            consumed.extend(vec![left2(), right2(), top(), bottom(),
                bottom_left(), bottom_right(), top_left(), top_right()]);
        }
        // circle 10
        //      .--.
        //     ( +  )
        //      `--'
        if self.in_left(2).is('(')
            && self.in_right(3).is(')')
            && self.top_left().any(".,")
            && self.top().is_satisfied(&IsStrongAll(vec![K,O]))
            && self.top_right().is('-')
            && self.top_right().right().is_satisfied(&IsStrongAll(vec![K,O]))
            && self.bottom_left().any("`'")
            && self.bottom().is_satisfied(&IsStrongAll(vec![K,O]))
            && self.bottom_right().is_satisfied(&IsStrongAll(vec![K,O]))
            && self.bottom_right().right().is('\''){
            elm.push(open_circle(&this().o(),10));
            consumed.extend(vec![left2(), right3(), top_left(), top(), top_right(),
                top_right2(), bottom_left(), bottom(), bottom_right(), bottom_right2(),
            ]);
        }

        // Circle 12
        //        _
        //      .' '.
        //     (  +  )
        //      `._.'
        if self.in_left(3).is('(')
            && self.in_right(3).is(')')
            && self.in_top(2).is('_')
            && self.bottom().is('_')
            && self.top().in_left(2).any(",.")
            && self.top_left().is('\'')
            && self.top_right().any("`'")
            && self.top().in_right(2).is('.')
            && self.bottom().in_left(2).any("`'")
            && self.bottom_left().is('.')
            && self.bottom_right().any(".,")
            && self.bottom().in_right(2).is('\''){
                elm.push(open_circle(m, 12));
                consumed.extend(vec![
                        left3(), right3(), top2(), bottom(), top_left2(), top_left(), top_right(), top_right2(),
                        bottom_left2(), bottom_left(), bottom_right(), bottom_right2()
                ]);
        }
        //
        //        __
        //      ,'  `.
        //     (  +   )  14
        //      `.__,'
        //
        //        ___
        //      ,'   `.
        //     /   +   \  16
        //     \       /
        //      `.___,'
        //
        //        ____
        //      ,'    `.
        //     /   +    \  18
        //     \        /
        //      `.____,'
        //
        //        ____
        //      ,'    `.
        //     /        \
        //    (    +     )  20
        //     \        /
        //      `.____,'
        //
        //
        //        _____
        //      ,'     `.
        //     /         \
        //    (     +     )  22
        //     \         /
        //      `._____,'
        //
        //
        //        ______
        //      ,'      `.
        //     /          \
        //    |     +      | 24
        //    |            |
        //     \          /
        //      `.______,'
        //
        (elm, consumed)
    }
}

