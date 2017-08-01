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

pub trait Round{
    
    fn round(&self) -> (Vec<Fragment>, Vec<Location>, bool);
}


impl <'g>Round for FocusChar<'g>{
    ///
    /// Enhance drawings by making circular shapes into circle
    /// element
    ///  example:
    ///   _   .-.    .--.
    ///  (_) (   )  (    )
    ///       `-'    `--'
    ///  
    ///
    /// The circle is held by the element that is the center of the circle
    /// as oppused to arcs where the arcs
    /// are held by the element at its center along the arc points
    /// 
    /// Arc and circles will be treated differently
    fn round(&self) -> (Vec<Fragment>, Vec<Location>, bool) {
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


        let go_top = |step| Location::jump(Top,step);
        let go_bottom = |step| Location::jump(Bottom,step);
        let go_left = |step| Location::jump(Left,step);
        let go_right = |step| Location::jump(Right,step);
        let go_top_left = |step| Location::jump(TopLeft,step);
        let go_top_right = |step| Location::jump(TopRight,step);
        let go_bottom_left = |step| Location::jump(BottomLeft,step);
        let go_bottom_right = |step| Location::jump(BottomRight,step);

        let mut elm = vec![];
        let mut consumed = vec![];
        // tells whether that element
        // containing the arc/circle is along
        // with it, if along then no
        // additional processing is needed for this element
        // if not along, then further enhancement
        // intended behaviors would have to take effect
        let mut along_arc = false;

        let mut matched_arc = false;
        let mut matched_circle = false;
        let enable_round_pill = true;


        ////////////////////////////////
        //  Big Arc
        //
        //////////////////////////////////
        if self.any(".,"){

            //        _         _ 
            //      ,'|       .'| 
            //     /  |      /  | 
            //    |---+     |---+  
            if self.right().is('\'') 
                && self.top().in_right(2).is('_')
                && self.bottom_left().is('/') 
                && self.in_bottom(2).in_left(2).is('|'){

                    elm.extend(vec![arc(&top().go_right(2).u(), 
                                        &go_bottom(2).go_left(2).c(),
                                        20),
                                 line(&go_bottom(2).go_left(2).c(), 
                                      &go_bottom(2).go_left(2).w())
                              ]);
                    
                    consumed.extend(vec![
                                    right(), 
                                    bottom_left(),
                                    go_bottom(2).go_left(2)
                        ]);
                    matched_arc = true;
                    along_arc = true;
             }
             //     _
             //     |`.
             //     |  \
             //     +---|
             if self.left().is('`') 
                 && self.top().in_left(2).is('_')
                 && self.bottom_right().is('\\') 
                 && self.in_bottom(2).in_right(2).is('|'){

                 elm.extend(vec![
                         arc(&go_bottom(2).go_right(2).c(),
                             &top().go_left(2).y(),
                             20),
                         line(&go_bottom(2).go_right(2).c(),
                              &go_bottom(2).go_right(2).w())
                     ]); 
                 consumed.extend(vec![
                        left(),
                        bottom_right(),
                        go_bottom(2).go_right(2)
                 ]);
                 matched_arc = true;
                 along_arc = true;
             }

                
             
        }

        ////////////////////////////////
        // Bigarc bottom
        //
        //   |---+
        //    \  |
        //     `._
        //
        ///////////////////////////////
        if self.any("`'"){
            if self.right().is('.') 
                && self.in_right(2).is('_')
                && self.top_left().is('\\') 
                && self.in_top(2).in_left(2).is('|'){

                elm.extend(vec![
                    arc(&go_top(2).go_left(2).w(),
                        &go_right(2).u(),
                        20),
                    line(&go_top(2).go_left(2).w(),
                         &go_top(2).go_left(2).c())
                ]);

                consumed.extend(vec![
                      right(),
                      top_left(),
                      go_top(2).go_left(2)
                ]);
                matched_arc = true;
                along_arc = true;
            }
        }
        /////////////////////////////////
        //
        //        +---|     +---|
        //        |  /      |  /
        //        _.'       _,'
        //
        ////////////////////////////////
        if self.is('\'') 
            && self.left().any(".,") 
            && self.in_left(2).is('_')
            && self.top_right().is('/')
            && self.in_top(2).in_right(2).is('|'){
            elm.extend(vec![
                    arc(&go_left(2).y(),
                        &go_top(2).go_right(2).w(),
                        20),
                    line(&go_top(2).go_right(2).w(),
                         &go_top(2).go_right(2).c())
                ]);
            consumed.extend(vec![
                left(),
                top_right(),
                go_top(2).go_right(2)
            ]);
            matched_arc = true;
            along_arc = true;
        }
        
        // circle 0
        //   _
        //  (_)
        if  self.is('_')
            && self.left().is('(')
            && self.right().is(')')
            && self.top().is('_'){

            elm.push(open_circle(m, 4));
            consumed.extend(vec![
                top(), left(), right(),
            ]);
            matched_circle = true;
            along_arc = true;// since the circle is too small
            // that the center and the bottom points of
            // the circle share the same element `_`
        }
        // circle 1
        //  .-.
        // ( + )
        //  `-'
        if self.in_left(2).is('(')
            && self.in_right(2).is(')')
            && self.top().is('-')
            && self.bottom().is('-')
            && self.top_left().is('.')
            && self.top_right().is('.')
            && self.bottom_left().any("`'")
            && self.bottom_right().is('\''){

            elm.push(open_circle(m, 8));
            consumed.extend(vec![
                go_left(2),
                go_right(2),
                top(),
                bottom(),
                top_left(),
                top_right(),
                bottom_left(),
                bottom_right(),
            ]);
            matched_circle = true;
            along_arc = false;
        }

        // circle 2
        //     .--.
        //    ( +  )
        //     `--'
        if self.in_left(2).is('(')
            && self.in_right(3).is(')')
            && self.top().is('-')
            && self.top_left().is('.')
            && self.top_right().is('-')
            && self.top().in_right(2).is('.')
            && self.bottom_left().any("`'")
            && self.bottom().is('-')
            && self.bottom_right().is('-')
            && self.bottom().in_right(2).is('\''){
            elm.push(open_circle(o, 10));
            consumed.extend(vec![
                go_left(2),
                go_right(3),
                top(),
                bottom(),
                top_left(),
                top_right(),
                top().go_right(2),
                bottom_left(),
                bottom_right(),
                bottom().go_right(2),
            ]);
            matched_circle = true;
            along_arc = false;
        }
        /////////////////////////////
        //  top left arc of circle3
        //     _.-
        //   .'  |
        //  (----+
        //
        /////////////////////////////
        {   // if 4 of them match then consume all, and make a full circle
            let mut quadrants = vec![];//temp storage for the arcs, replace with circle when all quadrants matched
            let mut top_left_arc_matched = false;
            let mut top_right_arc_matched = false;
            let mut bottom_left_arc_matched = false;
            let mut bottom_right_arc_matched = false;
            if self.in_left(5).is('(')
                && self.in_left(4).top().is('.')
                && self.in_left(3).top().is('\'')
                && self.in_left(2).in_top(2).is('_')
                && self.left().in_top(2).is('.')
                && self.in_top(2).is('-') 
            {
                quadrants.push(arc(&go_top(2).m(),
                            &go_left(4).m(),
                             18));
                top_left_arc_matched = true;
                consumed.extend(vec![
                    go_left(5),
                    go_left(4).top(),
                    go_left(3).top(),
                    go_left(2).go_top(2),
                    left().go_top(2),
                    go_top(2)
                ]);
                matched_arc = true;
                along_arc = false;
            }
            ///////////////////////////////
            // top right arc of the circle3
            //  -._
            //  |  `.
            //  +----)
            //////////////////////////////
            if self.in_right(5).is(')')
                && self.in_right(4).top().is('.')
                && self.in_right(3).top().any("`'")
                && self.in_right(2).in_top(2).is('_')
                && self.right().in_top(2).is('.')
                && self.in_top(2).is('-'){
                quadrants.push(
                    arc(&go_right(4).m(),
                        &go_top(2).m(),
                        18)
                );
                top_right_arc_matched = true;
                consumed.extend(vec![
                    go_right(5),
                    go_right(4).top(),
                    go_right(3).top(),
                    go_right(2).go_top(2),
                    right().go_top(2),
                    go_top(2)
                ]);
                matched_arc = true;
                along_arc = false;
            }
            ////////////////////////////////
            //  bottom_left arc of the circle3 
            //   
            //  (----+
            //   `._ |
            //      `-
            ////////////////////////////////
            if self.in_left(5).is('(')
                && self.in_left(4).bottom().any("`'")
                && self.in_left(3).bottom().is('.')
                && self.in_left(2).bottom().is('_')
                && self.left().in_bottom(2).any("`'")
                && self.in_bottom(2).is('-'){
                quadrants.push(
                    arc(&go_left(4).m(),
                        &go_bottom(2).m(),
                        18)
                );
                bottom_left_arc_matched = true;
                consumed.extend(vec![
                   go_left(5),
                   go_left(4).bottom(),
                   go_left(3).bottom(),
                   go_left(2).bottom(),
                   left().go_bottom(2),
                   go_bottom(2)
                ]);
                matched_arc = true;
                along_arc = false;
            }
            ///////////////////////////////////
            //  bottom_right arc of the circle3
            //    +----)
            //    | _,'
            //    -'
            //
            ////////////////////////////////////
            if self.in_right(5).is(')')
                && self.in_right(4).bottom().is('\'')
                && self.in_right(3).bottom().is(',')
                && self.in_right(2).bottom().is('_')
                && self.right().in_bottom(2).is('\'')
                && self.in_bottom(2).is('-'){
                quadrants.push(
                    arc(&go_bottom(2).m(),
                        &go_right(4).m(),
                        18)
                );
                bottom_right_arc_matched = true;
                consumed.extend(vec![
                    go_right(5),
                    go_right(4).bottom(),
                    go_right(3).bottom(),
                    go_right(2).bottom(),
                    right().go_bottom(2),
                    go_bottom(2),
                ]);
                matched_arc = true;
                along_arc = false;
            }
            if top_left_arc_matched
                && top_right_arc_matched
                && bottom_left_arc_matched
                && bottom_right_arc_matched{
                elm.push(open_circle(m, 18));
                matched_circle = true;
                along_arc = false;
            }
            else{
                elm.extend(quadrants);
            }
        }
        // if true
        //    .     .
        //   (  and  ) will be drawn from the top to bottom with 1 arc
        //    `     '
        // else the default behavior is used which is drawing with combination 
        // of rounded corner and lines connecting the arc at `(` or `)`
        if !matched_arc
            && !matched_circle{
            if self.is('('){
                if enable_round_pill{
                    //   .
                    //  (
                    //   `
                    if self.top_right().any(".,")
                        && self.bottom_right().any("`'"){
                        elm.extend(
                            vec![
                                arc(&top_right().o(), &bottom_right().o(), 10),
                            ]);
                        consumed.extend(vec![
                            top_right(), 
                            bottom_right()
                        ]);
                        along_arc = true;
                    }
                }
            }
            if self.is(')'){
                if enable_round_pill{
                    //   .
                    //    )
                    //   '
                    if self.top_left().any(".,")
                        && self.bottom_left().any("`'"){
                        elm.extend(
                            vec![
                                arc(&bottom_left().k(), &top_left().k(), 10),
                            ]);
                        consumed.extend(vec![
                            top_left(), 
                            bottom_left()
                        ]);
                        along_arc = true;
                    }
                }
            }
        }
        (elm, consumed, along_arc)
    }
}

