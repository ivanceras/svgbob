use location::{Location};

use block::Block;
use block::Block::{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y};
use point_block::PointBlock;

use fragments::Fragment;
use fragments::{arc, arrow_line, line, circle_start_line, square_start_line, circle_open_line, big_circle_open_line, start_arrow_line, dashed_line};

use self::Signal::{Medium, Strong, Weak};
use box_drawing;
use location::Direction::{Bottom, BottomLeft, BottomRight, Left, Right, Top, TopLeft, TopRight};

use self::Can::{ConnectTo, Is, IsStrongAll};

/// the strength of signal
/// whether or not connects to the direction
///  | has a strong signal to connect top and bottom
///    and has a weak signal to connect left and right
///  + has a medium signal to connect top, bottom, left, right
///    and has a weak signal to connect top_left, top_right, botom_left, bottom_right
///
///   Strong + Strong connects
///   Medium + Medium is situational, but mostly connects
///   Weak + Weak is situational, but mostly doesn't connects
///   Strong + Medium connects
///   Strong + Weak connects
///   Silent signals, are there but are not in used
///   They are just potential and are reluctant

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum Signal {
    Weak,
    Medium,
    /// Strong is drawn as drawing element
    /// regardless if it can connect or not
    Strong,
}


#[derive(Debug)]
pub struct Characteristic {
    /// these are the default behavior of the fragment
    /// describe the signal strength: Signal of connection from certain blocks: Block
    /// if connecting from this block: Block the line is descibed by the fragments: Vec<Fragments>
    pub properties: Vec<(Block, Signal, Vec<Fragment>)>,
    /// if condition is met, the block becomes a strong signal
    pub intensify: Vec<(Block, Condition)>,
    /// after checking intensifier, if the connection signal to these blocks
    /// are strong
    /// then use these fragments instead
    pub intended_behavior: Vec<(Vec<Block>, Vec<Fragment>)>,
    /// typing characters are dynamic
    /// while box uncide drawing are static
    pub is_static: bool,
}

impl Characteristic {
    pub fn is_strong_block(&self, arg_block: &Block) -> bool {
        self.properties
            .iter()
            .any(|&(ref block, ref signal, _)| block == arg_block && *signal == Strong)
    }

    // get default signal of this block
    // without applying intensifier
    pub fn get_block_signal(&self, arg_block: &Block) -> Option<Signal> {
        for &(ref block, ref signal, _) in &self.properties {
            if block == arg_block {
                return Some(signal.clone());
            }
        }
        None
    }
}

pub trait Properties {
    fn get_characteristic(&self) -> Option<Characteristic>;

    fn is(&self, ch: char) -> bool;

    fn is_static(&self) -> bool;

    fn any(&self, s: &str) -> bool;

    fn in_any(&self, ch: Vec<char>) -> bool;

    fn can_connect(&self, signal: &Signal, block: &Block) -> bool;
}

#[derive(Debug)]
pub enum Can {
    /// test if this character connection to specified block
    /// pass the minimum signal
    ConnectTo(Block, Signal),
    /// test if the character is the same as the specified
    Is(char),
    /// test if all of the specified blocks are strong in its properties
    IsStrongAll(Vec<Block>),
}

/// Behavioral condition
#[derive(Debug)]
pub struct Condition {
    pub loc: Location,
    pub can: Can,
}

impl Properties for char {
    fn is(&self, ch: char) -> bool {
        *self == ch
    }
    fn is_static(&self) -> bool {
        if let Some(characteristic) = self.get_characteristic(){
            characteristic.is_static
        }
        else{
            false
        }
    }

    fn any(&self, s: &str) -> bool {
        s.contains(*self)
    }

    fn in_any(&self, ch: Vec<char>) -> bool {
        ch.contains(self)
    }

