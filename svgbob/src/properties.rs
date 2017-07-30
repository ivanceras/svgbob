use fragments::Direction;

use fragments::Block;
use fragments::Block::{
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y
};

use fragments::Fragment;
use fragments::{
    line,
    arrow_line,
    start_arrow_line,
    arc,
    open_circle,
    solid_circle,
};

use Element;


use fragments::Direction::{
    Top,Bottom,
    Left,Right,
    TopLeft,TopRight,
    BottomLeft,BottomRight
};
use self::Behavior::{Static,Dynamic};
use self::Signal::{Silent,Weak,Medium,Strong};
use box_drawing;

use self::Can::{
    ConnectTo,
    Is,
    Any
};


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
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Signal{
    Silent,
    Weak,
    Medium,
    /// Strong is drawn as drawing element
    /// regardless if it can connect or not
    Strong,
}

/// a location in the grid
/// relative to the focused char
/// go to direction and how many steps to get there
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Location(pub Vec<(Direction,usize)>);

impl Location{
    pub fn go(direction: Direction) -> Self{
        Self::jump(direction, 1)
    }

    fn jump(direction: Direction, step: usize) -> Self {
        Location(vec![(direction, step)])
    }

    fn go_to(&mut self, direction: Direction) {
        self.jump_to(direction, 1);
    }

    fn jump_to(&mut self, direction: Direction, step: usize){
        self.0.push((direction, step));
    }
}

/// An exact point in the grid
/// relative to the focused char
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct PointBlock{
    pub location: Option<Location>,
    pub block: Block,
    pub adjust: (f32, f32),
}

impl PointBlock{
    pub fn block(block: Block) -> Self {
        PointBlock{
            location: None,
            block: block,
            adjust: (0.0, 0.0),
        }
    }

    pub fn go(direction: Direction, step: usize, block: Block) -> Self {
        PointBlock{
            location: Some(Location::jump(direction, step)),
            block: block,
            adjust: (0.0, 0.0),
        }
    }

    pub fn adjust(&self, x: f32, y: f32) -> Self {
        let mut pb = self.clone();
        pb.adjust = (pb.adjust.0 + x, pb.adjust.1 + y);
        pb
    }

}


/// whether or not characters react to neighoring character
/// static are the box drawing characters
/// dynamic are the typing characters
/// static characters don't react to neighboring characters
/// while dynamic characters do
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Behavior{
    Static,  //stable
    Dynamic  //reactive
}

#[derive(Debug)]
pub struct Characteristic<'c>{
    /// these are the default behavior of the fragment
    pub properties: Vec<(Block, Signal, Vec<Fragment>)>,
    /// if condition is met, the block becomes a strong signal
    pub intensify: Vec<(Block, Condition<'c>)>,
    /// after checking intensifier, if the connection signal to these blocks 
    /// are strong
    /// then use these fragments instead
    pub intended_behavior: Vec<(Vec<Block>, Vec<Fragment>)>
}

impl <'c>Characteristic<'c> {

    ///
    /// get the natural strong signals of this
    /// character
    ///
    pub fn get_strong_signals(&self) -> Vec<Block> {
        let mut strong_signals = vec![];
        for &(ref block, ref signal, _) in &self.properties{
            if *signal == Strong{
                strong_signals.push(block.clone());
            }
        }
        strong_signals
    }

    pub fn is_strong_block(&self, arg_block: &Block) -> bool {
        self.properties.iter()
            .any(|&(ref block, ref signal, _)|
                block == arg_block && *signal == Strong
            )
    }

    // get default signal of this block
    // without applying intensifier
    pub fn get_block_signal(&self, arg_block: &Block) -> Option<Signal> {
        for &(ref block, ref signal, _) in &self.properties{
            if block == arg_block{
                return Some(signal.clone())
            }
        }
        None
    }
}



pub trait Properties{

    fn get_characteristic(&self) -> Option<Characteristic>;

    fn is(&self, ch: char) -> bool;

    fn any(&self, s: &str) -> bool;

    fn in_any(&self, ch: Vec<char>) -> bool;

    fn can_connect(&self, signal: &Signal, block: &Block) -> bool;

}

#[derive(Debug)]
pub enum Can<'c>{
   ConnectTo(Block, Signal),
   Is(char),
   Any(&'c str)
}

