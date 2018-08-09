
use focus_char::FocusChar;
use fragments::Fragment;
use location::Location;
use location::Direction::{Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight};
use block::Block::{K, M, O, W, Y};
use point_block::PointBlock;
use fragments::open_circle;

pub trait EnhanceCircle {
    fn enhance_circle(&self) -> (Vec<Fragment>, Vec<Location>);
}

impl<'g> EnhanceCircle for FocusChar<'g> {

    fn enhance_circle(&self) -> (Vec<Fragment>, Vec<Location>) {
        let mut elm = vec![];
        let mut consumed = vec![];

        //let _a = &PointBlock::block(A);
        //let _b = &PointBlock::block(B);
        //let c = &PointBlock::block(C);
        //let _d = &PointBlock::block(D);
        //let e = &PointBlock::block(E);
        //let _f = &PointBlock::block(F);
        //let _g = &PointBlock::block(G);
        //let _h = &PointBlock::block(H);
        //let _i = &PointBlock::block(I);
        //let _j = &PointBlock::block(J);
        //let k = &PointBlock::block(K);
        //let _l = &PointBlock::block(L);
        let m = &PointBlock::block(M);
        //let _n = &PointBlock::block(N);
        let o = &PointBlock::block(O);
        //let _p = &PointBlock::block(P);
        //let _q = &PointBlock::block(Q);
        //let _r = &PointBlock::block(R);
        //let _s = &PointBlock::block(S);
        //let _t = &PointBlock::block(T);
        //let u = &PointBlock::block(U);
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

        let top2 = || Location::jump(Top,2);
        let top2_right = || Location::jump(Top,2).right();
        let top2_right2 = || Location::jump(Top,2).go_right(2);
        let top2_right3 = || Location::jump(Top,2).go_right(3);
        let top2_right4 = || Location::jump(Top,2).go_right(4);
        let top2_right5 = || Location::jump(Top,2).go_right(5);
        let top2_left = || Location::jump(Top,2).left();
        let top2_left2 = || Location::jump(Top,2).go_left(2);
        let top2_left3 = || Location::jump(Top,2).go_left(3);
        let top2_left4 = || Location::jump(Top,2).go_left(4);
        let bottom2 = || Location::jump(Bottom,2);
        let bottom2_left = || Location::jump(Bottom,2).left();
        let bottom2_left2 = || Location::jump(Bottom,2).go_left(2);
        let bottom2_left3 = || Location::jump(Bottom,2).go_left(3);
        let bottom2_left4 = || Location::jump(Bottom,2).go_left(4);
        let bottom2_left5 = || Location::jump(Bottom,2).go_left(5);
        let bottom2_right = || Location::jump(Bottom, 2).right();
        let bottom2_right2 = || Location::jump(Bottom, 2).go_right(2);
        let bottom2_right3 = || Location::jump(Bottom, 2).go_right(3);
        let bottom2_right4 = || Location::jump(Bottom, 2).go_right(4);
        let bottom2_right5 = || Location::jump(Bottom, 2).go_right(5);
        let bottom2_right6 = || Location::jump(Bottom, 2).go_right(6);
        let left2 = || Location::jump(Left,2);
        let right2 = || Location::jump(Right,2);
        let top_right2 = || top().go_right(2);
        let top_left2 = || top().go_left(2);
        let bottom_right2 = || bottom().go_right(2);
        let bottom_left2 = || bottom().go_left(2);

        let top3 = || Location::jump(Top,3);
        let top3_left = || Location::jump(Top,3).left();
        let top3_left2 = || Location::jump(Top,3).go_left(2);
        let top3_right = || Location::jump(Top,3).right();
        let top3_right2 = || Location::jump(Top,3).go_right(2);
        let top3_right3 = || Location::jump(Top,3).go_right(3);
        let bottom3 = || Location::jump(Bottom,3);
        let bottom3_right = || Location::jump(Bottom, 3).right();
        let bottom3_right2 = || Location::jump(Bottom,3).go_right(2);
        let bottom3_right3 = || Location::jump(Bottom,3).go_right(3);
        let bottom3_right4 = || Location::jump(Bottom,3).go_right(4);
        let bottom3_right5 = || Location::jump(Bottom,3).go_right(5);
        let bottom3_left = || Location::jump(Bottom, 3).left();
        let bottom3_left2 = || Location::jump(Bottom,3).go_left(2);
        let bottom3_left3 = || Location::jump(Bottom,3).go_left(3);
        let bottom3_left4 = || Location::jump(Bottom,3).go_left(4);
        let bottom3_left5 = || Location::jump(Bottom,3).go_left(5);
        let left3 = || Location::jump(Left,3);
        let right3 = || Location::jump(Right,3);
        let top_right3 = || top().go_right(3);
        let top_left3 = || top().go_left(3);
        let bottom_right3 = || bottom().go_right(3);
        let bottom_left3 = || bottom().go_left(3);

        let top4 = || Location::jump(Top,4);
        let bottom4 = || Location::jump(Bottom,4);
        let left4 = || Location::jump(Left,4);
        let right4 = || Location::jump(Right,4);
        let top_right4 = || top().go_right(4);
        let top_left4 = || top().go_left(4);
        let bottom_right4 = || bottom().go_right(4);
        let bottom_left4 = || bottom().go_left(4);

        let top5 = || Location::jump(Top,5);
        let bottom5 = || Location::jump(Bottom,5);
        let left5 = || Location::jump(Left,5);
        let right5 = || Location::jump(Right,5);
        let top_right5 = || top().go_right(5);
        let top_left5 = || top().go_left(5);
        let bottom_right5 = || bottom().go_right(5);
        let bottom_left5 = || bottom().go_left(5);

        let top6 = || Location::jump(Top,6);
        let bottom6 = || Location::jump(Bottom,6);
        let left6 = || Location::jump(Left,6);
        let right6 = || Location::jump(Right,6);
        let top_right6 = || top().go_right(6);
        let top_left6 = || top().go_left(6);
        let bottom_right6 = || bottom().go_right(6);
        let bottom_left6 = || bottom().go_left(6);

        let top7 = || Location::jump(Top,7);
        let bottom7 = || Location::jump(Bottom,7);
        let left7 = || Location::jump(Left,7);
        let right7 = || Location::jump(Right,7);
        let top_right7 = || top().go_right(7);
        let top_left7 = || top().go_left(7);
        let bottom_right7 = || bottom().go_right(7);
        let bottom_left7 = || bottom().go_left(7);


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
            && self.top().can_be_strong_all_blocks(vec![K,O])
            && self.bottom().can_be_strong_all_blocks(vec![K,O])
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
            && self.top().can_be_strong_all_blocks(vec![K,O])
            && self.top_right().can_be_strong_all_blocks(vec![K,O])
            && self.top().in_right(2).is('.')
            && self.bottom_left().any("`'")
            && self.bottom().can_be_strong_all_blocks(vec![K,O])
            && self.bottom_right().can_be_strong_all_blocks(vec![K,O])
            && self.bottom().in_right(2).is('\''){
            elm.push(open_circle(o,10));
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
        if self.in_left(3).is('(')
            && self.in_right(4).is(')')
            && self.in_top(2).is('_')
            && self.in_top(2).right().is('_')
            && self.bottom().is('_')
            && self.bottom_right().is('_')
            && self.top().in_left(2).is(',')
            && self.top_left().is('\'')
            && self.top().in_right(2).is('`')
            && self.top().in_right(3).is('.')
            && self.bottom().in_left(2).is('`')
            && self.bottom_left().is('.')
            && self.bottom().in_right(2).is(',')
            && self.bottom().in_right(3).is('\''){

            elm.push(open_circle(o, 14));
            consumed.extend(vec![
                left3(), right4(), top2(), top2_right(), bottom(), bottom_right(),
                top_left2(), top_left(), top_right2(), top_right3(), bottom_left2(),
                bottom_left(), bottom_right2(), bottom_right3()
            ]);
        }
        //        ___
        //      ,'   `.
        //     /   +   \  16
        //     \       /
        //      `.___,'
        //
        if self.in_left(4).is('/')
            && self.top().in_left(3).is(',')
            && self.top().in_left(2).is('\'')
            && self.in_top(2).left().is('_')
            && self.in_top(2).is('_')
            && self.in_top(2).right().is('_')
            && self.top().in_right(2).is('`')
            && self.top().in_right(3).is('.')
            && self.in_right(4).is('\\')
            && self.bottom().in_right(4).is('/')
            && self.in_bottom(2).in_right(3).is('\'')
            && self.in_bottom(2).in_right(2).is(',')
            && self.in_bottom(2).right().is('_')
            && self.in_bottom(2).is('_')
            && self.in_bottom(2).left().is('_')
            && self.in_bottom(2).in_left(2).is('.')
            && self.in_bottom(2).in_left(3).is('`')
            && self.bottom().in_left(4).is('\\'){
            
                elm.push(open_circle(w,16));
                consumed.extend(vec![
                    left4(), top_left3(), top_left2(), top2_left(), top2(),
                    top2_right(), top_right2(), top_right3(),
                    right4(), bottom_right4(), bottom2_right3(),
                    bottom2_right2(), bottom2_right(), bottom2(),
                    bottom2_left(), bottom2_left2(), bottom2_left3(),
                    bottom_left4()
                ]);
        }
        //        ____
        //      ,'    `.
        //     /   +    \  18
        //     \        /
        //      `.____,'
        //
        if self.in_left(4).is('/')
            && self.top().in_left(3).any(".,")
            && self.top().in_left(2).is('\'')
            && self.in_top(2).left().is('_')
            && self.in_top(2).is('_')
            && self.in_top(2).right().is('_')
            && self.in_top(2).in_right(2).is('_')
            && self.top().in_right(3).is('`')
            && self.top().in_right(4).is('.')
            && self.in_right(5).is('\\')
            && self.bottom().in_right(5).is('/')
            && self.in_bottom(2).in_right(4).is('\'')
            && self.in_bottom(2).in_right(3).any(".,")
            && self.in_bottom(2).in_right(2).is('_')
            && self.in_bottom(2).right().is('_')
            && self.in_bottom(2).is('_')
            && self.in_bottom(2).left().is('_')
            && self.in_bottom(2).in_left(2).is('.')
            && self.in_bottom(2).in_left(3).is('`')
            && self.bottom().in_left(4).is('\\') {
                elm.push(open_circle(y, 18));
                consumed.extend(vec![
                    left4(), top_left3(), top_left2(), top2_left(), top2(),
                    top2_right(), top2_right2(), top_right3(), top_right4(),
                    right5(), bottom_right5(), bottom2_right4(), bottom2_right3(),
                    bottom2_right2(), bottom2_right(), bottom2(), bottom2_left(),
                    bottom2_left2(), bottom2_left3(), bottom_left4()
                ]);
            }
        //        ____
        //      ,'    `.
        //     /        \
        //    (    +     )  20
        //     \        /
        //      `.____,'
        //
        if self.in_left(5).is('(')
            && self.top().in_left(4).is('/')
            && self.in_top(2).in_left(3).any(",.")
            && self.in_top(2).in_left(2).is('\'')
            && self.in_top(3).left().is('_')
            && self.in_top(3).is('_')
            && self.in_top(3).right().is('_')
            && self.in_top(3).in_right(2).is('_')
            && self.in_top(2).in_right(3).is('`')
            && self.in_top(2).in_right(4).is('.')
            && self.top().in_right(5).is('\\')
            && self.in_right(6).is(')')
            && self.bottom().in_right(5).is('/')
            && self.in_bottom(2).in_right(4).is('\'')
            && self.in_bottom(2).in_right(3).any(",.")
            && self.in_bottom(2).in_right(2).is('_')
            && self.in_bottom(2).right().is('_')
            && self.in_bottom(2).is('_')
            && self.in_bottom(2).left().is('_')
            && self.in_bottom(2).in_left(2).is('.')
            && self.in_bottom(2).in_left(3).any("`'")
            && self.bottom().in_left(4).is('\\'){
                elm.push(open_circle(o, 20));
                consumed.extend(vec![
                    left5(), top_left4(), top2_left3(), top2_left2(),
                    top3_left(), top3(), top3_right(), top3_right2(),
                    top2_right3(), top2_right4(), top_right5(),
                    right6(), bottom_right5(), bottom2_right4(),
                    bottom2_right3(), bottom2_right2(), bottom2_right(),
                    bottom2(), bottom2_left(), bottom2_left2(),
                    bottom2_left3(), bottom_left4(),
                ]);
        }
        //
        //        _____
        //      ,'     `.
        //     /         \
        //    (     +     )  22
        //     \         /
        //      `._____,'
        //
        if self.in_left(6).is('(')
            && self.top().in_left(5).is('/')
            && self.in_top(2).in_left(4).any(",.")
            && self.in_top(2).in_left(3).is('\'')
            && self.in_top(3).in_left(2).is('_')
            && self.in_top(3).left().is('_')
            && self.in_top(3).is('_')
            && self.in_top(3).right().is('_')
            && self.in_top(3).in_right(2).is('_')
            && self.in_top(2).in_right(3).is('`')
            && self.in_top(2).in_right(4).is('.')
            && self.top().in_right(5).is('\\')
            && self.in_right(6).is(')')
            && self.bottom().in_right(5).is('/')
            && self.in_bottom(2).in_right(4).is('\'')
            && self.in_bottom(2).in_right(3).any(".,")
            && self.in_bottom(2).in_right(2).is('_')
            && self.in_bottom(2).right().is('_')
            && self.in_bottom(2).is('_')
            && self.in_bottom(2).left().is('_')
            && self.in_bottom(2).in_left(2).is('_')
            && self.in_bottom(2).in_left(3).is('.')
            && self.in_bottom(2).in_left(4).is('`')
            && self.bottom().in_left(5).is('\\') {

            elm.push(open_circle(m, 22));
            consumed.extend(vec![
                left6(), top_left5(), top2_left4(), top2_left3(),
                top2_left2(), top3_left2(), top3_left(), top3(),
                top3_right(), top3_right2(), top2_right3(), top2_right4(),
                top_right5(), right6(), bottom_right5(),
                bottom2_right4(), bottom2_right3(), bottom2_right2(), bottom2_right(),
                bottom2(), bottom2_left(), bottom2_left2(), bottom2_left3(),
                bottom2_left4(), bottom_left5()
            ]);
        }
        //
        //        ______
        //      ,'      `.
        //     /          \
        //    |     +      | 24
        //    |            |
        //     \          /
        //      `.______,'
        //
        if self.in_left(6).is('|')
            && self.top().in_left(5).is('/')
            && self.in_top(2).in_left(4).any(".,")
            && self.in_top(2).in_left(3).is('\'')
            && self.in_top(3).in_left(2).is('_')
            && self.in_top(3).left().is('_')
            && self.in_top(3).is('_')
            && self.in_top(3).right().is('_')
            && self.in_top(3).in_right(2).is('_')
            && self.in_top(3).in_right(3).is('_')
            && self.in_top(2).in_right(4).is('`')
            && self.in_top(2).in_right(5).is('.')
            && self.top().in_right(6).is('\\')
            && self.in_right(7).is('|')
            && self.bottom().in_right(7).is('|')
            && self.in_bottom(2).in_right(6).is('/')
            && self.in_bottom(3).in_right(5).is('\'')
            && self.in_bottom(3).in_right(4).any(".,")
            && self.in_bottom(3).in_right(3).is('_')
            && self.in_bottom(3).in_right(2).is('_')
            && self.in_bottom(3).right().is('_')
            && self.in_bottom(3).is('_')
            && self.in_bottom(3).left().is('_')
            && self.in_bottom(3).in_left(2).is('_')
            && self.in_bottom(3).in_left(3).is('.')
            && self.in_bottom(3).in_left(4).is('`')
            && self.in_bottom(2).in_left(5).is('\\')
            && self.bottom().in_left(6).is('|')
        {
            elm.push(open_circle(y,24));
            consumed.extend(vec![
                left6(), top_left5(), top2_left4(), top2_left3(),
                top3_left2(), top3_left(), top3(),
                top3_right(), top3_right2(), top3_right3(),
                top2_right4(), top2_right5(), top_right6(),
                right7(), bottom_right7(),
                bottom2_right6(), bottom3_right5(), bottom3_right4(),
                bottom3_right3(), bottom3_right2(), bottom3_right(),
                bottom3(), bottom3_left(), bottom3_left2(), bottom3_left3(),
                bottom3_left4(), bottom2_left5(), bottom_left6()
            ]);
        }
        (elm, consumed)
    }
}