    /// get the characteristic of a character
    /// it's behavior and the intended behavior
    ///
    ///    ┌─┬─┬─┬─┬─┐
    ///    │a│b│c│d│e│
    ///    ├─┼─┼─┼─┼─┤
    ///    │f│g│h│i│j│
    ///    ├─┼─┼─┼─┼─┤
    ///    │k│l│m│n│o│
    ///    ├─┼─┼─┼─┼─┤
    ///    │p│q│r│s│t│
    ///    ├─┼─┼─┼─┼─┤
    ///    │u│v│w│x│y│
    ///    └─┴─┴─┴─┴─┘
    ///
    fn get_characteristic(&self) -> Option<Characteristic> {
        let a = &PointBlock::block(A);
        let _b = &PointBlock::block(B);
        let c = &PointBlock::block(C);
        let _d = &PointBlock::block(D);
        let e = &PointBlock::block(E);
        let f = &PointBlock::block(F);
        let g = &PointBlock::block(G);
        let h = &PointBlock::block(H);
        let i = &PointBlock::block(I);
        let j = &PointBlock::block(J);
        let k = &PointBlock::block(K);
        let _l = &PointBlock::block(L);
        let m = &PointBlock::block(M);
        let _n = &PointBlock::block(N);
        let o = &PointBlock::block(O);
        let _p = &PointBlock::block(P);
        let q = &PointBlock::block(Q);
        let r = &PointBlock::block(R);
        let s = &PointBlock::block(S);
        let _t = &PointBlock::block(T);
        let u = &PointBlock::block(U);
        let _v = &PointBlock::block(V);
        let w = &PointBlock::block(W);
        let _x = &PointBlock::block(X);
        let y = &PointBlock::block(Y);

        let top = || Location::go(Top);
        let bottom = || Location::go(Bottom);
        let left = || Location::go(Left);
        let right = || Location::go(Right);
        let top_left = || Location::go(TopLeft);
        let top_right = || Location::go(TopRight);
        let bottom_left = || Location::go(BottomLeft);
        let bottom_right = || Location::go(BottomRight);

        /////////////////////////////////
        //
        // | vertical line, or, pipe
        //
        /////////////////////////////////
        if self.is('|') {
            Some(Characteristic {
                is_static: false,
                properties: vec![(C, Strong, vec![line(c, w)]), (W, Strong, vec![line(c, w)])],
                intensify: vec![
                    /*
                    //    |
                    //     \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: IsStrongAll(vec![A, Y]),
                        },
                    ),
                    //      |
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: IsStrongAll(vec![U, E]),
                        },
                    ),
                    //     /
                    //    |
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: IsStrongAll(vec![U, E]),
                        },
                    ),
                    //     \
                    //      |
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: IsStrongAll(vec![A, Y]),
                        },
                    ),
                    */
                    //  |-
                    ( O,
                      Condition{
                          loc: right(),
                          can: IsStrongAll(vec![K,O])
                      }
                    ),
                    //  -|
                    ( K,
                      Condition{
                          loc: left(),
                          can: IsStrongAll(vec![K,O])
                        }
                    ),
                ],
                intended_behavior: vec![
                    /*
                    //    |
                    //     \
                    (vec![Y], vec![line(c, m), line(m, y)]),
                    //      |
                    //     /
                    (vec![U], vec![line(c, m), line(m, u)]),
                    //    /
                    //   |
                    (vec![E], vec![line(w, m), line(m, e)]),
                    //     \
                    //      |
                    (vec![A], vec![line(w, m), line(m, a)]),
                    */
                    //    |-
                    (vec![O], vec![line(o, m), line(c,w)]),
                    //   -|
                    (vec![K], vec![line(m, k), line(c,w)]),
                ],
            })
        }
        //////////////////////////////
        //
        // - dash, horizontal line, minus sign
        //
        /////////////////////////////
        else if self.is('-') {
            Some(Characteristic {
                is_static: false,
                properties: vec![(K, Strong, vec![line(k, o)]), (O, Strong, vec![line(k, o)])],
                intensify: vec![],
                intended_behavior: vec![],
            })
        }
        /////////////////////////////
        //
        //  ~ tilde will be hidden lines
        //
        //////////////////////////////
        else if self.is('~'){
            Some(Characteristic {
                is_static: false,
                properties: vec![(K, Strong, vec![dashed_line(k, o)]), (O, Strong, vec![dashed_line(k, o)])],
                intensify: vec![],
                intended_behavior: vec![],
            })
        }
        ///////////////////////////////
        //
        // = equal sign
        //
        ///////////////////////////////
        else if self.is('=') {
            Some(Characteristic {
                is_static: false,
                properties: vec![
                    (K, Strong, vec![line(&k.adjust(0.0, 1.0), &o.adjust(0.0, 1.0))]),
                    (O, Strong, vec![line(&k.adjust(0.0, 1.0), &o.adjust(0.0, 1.0))]),
                    (F, Strong, vec![line(&f.adjust(0.0, 1.0), &j.adjust(0.0, 1.0))]),
                    (J, Strong, vec![line(&f.adjust(0.0, 1.0), &j.adjust(0.0, 1.0))]),
                ],
                intensify: vec![],
                intended_behavior: vec![],
            })
        }
        /////////////////////////////////
        //
        // _ underscore
        //
        ////////////////////////////////
        else if self.is('_') {
            Some(Characteristic {
                is_static: false,
                properties: vec![(U, Strong, vec![line(u, y)]), (Y, Strong, vec![line(u, y)])],
                intensify: vec![
                ],
                intended_behavior: vec![
                ],
            })
        }
        /////////////////////////////
        //
        // / forward slash
        //
        ////////////////////////////
        else if self.is('/') {
            Some(Characteristic {
                is_static: false,
                properties: vec![(E, Strong, vec![line(u, e)]), (U, Strong, vec![line(u, e)])],
                intensify: vec![
                    //   |
                    //   /
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: ConnectTo(W, Strong),
                        },
                    ),
                    //   /
                    //   |
                    (
                        W,
                        Condition {
                            loc: bottom(),
                            can: ConnectTo(C, Strong),
                        },
                    ),
                    //    /-
                    (
                        O,
                        Condition{
                            loc: right(),
                            can: IsStrongAll(vec![K, O]),
                        },
                    ),
                    //   -/
                    (
                        K,
                        Condition{
                            loc: left(),
                            can: IsStrongAll(vec![K, O]),
                        },
                    ),
                    //   /_
                    (
                        Y,
                        Condition{
                            loc: right(),
                            can: IsStrongAll(vec![U,Y])
                        }
                    ),
                ],
                intended_behavior: vec![
                    //   |
                    //   /
                    (vec![C], vec![line(u, m), line(m, c)]),
                    //  /
                    //  |
                    (vec![W], vec![line(m, w), line(m, e)]),
                    //   /-
                    (vec![O], vec![line(o, m), line(u, e)]),
                    //  -/
                    (vec![K], vec![line(k, m), line(u, e)]),
                    //   /_
                    (vec![Y], vec![line(u, y), line(u, e)]),
                ],
            })
        }
        ////////////////////////////////
        //
        // \ backslash
        //
        ////////////////////////////////
        else if self.is('\\') {
            Some(Characteristic {
                is_static: false,
                properties: vec![(A, Strong, vec![line(a, y)]), (Y, Strong, vec![line(a, y)])],
                intensify: vec![
                    //    \
                    //    |
                    (
                        W,
                        Condition {
                            loc: bottom(),
                            can: ConnectTo(C, Strong),
                        },
                    ),
                    //    |
                    //    \
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: ConnectTo(W, Strong),
                        },
                    ),
                    //  -\
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: IsStrongAll(vec![K,O])
                        }
                    ),
                    //   \-
                    (
                        O,
                        Condition{
                            loc: right(),
                            can: IsStrongAll(vec![K,O])
                        }
                    ),
                    //  _\
                    (
                        U,
                        Condition{
                            loc: left(),
                            can: IsStrongAll(vec![U,Y])
                        }
                    ),
                ],
                intended_behavior: vec![
                    //    \
                    //    |
                    (vec![W], vec![line(a, m), line(m, w)]),
                    //   |
                    //   \
                    (vec![C], vec![line(y, m), line(m, c)]),
                    //  -\ 
                    (vec![K], vec![line(a, y), line(k, m)]),
                    //   \-
                    (vec![O], vec![line(a, y), line(m, o)]),
                    //   _\
                    (vec![U], vec![line(u, y), line(a, y)]),
                ],
            })
        }
        /////////////////////////////////
        //
        // + plus sign, cross
        //
        /////////////////////////////////
        else if self.is('+') {
            Some(Characteristic {
                is_static: false,
                properties: vec![
                    // emits medium signal if Block C is strong, if connecting from C, then the line will be M to C
                    (C, Medium, vec![line(m, c)]),
                    (K, Medium, vec![line(m, k)]),
                    (O, Medium, vec![line(m, o)]),
                    (W, Medium, vec![line(m, w)]),
                    // emits a weak signal if Block A is strong, if connecting from A line will be
                    // M to A
                    (A, Weak, vec![line(m, a)]),
                    (E, Weak, vec![line(m, e)]),
                    (U, Weak, vec![line(m, u)]),
                    (Y, Weak, vec![line(m, y)]),
                ],
                intensify: vec![
                    //   |     .
                    //   +     +
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: ConnectTo(W, Medium),
                        },
                    ),
                    //   +     +
                    //   |     '
                    (
                        W,
                        Condition {
                            loc: bottom(),
                            can: ConnectTo(C, Medium),
                        },
                    ),
                    //   -+    '+
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Weak),
                        },
                    ),
                    //    +-   +'
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Weak),
                        },
                    ),
                    //    \   	╲	╳   but not  _
                    //     +     +   +            +
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: IsStrongAll(vec![A, Y]),
                        },
                    ),
                    //     /  	╱	╳	 but not    _
                    //    +    +   +                   +
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: IsStrongAll(vec![E, U]),
                        },
                    ),
                    //      +
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //     +
                    //      \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                ],
                intended_behavior: vec![],
            })
        }
        ////////////////////////////
        //
        // letter x X
        //
        ////////////////////////////
        else if self.any("xX") {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    //    \
                    //     x
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: ConnectTo(Y, Strong),
                        },
                    ),
                    //      /
                    //     x
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: ConnectTo(U, Strong),
                        },
                    ),
                    //     x
                    //    /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //      x
                    //       \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                    //    .
                    //     X
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: ConnectTo(Y, Weak),
                        },
                    ),
                    //      .
                    //     X
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: ConnectTo(U, Weak),
                        },
                    ),
                    //      X
                    //       `
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Weak),
                        },
                    ),
                    //      X
                    //     '
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Weak),
                        },
                    ),
                ],
                intended_behavior: vec![],
                properties: vec![
                    (A, Medium, vec![line(m, a)]),
                    (E, Medium, vec![line(m, e)]),
                    (U, Medium, vec![line(m, u)]),
                    (Y, Medium, vec![line(m, y)]),
                ],
            })
        }
        ///////////////////////////
        //
        // ., dot or period and comma
        //
        ///////////////////////////
        else if self.any(".,") {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    //  -.  +.
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Medium),
                        },
                    ),
                    //  .-  .+
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Medium),
                        },
                    ),
                    //  _.
                    (
                        U,
                        Condition {
                            loc: left(),
                            can: ConnectTo(Y, Strong),
                        },
                    ),
                    //  ._
                    (
                        Y,
                        Condition {
                            loc: right(),
                            can: ConnectTo(U, Strong),
                        },
                    ),
                    //      .
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //      /    only for / else   _
                    //     .                        .   will connect
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: IsStrongAll(vec![E, U]),
                        },
                    ),
                    //      .
                    //       \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                    //      \     only \ or else this connects as well  _
                    //       .                                           .
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: IsStrongAll(vec![A, Y]),
                        },
                    ),
                    //   .    .
                    //   |    '
                    (
                        W,
                        Condition {
                            loc: bottom(),
                            can: ConnectTo(C, Medium),
                        },
                    ),
                    //   |    only |
                    //   .
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: Is('|'),
                        },
                    ),
                    //   .    only X
                    //    X
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: Is('X'),
                        },
                    ),
                    //     .  only X
                    //    X
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: Is('X'),
                        },
                    ),
                ],
                intended_behavior: vec![
                    //     .-
                    //    /
                    (vec![O, U], vec![arc(o, q, 4), line(q, u)]),
                    //     .-
                    //      \
                    (vec![O, Y], vec![arc(o, s, 4), line(s, y)]),
                    //     -.
                    //       \
                    (vec![K, Y], vec![arc(s, k, 4), line(s, y)]),
                    //     -.
                    //     /
                    (vec![K, U], vec![line(u, q), arc(q, k, 2)]),
                    //       /
                    //      .
                    //     /
                    (vec![U, E], vec![line(u, e)]),
                    //     \
                    //      .
                    //       \
                    (vec![A, Y], vec![line(a, y)]),
                    //    \
                    //     .
                    //     |
                    (vec![A, W], vec![line(a, g), arc(r, g, 8), line(r, w)]),
                    //    \
                    //     .
                    //    /
                    (vec![A, U], vec![line(a, g), arc(q, g, 8), line(q, u)]),
                    //      /
                    //     .
                    //      \
                    (vec![E, Y], vec![line(e, i), arc(i, s, 8), line(s, y)]),
                    //     |
                    //     .
                    //     |
                    (vec![C, W], vec![line(c, w)]),
                    //       /
                    //      .
                    //      |
                    (vec![E, W], vec![line(e, i), arc(i, r, 8), line(r, w)]),
                    //     |
                    //     .
                    //    /
                    (vec![C, U], vec![line(u, q), arc(q, h, 8), line(h, c)]),
                    //     |
                    //     .
                    //      \
                    (vec![C, Y], vec![line(c, h), arc(h, s, 8), line(s, y)]),
                    //      .
                    //     / \
                    (vec![U, Y], vec![line(m, u), line(m, y)]),
                ],
                properties: vec![
                    (O, Weak, vec![arc(o, r, 2)]),
                    (K, Weak, vec![arc(r, k, 2)]),
                    (W, Medium, vec![line(r, w)]),
                    (U, Weak, vec![line(q, u)]),
                    (Y, Weak, vec![line(s, y)]),
                    (A, Weak, vec![line(m, a)]),
                    (E, Weak, vec![line(m, e)]),
                    (F, Weak, vec![line(m, f)]),
                    (J, Weak, vec![line(m, j)]),
                ],
            })
        }
        ///////////////////////////
        //
        // `' single quote and backquote
        //
        //////////////////////////
        else if self.any("`'") {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    //  -'   +'
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Medium),
                        },
                    ),
                    //  '-    '+
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Medium),
                        },
                    ),
                    //       /
                    //      '
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //      \
                    //       '
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: ConnectTo(Y, Strong),
                        },
                    ),
                    //        /
                    //       '
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: ConnectTo(U, Strong),
                        },
                    ),
                    //   |    .
                    //   '    '
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: ConnectTo(W, Medium),
                        },
                    ),
                    //        X
                    //       '
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: Is('X'),
                        },
                    ),
                    //      X
                    //       '
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: Is('X'),
                        },
                    ),
                ],
                intended_behavior: vec![
                    //    \
                    //     '-
                    (vec![A, O], vec![line(a, g), arc(g, o, 4)]),
                    //      /
                    //     '-
                    (vec![E, O], vec![line(e, i), arc(i, o, 2)]),
                    //       /
                    //     -'
                    (vec![K, E], vec![arc(k, i, 4), line(i, e)]),
                    //     \
                    //     -'
                    (vec![K, A], vec![arc(k, g, 2), line(g, a)]),
                    //     \ /
                    //      '
                    (vec![A, E], vec![line(a, m), line(m, e)]),
                ],
                properties: vec![
                    (C, Medium, vec![line(h, c)]),
                    (O, Weak, vec![arc(h, o, 2)]),
                    (K, Weak, vec![arc(k, h, 2)]),
                    (A, Weak, vec![line(a, g)]),
                    (E, Weak, vec![line(e, i)]),
                    (F, Weak, vec![line(c, f)]),
                    (J, Weak, vec![line(c, j)]),
                ],
            })
        }
        //////////////////////
        //
        // * asterisk or star
        //
        //////////////////////
        else if self.is('*') {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    //    |
                    //    *
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: ConnectTo(W, Strong),
                        },
                    ),
                    //    *
                    //    |
                    (
                        W,
                        Condition {
                            loc: bottom(),
                            can: ConnectTo(C, Strong),
                        },
                    ),
                    //   -*
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Weak),
                        },
                    ),
                    //    *-
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Weak),
                        },
                    ),
                    //    \    but not  _
                    //     *             *
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: IsStrongAll(vec![A, Y]),
                        },
                    ),
                    //     /   but not    _
                    //    *              *
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: IsStrongAll(vec![E, U]),
                        },
                    ),
                    //      *
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //     *
                    //      \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                ],
                intended_behavior: vec![],
                properties: vec![
                    (C, Medium, vec![circle_start_line(m, c)]),
                    (W, Medium, vec![circle_start_line(m, w)]),
                    (K, Medium, vec![circle_start_line(m, k)]),
                    (O, Medium, vec![circle_start_line(m, o)]),
                    (A, Medium, vec![circle_start_line(m, a)]),
                    (E, Medium, vec![circle_start_line(m, e)]),
                    (U, Medium, vec![circle_start_line(m, u)]),
                    (Y, Medium, vec![circle_start_line(m, y)]),
                ],
            })
        }
        ///////////////////////////
        //
        // small letter o
        //
        //////////////////////////
        else if self.is('o') {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    //    |
                    //    o
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: ConnectTo(W, Strong),
                        },
                    ),
                    //    o
                    //    |
                    (
                        W,
                        Condition {
                            loc: bottom(),
                            can: ConnectTo(C, Strong),
                        },
                    ),
                    //   -o
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Weak),
                        },
                    ),
                    //    o-
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Weak),
                        },
                    ),
                    //    \    but not  _
                    //     o             o
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: IsStrongAll(vec![A, Y]),
                        },
                    ),
                    //     /   but not    _
                    //    o              o
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: IsStrongAll(vec![E, U]),
                        },
                    ),
                    //      o
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //     o
                    //      \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                ],
                intended_behavior: vec![],
                properties: vec![
                    (C, Medium, vec![circle_open_line(m, c)]),
                    (W, Medium, vec![circle_open_line(m, w)]),
                    (K, Medium, vec![circle_open_line(m, k)]),
                    (O, Medium, vec![circle_open_line(m, o)]),
                    (A, Medium, vec![circle_open_line(m, a)]),
                    (E, Medium, vec![circle_open_line(m, e)]),
                    (U, Medium, vec![circle_open_line(m, u)]),
                    (Y, Medium, vec![circle_open_line(m, y)]),
                ],
            })
        }
        /////////////////////////////
        //
        // big letter O
        //
        ////////////////////////////
        else if self.is('O') {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    //    |
                    //    O
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: ConnectTo(W, Strong),
                        },
                    ),
                    //    O
                    //    |
                    (
                        W,
                        Condition {
                            loc: bottom(),
                            can: ConnectTo(C, Strong),
                        },
                    ),
                    //   -O
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Weak),
                        },
                    ),
                    //    O-
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Weak),
                        },
                    ),
                    //    \    but not  _
                    //     O             O
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: IsStrongAll(vec![A, Y]),
                        },
                    ),
                    //     /   but not    _
                    //    O              O
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: IsStrongAll(vec![E, U]),
                        },
                    ),
                    //      O
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //     O
                    //      \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                ],
                intended_behavior: vec![],
                properties: vec![
                    (C, Medium, vec![big_circle_open_line(m, c)]),
                    (W, Medium, vec![big_circle_open_line(m, w)]),
                    (K, Medium, vec![big_circle_open_line(m, k)]),
                    (O, Medium, vec![big_circle_open_line(m, o)]),
                    (A, Medium, vec![big_circle_open_line(m, a)]),
                    (E, Medium, vec![big_circle_open_line(m, e)]),
                    (U, Medium, vec![big_circle_open_line(m, u)]),
                    (Y, Medium, vec![big_circle_open_line(m, y)]),
                ],
            })
        }
        /////////////////////////////
        //
        // # pound sign, sharp, hashtag
        //
        ////////////////////////////
        else if self.is('#') {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    //    |
                    //    #
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: ConnectTo(W, Strong),
                        },
                    ),
                    //    #
                    //    |
                    (
                        W,
                        Condition {
                            loc: bottom(),
                            can: ConnectTo(C, Strong),
                        },
                    ),
                    //   -#
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Weak),
                        },
                    ),
                    //    #-
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Weak),
                        },
                    ),
                    //    \    but not  _
                    //     #             #
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: IsStrongAll(vec![A, Y]),
                        },
                    ),
                    //     /   but not    _
                    //    #              #
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: IsStrongAll(vec![E, U]),
                        },
                    ),
                    //      #
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //     #
                    //      \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                ],
                intended_behavior: vec![],
                properties: vec![
                    (C, Medium, vec![square_start_line(m, c)]),
                    (W, Medium, vec![square_start_line(m, w)]),
                    (K, Medium, vec![square_start_line(m, k)]),
                    (O, Medium, vec![square_start_line(m, o)]),
                    (A, Medium, vec![square_start_line(m, a)]),
                    (E, Medium, vec![square_start_line(m, e)]),
                    (U, Medium, vec![square_start_line(m, u)]),
                    (Y, Medium, vec![square_start_line(m, y)]),
                ],
            })
        }
        ////////////////////////////////
        //
        // < less than sign, arrow left
        //
        ///////////////////////////////
        else if self.any("<◀◄") {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    // <-   <+   <'  <.
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Weak),
                        },
                    ),
                    //  /
                    // <
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: ConnectTo(U, Strong),
                        },
                    ),
                    //  <
                    //   \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                    //   -<
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Strong),
                        }
                    )
                ],
                intended_behavior: vec![],
                properties: vec![
                    (O, Medium, vec![arrow_line(o, m)]),
                    (K, Weak, vec![start_arrow_line(o, m)]),
                    (E, Weak, vec![line(m, e)]),
                    (Y, Weak, vec![line(m, y)]),
                ],
            })
        }
        ////////////////////////////
        //
        // > greater than sign, arrow right
        //
        ////////////////////////////
        else if self.any(">▶►") {
            Some(Characteristic {
                is_static: false,
                intensify: vec![
                    //  ->  +>  .>  '>
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Weak),
                        },
                    ),
                    //   \
                    //    >
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: ConnectTo(Y, Strong),
                        },
                    ),
                    //   >
                    //  /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //  >-
                    ( O,
                      Condition {
                          loc: right(),
                          can: ConnectTo(K, Strong),
                        }
                    ),
                ],
                intended_behavior: vec![
                ],
                properties: vec![
                    (K, Medium, vec![arrow_line(k, m)]),
                    (O, Weak, vec![start_arrow_line(k, m)]),
                    (A, Weak, vec![line(m, a)]),
                    (U, Weak, vec![line(m, u)]),
                ],
            })
        }
        ////////////////////////
        //
        // ^ carret, arrow up
        //
        ///////////////////////
        else if self.any("^▲▴") {
            Some(Characteristic {
                is_static: false,
                properties: vec![
                    (W, Medium, vec![arrow_line(w, h)]),
                    (U, Medium, vec![arrow_line(u, i)]),
                    (Y, Medium, vec![arrow_line(y, g)]),
                    (K, Weak, vec![arrow_line(k, i)]),
                    (O, Weak, vec![arrow_line(o, g)]),
                ],
                intensify: vec![
                    //    ^
                    //    |
                    (
                        W,
                        Condition {
                            loc: bottom(),
                            can: ConnectTo(C, Strong),
                        },
                    ),
                    //      ^
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                    //     _^
                    //
                    (
                        U,
                        Condition {
                            loc: left(),
                            can: ConnectTo(Y, Strong),
                        },
                    ),
                    //      ^_
                    //
                    (
                        Y,
                        Condition {
                            loc: right(),
                            can: ConnectTo(U, Strong),
                        },
                    ),
                    //     -^
                    //
                    (
                        K,
                        Condition {
                            loc: left(),
                            can: ConnectTo(O, Strong),
                        },
                    ),
                    //      ^-
                    //
                    (
                        O,
                        Condition {
                            loc: right(),
                            can: ConnectTo(K, Strong),
                        },
                    ),
                    //     ^
                    //      \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                ],
                intended_behavior: vec![
                    //      ^
                    //     / \
                    (vec![U, Y], vec![line(u, m), line(m, y)]),
                ],
            })
        }
        //////////////////////////
        //
        // letter v V
        //
        //////////////////////////
        else if self.any("vV▼▾") {
            Some(Characteristic {
                is_static: false,
                properties: vec![
                    (C, Medium, vec![arrow_line(c, r)]),
                    (A, Medium, vec![arrow_line(a, s)]),
                    (E, Medium, vec![arrow_line(e, q)]),
                ],
                intensify: vec![
                    //    |    .
                    //    v    v
                    (
                        C,
                        Condition {
                            loc: top(),
                            can: ConnectTo(W, Medium),
                        },
                    ),
                    //    \
                    //     v
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: ConnectTo(Y, Strong),
                        },
                    ),
                    //     /
                    //    v
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: ConnectTo(U, Strong),
                        },
                    ),
                ],
                intended_behavior: vec![
                    //    \ /
                    //     v
                    (vec![A, E], vec![line(a, m), line(m, e)]),
                ],
            })
        }
        //  ◤
        else if self.is('◤'){
            Some(Characteristic {
                is_static: false,
                properties: vec![
                    (Y, Medium, vec![arrow_line(y, g)]),
                ],
                intensify: vec![
                    //     ◤
                    //      \
                    (
                        Y,
                        Condition {
                            loc: bottom_right(),
                            can: ConnectTo(A, Strong),
                        },
                    ),
                ],
                intended_behavior: vec![
                    //      ◤
                    //       \
                    (vec![Y], vec![arrow_line(y, a)]),
                ],
            })
        }
        // ◥ 
        else if self.is('◥') {
            Some(Characteristic {
                is_static: false,
                properties: vec![
                    (U, Medium, vec![arrow_line(u, i)]),
                ],
                intensify: vec![
                    //      ◥ 
                    //     /
                    (
                        U,
                        Condition {
                            loc: bottom_left(),
                            can: ConnectTo(E, Strong),
                        },
                    ),
                ],
                intended_behavior: vec![
                    //      ◥ 
                    //     / 
                    (vec![U], vec![arrow_line(u,e)]),
                ],
            })
        }
        // Specific arrow heads
        //  ◢
        else if self.is('◢'){
            Some(Characteristic {
                is_static: false,
                properties: vec![
                    (A, Medium, vec![arrow_line(a, s)]),
                ],
                intended_behavior: vec![
                    //    \
                    //     ◢
                    (vec![A], vec![arrow_line(a,s)]),
                ],
                intensify: vec![

                    //    \
                    //     ◢
                    (
                        A,
                        Condition {
                            loc: top_left(),
                            can: ConnectTo(Y, Strong),
                        },
                    ),
                ]

            })
        }
        //   /
        //  ◣
        else if self.is('◣'){
            Some(Characteristic {
                is_static: false,
                properties: vec![
                    (E, Medium, vec![arrow_line(e, q)]),
                ],
                intensify: vec![

                    //     /
                    //    ◣
                    (
                        E,
                        Condition {
                            loc: top_right(),
                            can: ConnectTo(U, Strong),
                        },
                    ),
                ],

                intended_behavior: vec![
                    //      /
                    //     ◣
                    (vec![E], vec![line(m, e)]),
                ],
            })

        }
        ///////////////////////////
        //
        // ( open parenthesis
        //
        //////////////////////////
        else if self.is('(') {
            Some(Characteristic {
                is_static: false,
                intensify: vec![],
                intended_behavior: vec![],
                properties: vec![
                    (E, Strong, vec![arc(e, y, 8)]),
                    (Y, Strong, vec![arc(e, y, 8)]),
                    (K, Weak, vec![arc(c, w, 4)]),
                    (O, Weak, vec![arc(c, w, 4)]),
                    (C, Weak, vec![arc(c, w, 4)]),
                    (W, Weak, vec![arc(c, w, 4)]),
                ],
            })
        }
        ///////////////////////////////
        //
        // close parenthesis
        //
        ///////////////////////////////
        else if self.is(')') {
            Some(Characteristic {
                is_static: false,
                intensify: vec![],
                intended_behavior: vec![],
                properties: vec![
                    (A, Strong, vec![arc(u, a, 8)]),
                    (U, Strong, vec![arc(u, a, 8)]),
                    (K, Weak, vec![arc(w, c, 4)]),
                    (O, Weak, vec![arc(w, c, 4)]),
                    (C, Weak, vec![arc(w, c, 4)]),
                    (W, Weak, vec![arc(w, c, 4)]),
                ],
            })
        }
        ///////////////////////////////
        //
        // [ open square bracket
        //
        ///////////////////////////////
        else if self.is('[') {
            Some(Characteristic {
                is_static: false,
                intensify: vec![],
                intended_behavior: vec![],
                properties: vec![
                    (E, Strong, vec![line(e, c), line(c, w), line(w, y)]),
                    (Y, Strong, vec![line(e, c), line(c, w), line(w, y)]),
                ],
            })
        }
        /////////////////////////////
        //
        // ] close square bracket
        //
        ////////////////////////////
        else if self.is(']') {
            Some(Characteristic {
                is_static: false,
                intensify: vec![],
                intended_behavior: vec![],
                properties: vec![
                    (A, Strong, vec![line(a, c), line(c, w), line(w, u)]),
                    (U, Strong, vec![line(a, c), line(c, w), line(w, u)]),
                ],
            })
        }
        //////////////////////////////
        //
        // : colon for vertical hidden line
        //
        //////////////////////////////
        else if self.is(':'){
            Some(Characteristic{
                is_static: false,
                intensify: vec![],
                intended_behavior: vec![],
                properties: vec![
                    (C, Strong, vec![dashed_line(c,w)]),
                    (W, Strong, vec![dashed_line(c,w)]),
                ],
            })
        }
        //////////////////////////////
        //
        // ! exclamation can also be for vertical hidden line
        //
        //////////////////////////////
        else if self.is('!'){
            Some(Characteristic{
                is_static: false,
                intensify: vec![],
                intended_behavior: vec![],
                properties: vec![
                    (C, Strong, vec![dashed_line(c,w)]),
                    (W, Strong, vec![dashed_line(c,w)]),
                ],
            })
        }
        // if nothing matches, try checking in box drawing
        else {
            let (blocks, fragments) = box_drawing::box_drawing(&self);
            let mut properties = vec![];
            for b in blocks {
                properties.push((b, Strong, fragments.clone()));
            }
            if !properties.is_empty() {
                Some(Characteristic {
                    is_static: true, // all of box drawing are static
                    intensify: vec![],
                    intended_behavior: vec![],
                    properties: properties,
                })
            } else {
                None
            }
        }
    }

    fn can_connect(&self, arg_signal: &Signal, block: &Block) -> bool {
        if let Some(character) = self.get_characteristic() {
            if let Some(signal) = character.get_block_signal(block) {
                signal == *arg_signal
            } else {
                false
            }
        } else {
            false
        }
    }
}