/// Behavioral condition
#[derive(Debug)]
pub struct Condition<'c>{
    pub loc: Location,
    pub can: Can<'c>, 
}

impl Properties for char{

    fn is(&self, ch: char) -> bool {
        *self == ch
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
    ///     ┌─┬─┬─┬─┬─┐
    ///     │a│b│c│d│e│
    ///     ├─┼─┼─┼─┼─┤
    ///     │f│g│h│i│j│
    ///     ├─┼─┼─┼─┼─┤
    ///     │k│l│m│n│o│
    ///     ├─┼─┼─┼─┼─┤
    ///     │p│q│r│s│t│
    ///     ├─┼─┼─┼─┼─┤
    ///     │u│v│w│x│y│
    ///     └─┴─┴─┴─┴─┘
    ///
    fn get_characteristic(&self) -> Option<Characteristic>{
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
        /*
        Template:

            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    ]
            })
         */
        if self.is('|'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (C, Strong, vec![line(c,w)]),
                    (W, Strong, vec![line(c,w)]),
                    ]
            })
        }
        else if self.is('-'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (K, Strong, vec![line(k,o)]),
                    (O, Strong, vec![line(k,o)])
                    ]
            })
        }
        else if self.is('_'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (U, Strong, vec![line(u,y)]),
                    (Y, Strong, vec![line(u,y)])
                    ]
            })
        }
        else if self.is('/'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (E, Strong, vec![line(u,e)]),
                    (U, Strong, vec![line(e,u)])
                    ]
            })
        }
        else if self.is('\\'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (A, Strong, vec![line(y,a)]),
                    (Y, Strong, vec![line(a,y)])
                    ]
            })
        }
        else if self.is('+'){
            Some(Characteristic{
                intensify: vec![
                        //   |     .
                        //   +     +
                        (C, Condition{
                                loc: top(),
                                can: ConnectTo(W, Medium)
                        }),
                        //   +     +
                        //   |     '
                        (W, Condition{
                                loc: bottom(),
                                can: ConnectTo(C, Medium)
                        }),
                        //   -+    '+
                        (K, Condition{
                                loc: left(),
                                can: ConnectTo(O, Weak)
                        }),
                        //    +-   +'
                        (O, Condition{
                                loc: right(),
                                can: ConnectTo(K, Weak)
                        }),
                        //    \    but not  _
                        //     +             +
                        (A, Condition{
                                loc: top_left(),
                                can: Is('\\')
                        }),
                        //     /   but not    _
                        //    +              +
                        (E, Condition{
                                loc: top_right(),
                                can: Is('/')
                        }),
                        //      +
                        //     /
                        (U, Condition{
                                loc: bottom_left(),
                                can: ConnectTo(E, Strong)
                        }),
                        //     +
                        //      \
                        (Y, Condition{
                                loc: bottom_right(),
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                        (C, Medium, vec![line(m,c)]),
                        (K, Medium, vec![line(m,k)]),
                        (O, Medium, vec![line(m,o)]),
                        (W, Medium, vec![line(m,w)]),
                        (A, Weak, vec![line(m,a)]),
                        (E, Weak, vec![line(m,e)]),
                        (U, Weak, vec![line(m,u)]),
                        (Y, Weak, vec![line(m,y)])
                    ]
            })
        }
        else if self.any("xX"){
            Some(Characteristic{
                intensify: vec![
                        //    \
                        //     x
                        (A, Condition{
                                loc: top_left(),
                                can: ConnectTo(Y, Strong)
                        }),
                        //      /
                        //     x
                        (E, Condition{
                                loc: top_right(),
                                can: ConnectTo(U, Strong)
                        }),
                        //     x
                        //    /
                        (U, Condition{
                                loc: bottom_left(),
                                can: ConnectTo(E, Strong)
                        }),
                        //      x
                        //       \
                        (Y, Condition{
                                loc: bottom_right(),
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (A, Medium, vec![line(m,a)]),
                    (E, Medium, vec![line(m,e)]),
                    (U, Medium, vec![line(m,u)]),
                    (Y, Medium, vec![line(m,y)])
                    ]
            })
        }
        else if self.any(".,"){
            Some(Characteristic{
                intensify: vec![
                     //  -.  +.
                    (K, Condition{
                        loc: left(),
                        can: ConnectTo(O, Medium)
                    }),
                    //  .-  .+
                    (O, Condition{
                        loc: right(),
                        can: ConnectTo(K, Medium)
                    }),
                    //  _.
                    (U, Condition{
                        loc: left(),
                        can: ConnectTo(Y,Strong)
                    }),
                    //  ._
                    (Y, Condition{
                        loc: right(),
                        can: ConnectTo(U,Strong)
                    }),
                    //      .
                    //     /
                    (U, Condition{
                        loc: bottom_left(),
                        can: ConnectTo(E,Strong)
                    }),
                    //      /    only for / else   _
                    //     .                        .   will connect
                    (E, Condition{
                        loc: top_right(),
                        can: Is('/')
                    }),
                    //      .
                    //       \
                    (Y, Condition{
                        loc: bottom_right(),
                        can: ConnectTo(A,Strong)
                    }),
                    //      \     only \ or else this connects as well  _
                    //       .                                           .
                    (A, Condition{
                        loc: top_left(),
                        can: Is('\\')
                    }),
                    //   .    .
                    //   |    '
                    (W, Condition{
                        loc: bottom(),
                        can: ConnectTo(C, Medium)
                    }),
                    //   |    only |
                    //   .                    
                    (C, Condition{
                        loc: top(),
                        can: Is('|')
                    }),
                    //  .'    .`
                    (J, Condition{
                        loc: right(),
                        can: Any("`'")
                    }),
                    //  '.    `.
                    (F, Condition{
                        loc: left(),
                        can: Any("`'")
                    }),
                    //    .
                    //   '
                    (P, Condition{
                        loc: bottom_left(),
                        can: Any("`'")
                    }),
                    //    .
                    //     '
                    (T, Condition{
                        loc: bottom_right(),
                        can: Any("`'")
                    }),
                    ],
                intended_behavior: vec![
                        //     .-
                        //    /
                        (vec![O,U], vec![arc(o,q,4),line(q,u)]),
                        //     .-
                        //      \
                        (vec![O,Y], vec![arc(o,s,4),line(s,y)]),
                        //     -.
                        //       \
                        (vec![K,Y], vec![arc(s,k,4),line(s,y)]),
                        //     -.
                        //     /
                        (vec![K,U], vec![line(u,q),arc(q,k,2)]),
                        //       /
                        //      .
                        //     /
                        (vec![U,E], vec![line(u,e)]),
                        //     \
                        //      .
                        //       \
                        (vec![A,Y], vec![line(a,y)]),
                        //    \
                        //     .
                        //     |
                        (vec![A,W], vec![line(a,g), arc(r,g,8),line(r,w)]),
                        //     |
                        //     .
                        //     |
                        (vec![C,W], vec![line(c,w)]),
                        //       /
                        //      .
                        //      |
                        (vec![E,W], vec![line(e,i), arc(i,r,8), line(r,w)]),
                        //     |
                        //     .
                        //    / 
                        (vec![C,U], vec![line(u,q), arc(q,h,8),line(h,c)]),
                        //     |
                        //     .
                        //      \
                        (vec![C,Y], vec![line(c,h), arc(h,s,8), line(s,y)]),
                        //  .'
                        (vec![J], vec![line(m,j)]),
                        //  '.
                        (vec![F], vec![line(m,f)]),
                        //   .
                        //  '
                        (vec![P], vec![line(m,p)]),
                        //   .
                        //    '
                        (vec![T], vec![line(m,t)]),
                        ],
                properties: vec![
                    (O, Weak, vec![arc(o,r,2)]),
                    (K, Weak, vec![arc(r,k,2)]),
                    (W, Medium, vec![line(r,w)]),
                    (U, Weak, vec![line(q,u)]),
                    (Y, Weak, vec![line(s,y)]),
                    (A, Weak, vec![line(m,a)]),
                    (E, Weak, vec![line(m,e)]),
                    (F, Weak, vec![line(m,f)]),
                    (J, Weak, vec![line(m,j)]),
                ]
            })
        }
        else if self.any("`'"){
            Some(Characteristic{
                intensify: vec![
                     //  -'   +'
                    (K, Condition{
                        loc: left(),
                        can: ConnectTo(O, Medium)
                    }),
                    //  '-    '+
                    (O, Condition{
                        loc: right(),
                        can: ConnectTo(K, Medium)
                    }),
                    //       /
                    //      '
                    (U, Condition{
                        loc: bottom_left(),
                        can: ConnectTo(E,Strong)
                    }),
                    //      \
                    //       '
                    (A, Condition{
                        loc: top_left(),
                        can: ConnectTo(Y,Strong)
                    }),
                    //        / 
                    //       '
                    (E, Condition{
                        loc: top_right(),
                        can: ConnectTo(U,Strong)
                    }),
                    //   |    .
                    //   '    '
                    (C, Condition{
                        loc: top(),
                        can: ConnectTo(W,Medium)
                    }),
                    //  '.   ',
                    (J, Condition{
                        loc: right(),
                        can: Any(".,")
                    }),
                    //  .'   .' 
                    (F, Condition{
                        loc: left(),
                        can: Any(".,")
                    }),
                    ],
                intended_behavior:vec![
                        //    \
                        //     '-
                        (vec![A,O], vec![line(a,g),arc(g,o,4)]),
                        //      /
                        //     '-
                        (vec![E,O], vec![line(e,i),arc(i,o,2)]),
                        //       /
                        //     -'
                        (vec![K,E], vec![arc(k,i,4),line(i,e)]),
                        //     \
                        //     -'
                        (vec![K,A], vec![arc(k,g,2),line(g,a)]),
                        //  '.
                        (vec![J], vec![line(c,j)]),
                        //   .'
                        (vec![F], vec![line(c,f)]),
                        
                        ],
                properties: vec![
                        (C, Medium, vec![line(h,c)]),
                        (O, Weak, vec![arc(h,o,2)]),
                        (K, Weak, vec![arc(k,h,2)]),
                        (A, Weak, vec![line(a,g)]),
                        (E, Weak, vec![line(e,i)]),
                        (F, Weak, vec![line(c,f)]),
                        (J, Weak, vec![line(c,j)]),
                    ]
            })
        }
        else if self.is('*'){
            Some(Characteristic{
                intensify: vec![
                        //    |
                        //    *
                        (C, Condition{
                                loc: top(),
                                can: ConnectTo(W, Strong)
                        }),
                        //    *
                        //    |
                        (W, Condition{
                                loc: bottom(),
                                can: ConnectTo(C, Strong)
                        }),
                        //   -*
                        (K, Condition{
                                loc: left(),
                                can: ConnectTo(O, Weak)
                        }),
                        //    *-
                        (O, Condition{
                                loc: right(),
                                can: ConnectTo(K, Weak)
                        }),
                        //    \    but not  _
                        //     *             *
                        (A, Condition{
                                loc: top_left(),
                                can: Is('\\')
                        }),
                        //     /   but not    _
                        //    *              *
                        (E, Condition{
                                loc: top_right(),
                                can: Is('/')
                        }),
                        //      *
                        //     /
                        (U, Condition{
                                loc: bottom_left(),
                                can: ConnectTo(E, Strong)
                        }),
                        //     *
                        //      \
                        (Y, Condition{
                                loc: bottom_right(),
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (C, Medium, vec![line(m,c), solid_circle(m,2)]),
                    (W, Medium, vec![line(m,w), solid_circle(m,2)]),
                    (K, Medium, vec![line(m,k), solid_circle(m,2)]),
                    (O, Medium, vec![line(m,o), solid_circle(m,2)]),
                    (A, Medium, vec![line(m,a), solid_circle(m,2)]),
                    (E, Medium, vec![line(m,e), solid_circle(m,2)]),
                    (U, Medium, vec![line(m,u), solid_circle(m,2)]),
                    (Y, Medium, vec![line(m,y), solid_circle(m,2)]),
                    ]
            })
        }
        else if self.is('o'){
            Some(Characteristic{
                intensify: vec![
                        //    |
                        //    o
                        (C, Condition{
                                loc: top(),
                                can: ConnectTo(W, Strong)
                        }),
                        //    o
                        //    |
                        (W, Condition{
                                loc: bottom(),
                                can: ConnectTo(C, Strong)
                        }),
                        //   -o
                        (K, Condition{
                                loc: left(),
                                can: ConnectTo(O, Weak)
                        }),
                        //    o-
                        (O, Condition{
                                loc: right(),
                                can: ConnectTo(K, Weak)
                        }),
                        //    \    but not  _
                        //     o             o
                        (A, Condition{
                                loc: top_left(),
                                can: Is('\\')
                        }),
                        //     /   but not    _
                        //    o              o
                        (E, Condition{
                                loc: top_right(),
                                can: Is('/')
                        }),
                        //      o
                        //     /
                        (U, Condition{
                                loc: bottom_left(),
                                can: ConnectTo(E, Strong)
                        }),
                        //     o
                        //      \
                        (Y, Condition{
                                loc: bottom_right(),
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (C, Medium, vec![line(h,c), open_circle(m,2)]),
                    (W, Medium, vec![line(r,w), open_circle(m,2)]),
                    (K, Medium, vec![open_circle(m,2)]),
                    (O, Medium, vec![open_circle(m,2)]),
                    (A, Medium, vec![line(g,a), open_circle(m,2)]),
                    (E, Medium, vec![line(i,e), open_circle(m,2)]),
                    (U, Medium, vec![line(q,u), open_circle(m,2)]),
                    (Y, Medium, vec![line(s,y), open_circle(m,2)]),
                    ]
            })
        }
        else if self.is('O'){
            Some(Characteristic{
                intensify: vec![
                        //    |
                        //    O
                        (C, Condition{
                                loc: top(),
                                can: ConnectTo(W, Strong)
                        }),
                        //    O
                        //    |
                        (W, Condition{
                                loc: bottom(),
                                can: ConnectTo(C, Strong)
                        }),
                        //   -O
                        (K, Condition{
                                loc: left(),
                                can: ConnectTo(O, Weak)
                        }),
                        //    O-
                        (O, Condition{
                                loc: right(),
                                can: ConnectTo(K, Weak)
                        }),
                        //    \    but not  _
                        //     O             O
                        (A, Condition{
                                loc: top_left(),
                                can: Is('\\')
                        }),
                        //     /   but not    _
                        //    O              O
                        (E, Condition{
                                loc: top_right(),
                                can: Is('/')
                        }),
                        //      O
                        //     /
                        (U, Condition{
                                loc: bottom_left(),
                                can: ConnectTo(E, Strong)
                        }),
                        //     O
                        //      \
                        (Y, Condition{
                                loc: bottom_right(),
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                        (C, Medium, vec![open_circle(m,3)]),
                        (W, Medium, vec![open_circle(m,3)]),
                        //TODO: properties should be able to consume
                        // some elements
                        (K, Medium, vec![open_circle(m,3)]),
                        (O, Medium, vec![open_circle(m,3)]),
                        (A, Medium, vec![line(a, &g.adjust(-0.5,-0.5)),
                                         open_circle(m,3)
                                        ]),
                        (E, Medium, vec![open_circle(m,3)]),
                        (U, Medium, vec![open_circle(m,3)]),
                        (Y, Medium, vec![open_circle(m,3)]),
                    ]
            })
        }
        else if self.is('<'){
            Some(Characteristic{
                intensify: vec![
                        // <-   <+   <'  <.
                        (O,Condition{
                            loc: right(),
                            can: ConnectTo(K, Weak)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                        (O, Medium, vec![arrow_line(o,m)]),
                        (K, Weak, vec![start_arrow_line(o,m)]),
                        (E, Weak, vec![line(m,e)]),
                        (Y, Weak, vec![line(m,y)])
                    ]
            })
        }
        else if self.is('>'){
            Some(Characteristic{
                intensify: vec![
                        //  ->  +>  .>  '>
                        (K, Condition{
                            loc: left(),
                            can: ConnectTo(O,Weak)
                        })
                        ],
                intended_behavior:vec![],
                properties: vec![
                        (K, Medium, vec![arrow_line(k,m)]),
                        (O, Weak, vec![start_arrow_line(k,m)]),
                        (A, Weak, vec![line(m,a)]),
                        (U, Weak, vec![line(m,u)]),
                    ]
            })
        }
        else if self.is('^'){
            Some(Characteristic{
                intensify: vec![
                        //    ^
                        //    |
                        (W, Condition{
                                loc: bottom(),
                                can: ConnectTo(C, Strong)
                        }),
                        //      ^
                        //     /
                        (U, Condition{
                                loc: bottom_left(),
                                can: ConnectTo(E, Strong)
                        }),
                        //     _^ 
                        //      
                        (U, Condition{
                                loc: left(),
                                can: ConnectTo(Y, Strong)
                        }),
                        //      ^_ 
                        //      
                        (Y, Condition{
                                loc: right(),
                                can: ConnectTo(U, Strong)
                        }),
                        //     -^ 
                        //      
                        (K, Condition{
                                loc: left(),
                                can: ConnectTo(O, Strong)
                        }),
                        //      ^- 
                        //      
                        (O, Condition{
                                loc: right(),
                                can: ConnectTo(K, Strong)
                        }),
                        //     ^
                        //      \
                        (Y, Condition{
                                loc: bottom_right(),
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![
                        //      ^
                        //     / \
                        (vec![U,Y], vec![line(u,m), line(m,y)]),
                        ],
                properties: vec![
                    (W, Medium, vec![arrow_line(w, h)]),
                    (U, Medium , vec![arrow_line(u, i)]),
                    (Y, Medium, vec![arrow_line(y, g)]),
                    (K, Weak, vec![arrow_line(k,i)]),
                    (O, Weak, vec![arrow_line(o,g)]),
                    ]
            })
        }
        else if self.any("vV"){
            Some(Characteristic{
                intensify: vec![
                        //    |    .
                        //    v    v
                        (C, Condition{
                                loc: top(),
                                can: ConnectTo(W, Medium)
                        }),
                        //    \
                        //     v
                        (A, Condition{
                                loc: top_left(),
                                can: ConnectTo(Y, Strong)
                        }),
                        //     /
                        //    v
                        (E, Condition{
                                loc: top_right(),
                                can: ConnectTo(U, Strong)
                        }),
                        ],
                intended_behavior:vec![
                        //    \ /
                        //     v
                        (vec![A,E], vec![line(a,m),line(m,e)]),
                        ],
                properties: vec![
                    (C, Medium, vec![arrow_line(c, r)]),
                    (A, Medium, vec![arrow_line(a, s)]),
                    (E, Medium, vec![arrow_line(e, q)])
                    ]
            })
        }
        else if self.is('('){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (E, Strong, vec![arc(e,y,8)]),
                    (Y, Strong, vec![arc(e,y,8)]),
                    (K, Weak, vec![arc(c,w,4)]),
                    (O, Weak, vec![arc(c,w,4)]),
                    (C, Weak, vec![arc(c,w,4)]),
                    (W, Weak, vec![arc(c,w,4)])
                    ]
            })
        }
        else if self.is(')'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (A, Strong, vec![arc(u,a,8)]),
                    (U, Strong, vec![arc(u,a,8)]),
                    (K, Weak, vec![arc(w,c,4)]),
                    (O, Weak, vec![arc(w,c,4)]),
                    (C, Weak, vec![arc(w,c,4)]),
                    (W, Weak, vec![arc(w,c,4)])
                    ]
            })
        }
        // HACK: remove this after transitioning to 
        //  Location, and PointBlock
        else if self.any(" ,.`'"){
            Some(Characteristic{
                intensify: vec![
                    //      .                  .'
                    //     '   when used in   '     
                    (T, Condition{
                        loc: right(),
                        can: Any(".,")
                    }),
                    (W, Condition{
                        loc: bottom(),
                        can: Any("`'")
                    }),
                    //   .▒   used in  .▒
                    //    '             '.
                    // if the side is at the right side of the line in
                    // a string, mostly there is no space to the right
                    (P, Condition{
                        loc: left(),
                        can: Any(".,")
                    }),
                        ],
                intended_behavior:vec![
                    (vec![T,W], vec![line(t, w)]),
                    (vec![P,W], vec![line(p, w)]),
                ],
                properties: vec![
                    ]
            })

        }
        else{
            let (blocks, fragments) = box_drawing::box_drawing(&self);
            let mut properties = vec![];
            for b in blocks{
                properties.push((b, Strong, fragments.clone()));
            }
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: properties, 
            })
        }
    }


    fn can_connect(&self, arg_signal: &Signal, block: &Block) -> bool {
        if let Some(character) = self.get_characteristic(){
            if let Some(signal) = character.get_block_signal(block){
                signal == *arg_signal
            }else{
                false
            }
        }else{
            false
        }
    }

}
