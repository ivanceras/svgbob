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
use fragments::Fragment::{
    Line,
    ArrowLine,
    StartArrowLine,
    Arc,
    OpenCircle,
    SolidCircle,
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

    /*
    fn get_signal_modifier(&self) -> Vec<(Condition, (Block, Signal))>;
    /// the default behavior of the character
    /// and the default fragment when it connect to 
    /// its block
    fn get_properties(&self) -> Vec<(Block, Signal, Vec<Fragment>)>;

    /// the intended behavior of the character
    /// when all the conditions are met
    fn get_intended_behavior(&self) -> Vec<(Vec<Condition>,Vec<Fragment>)>;
    */
    fn get_characteristic(&self) -> Option<Characteristic>;

    fn is(&self, ch: char) -> bool;

    fn any(&self, s: &str) -> bool;

    fn in_any(&self, ch: Vec<char>) -> bool;

    //fn can_connect_static(&self, block: &Block) -> bool;

    fn can_connect(&self, signal: &Signal, block: &Block) -> bool;

    //fn get_frag_to(&self, block: &Block) -> Vec<Fragment>;

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
    pub loc: Direction,
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
    ///     │A│B│C│D│E│
    ///     ├─┼─┼─┼─┼─┤
    ///     │F│G│H│I│J│
    ///     ├─┼─┼─┼─┼─┤
    ///     │K│L│M│N│O│
    ///     ├─┼─┼─┼─┼─┤
    ///     │P│Q│R│S│T│
    ///     ├─┼─┼─┼─┼─┤
    ///     │U│V│W│X│Y│
    ///     └─┴─┴─┴─┴─┘
    ///
    fn get_characteristic(&self) -> Option<Characteristic>{
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
                    (C, Strong, vec![Line(C,W)]),
                    (W, Strong, vec![Line(C,W)]),
                    ]
            })
        }
        else if self.is('-'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (K, Strong, vec![Line(K,O)]),
                    (O, Strong, vec![Line(K,O)])
                    ]
            })
        }
        else if self.is('_'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (U, Strong, vec![Line(U,Y)]),
                    (Y, Strong, vec![Line(U,Y)])
                    ]
            })
        }
        else if self.is('/'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (E, Strong, vec![Line(U,E)]),
                    (U, Strong, vec![Line(E,U)])
                    ]
            })
        }
        else if self.is('\\'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (A, Strong, vec![Line(Y,A)]),
                    (Y, Strong, vec![Line(A,Y)])
                    ]
            })
        }
        else if self.is('+'){
            Some(Characteristic{
                intensify: vec![
                        //   |     .
                        //   +     +
                        (C, Condition{
                                loc: Top,
                                can: ConnectTo(W, Medium)
                        }),
                        //   +     +
                        //   |     '
                        (W, Condition{
                                loc: Bottom,
                                can: ConnectTo(C, Medium)
                        }),
                        //   -+    '+
                        (K, Condition{
                                loc: Left,
                                can: ConnectTo(O, Weak)
                        }),
                        //    +-   +'
                        (O, Condition{
                                loc: Right,
                                can: ConnectTo(K, Weak)
                        }),
                        //    \    but not  _
                        //     +             +
                        (A, Condition{
                                loc: TopLeft,
                                can: Is('\\')
                        }),
                        //     /   but not    _
                        //    +              +
                        (E, Condition{
                                loc: TopRight,
                                can: Is('/')
                        }),
                        //      +
                        //     /
                        (U, Condition{
                                loc: BottomLeft,
                                can: ConnectTo(E, Strong)
                        }),
                        //     +
                        //      \
                        (Y, Condition{
                                loc: BottomRight,
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                        (C, Medium, vec![Line(M,C)]),
                        (K, Medium, vec![Line(M,K)]),
                        (O, Medium, vec![Line(M,O)]),
                        (W, Medium, vec![Line(M,W)]),
                        (A, Weak, vec![Line(M,A)]),
                        (E, Weak, vec![Line(M,E)]),
                        (U, Weak, vec![Line(M,U)]),
                        (Y, Weak, vec![Line(M,Y)])
                    ]
            })
        }
        else if self.any("xX"){
            Some(Characteristic{
                intensify: vec![
                        //    \
                        //     X
                        (A, Condition{
                                loc: TopLeft,
                                can: ConnectTo(Y, Strong)
                        }),
                        //      /
                        //     X
                        (E, Condition{
                                loc: TopRight,
                                can: ConnectTo(U, Strong)
                        }),
                        //     X
                        //    /
                        (U, Condition{
                                loc: BottomLeft,
                                can: ConnectTo(E, Strong)
                        }),
                        //      X
                        //       \
                        (Y, Condition{
                                loc: BottomRight,
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (A, Medium, vec![Line(M,A)]),
                    (E, Medium, vec![Line(M,E)]),
                    (U, Medium, vec![Line(M,U)]),
                    (Y, Medium, vec![Line(M,Y)])
                    ]
            })
        }
        else if self.any(".,"){
            Some(Characteristic{
                intensify: vec![
                     //  -.  +.
                    (K, Condition{
                        loc: Left,
                        can: ConnectTo(O, Medium)
                    }),
                    //  .-  .+
                    (O, Condition{
                        loc: Right,
                        can: ConnectTo(K, Medium)
                    }),
                    //  _.
                    (U, Condition{
                        loc: Left,
                        can: ConnectTo(Y,Strong)
                    }),
                    //  ._
                    (Y, Condition{
                        loc: Right,
                        can: ConnectTo(U,Strong)
                    }),
                    //      .
                    //     /
                    (U, Condition{
                        loc: BottomLeft,
                        can: ConnectTo(E,Strong)
                    }),
                    //      /    only for / else   _
                    //     .                        .   will connect
                    (E, Condition{
                        loc: TopRight,
                        can: Is('/')
                    }),
                    //      .
                    //       \
                    (Y, Condition{
                        loc:BottomRight,
                        can: ConnectTo(A,Strong)
                    }),
                    //      \     only \ or else this connects as well  _
                    //       .                                           .
                    (A, Condition{
                        loc: TopLeft,
                        can: Is('\\')
                    }),
                    //   .    .
                    //   |    '
                    (W, Condition{
                        loc:Bottom,
                        can: ConnectTo(C, Medium)
                    }),
                    //   |    only |
                    //   .                    
                    (C, Condition{
                        loc: Top,
                        can: Is('|')
                    }),
                    //  .'    .`
                    (J, Condition{
                        loc: Right,
                        can: Any("`'")
                    }),
                    //  '.    `.
                    (F, Condition{
                        loc: Left,
                        can: Any("`'")
                    }),
                    //    .
                    //   '
                    (P, Condition{
                        loc: BottomLeft,
                        can: Any("`'")
                    }),
                    //    .
                    //     '
                    (T, Condition{
                        loc: BottomRight,
                        can: Any("`'")
                    }),
                    ],
                intended_behavior: vec![
                        //     .-
                        //    /
                        (vec![O,U], vec![Arc(O,Q,4),Line(Q,U)]),
                        //     .-
                        //      \
                        (vec![O,Y], vec![Arc(O,S,4),Line(S,Y)]),
                        //     -.
                        //       \
                        (vec![K,Y], vec![Arc(S,K,4),Line(S,Y)]),
                        //     -.
                        //     /
                        (vec![K,U], vec![Line(U,Q),Arc(Q,K,2)]),
                        //       /
                        //      .
                        //     /
                        (vec![U,E], vec![Line(U,E)]),
                        //     \
                        //      .
                        //       \
                        (vec![A,Y], vec![Line(A,Y)]),
                        //    \
                        //     .
                        //     |
                        (vec![A,W], vec![Line(A,G), Arc(R,G,8),Line(R,W)]),
                        //     |
                        //     .
                        //     |
                        (vec![C,W], vec![Line(C,W)]),
                        //       /
                        //      .
                        //      |
                        (vec![E,W], vec![Line(E,I), Arc(I,R,8), Line(R,W)]),
                        //     |
                        //     .
                        //    / 
                        (vec![C,U], vec![Line(U,Q), Arc(Q,H,8),Line(H,C)]),
                        //     |
                        //     .
                        //      \
                        (vec![C,Y], vec![Line(C,H), Arc(H,S,8), Line(S,Y)]),
                        //  .'
                        (vec![J], vec![Line(M,J)]),
                        //  '.
                        (vec![F], vec![Line(M,F)]),
                        //   .
                        //  '
                        (vec![P], vec![Line(M,P)]),
                        //   .
                        //    '
                        (vec![T], vec![Line(M,T)]),
                        ],
                properties: vec![
                    (O, Weak, vec![Arc(O,R,2)]),
                    (K, Weak, vec![Arc(R,K,2)]),
                    (W, Medium, vec![Line(R,W)]),
                    (U, Weak, vec![Line(Q,U)]),
                    (Y, Weak, vec![Line(S,Y)]),
                    (A, Weak, vec![Line(M,A)]),
                    (E, Weak, vec![Line(M,E)]),
                    (F, Weak, vec![Line(M,F)]),
                    (J, Weak, vec![Line(M,J)]),
                ]
            })
        }
        else if self.any("`'"){
            Some(Characteristic{
                intensify: vec![
                     //  -'   +'
                    (K, Condition{
                        loc: Left,
                        can: ConnectTo(O, Medium)
                    }),
                    //  '-    '+
                    (O, Condition{
                        loc: Right,
                        can: ConnectTo(K, Medium)
                    }),
                    //       /
                    //      '
                    (U, Condition{
                        loc:BottomLeft,
                        can: ConnectTo(E,Strong)
                    }),
                    //      \
                    //       '
                    (A, Condition{
                        loc:TopLeft,
                        can: ConnectTo(Y,Strong)
                    }),
                    //        / 
                    //       '
                    (E, Condition{
                        loc:TopRight,
                        can: ConnectTo(U,Strong)
                    }),
                    //   |    .
                    //   '    '
                    (C, Condition{
                        loc:Top,
                        can: ConnectTo(W,Medium)
                    }),
                    //  '.   ',
                    (J, Condition{
                        loc: Right,
                        can: Any(".,")
                    }),
                    //  .'   .' 
                    (F, Condition{
                        loc: Left,
                        can: Any(".,")
                    }),
                    ],
                intended_behavior:vec![
                        //    \
                        //     '-
                        (vec![A,O], vec![Line(A,G),Arc(G,O,4)]),
                        //      /
                        //     '-
                        (vec![E,O], vec![Line(E,I),Arc(I,O,2)]),
                        //       /
                        //     -'
                        (vec![K,E], vec![Arc(K,I,4),Line(I,E)]),
                        //     \
                        //     -'
                        (vec![K,A], vec![Arc(K,G,2),Line(G,A)]),
                        //  '.
                        (vec![J], vec![Line(C,J)]),
                        //   .'
                        (vec![F], vec![Line(C,F)]),
                        
                        ],
                properties: vec![
                        (C, Medium, vec![Line(H,C)]),
                        (O, Weak, vec![Arc(H,O,2)]),
                        (K, Weak, vec![Arc(K,H,2)]),
                        (A, Weak, vec![Line(A,G)]),
                        (E, Weak, vec![Line(E,I)]),
                        (F, Weak, vec![Line(C,F)]),
                        (J, Weak, vec![Line(C,J)]),
                    ]
            })
        }
        else if self.is('*'){
            Some(Characteristic{
                intensify: vec![
                        //    |
                        //    *
                        (C, Condition{
                                loc: Top,
                                can: ConnectTo(W, Strong)
                        }),
                        //    *
                        //    |
                        (W, Condition{
                                loc: Bottom,
                                can: ConnectTo(C, Strong)
                        }),
                        //   -*
                        (K, Condition{
                                loc: Left,
                                can: ConnectTo(O, Weak)
                        }),
                        //    *-
                        (O, Condition{
                                loc: Right,
                                can: ConnectTo(K, Weak)
                        }),
                        //    \    but not  _
                        //     *             *
                        (A, Condition{
                                loc: TopLeft,
                                can: Is('\\')
                        }),
                        //     /   but not    _
                        //    *              *
                        (E, Condition{
                                loc: TopRight,
                                can: Is('/')
                        }),
                        //      *
                        //     /
                        (U, Condition{
                                loc: BottomLeft,
                                can: ConnectTo(E, Strong)
                        }),
                        //     *
                        //      \
                        (Y, Condition{
                                loc: BottomRight,
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (C, Medium, vec![Line(M,C), SolidCircle(M,2)]),
                    (W, Medium, vec![Line(M,W), SolidCircle(M,2)]),
                    (K, Medium, vec![Line(M,K), SolidCircle(M,2)]),
                    (O, Medium, vec![Line(M,O), SolidCircle(M,2)]),
                    (A, Medium, vec![Line(M,A), SolidCircle(M,2)]),
                    (E, Medium, vec![Line(M,E), SolidCircle(M,2)]),
                    (U, Medium, vec![Line(M,U), SolidCircle(M,2)]),
                    (Y, Medium, vec![Line(M,Y), SolidCircle(M,2)]),
                    ]
            })
        }
        else if self.is('o'){
            Some(Characteristic{
                intensify: vec![
                        //    |
                        //    o
                        (C, Condition{
                                loc: Top,
                                can: ConnectTo(W, Strong)
                        }),
                        //    o
                        //    |
                        (W, Condition{
                                loc: Bottom,
                                can: ConnectTo(C, Strong)
                        }),
                        //   -o
                        (K, Condition{
                                loc: Left,
                                can: ConnectTo(O, Weak)
                        }),
                        //    o-
                        (O, Condition{
                                loc: Right,
                                can: ConnectTo(K, Weak)
                        }),
                        //    \    but not  _
                        //     o             o
                        (A, Condition{
                                loc: TopLeft,
                                can: Is('\\')
                        }),
                        //     /   but not    _
                        //    o              o
                        (E, Condition{
                                loc: TopRight,
                                can: Is('/')
                        }),
                        //      o
                        //     /
                        (U, Condition{
                                loc: BottomLeft,
                                can: ConnectTo(E, Strong)
                        }),
                        //     o
                        //      \
                        (Y, Condition{
                                loc: BottomRight,
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (C, Medium, vec![Line(H,C), OpenCircle(M,2)]),
                    (W, Medium, vec![Line(R,W), OpenCircle(M,2)]),
                    (K, Medium, vec![OpenCircle(M,2)]),
                    (O, Medium, vec![OpenCircle(M,2)]),
                    (A, Medium, vec![Line(G,A), OpenCircle(M,2)]),
                    (E, Medium, vec![Line(I,E), OpenCircle(M,2)]),
                    (U, Medium, vec![Line(Q,U), OpenCircle(M,2)]),
                    (Y, Medium, vec![Line(S,Y), OpenCircle(M,2)]),
                    ]
            })
        }
        else if self.is('O'){
            Some(Characteristic{
                intensify: vec![
                        //    |
                        //    O
                        (C, Condition{
                                loc: Top,
                                can: ConnectTo(W, Strong)
                        }),
                        //    O
                        //    |
                        (W, Condition{
                                loc: Bottom,
                                can: ConnectTo(C, Strong)
                        }),
                        //   -O
                        (K, Condition{
                                loc: Left,
                                can: ConnectTo(O, Weak)
                        }),
                        //    O-
                        (O, Condition{
                                loc: Right,
                                can: ConnectTo(K, Weak)
                        }),
                        //    \    but not  _
                        //     O             O
                        (A, Condition{
                                loc: TopLeft,
                                can: Is('\\')
                        }),
                        //     /   but not    _
                        //    O              O
                        (E, Condition{
                                loc: TopRight,
                                can: Is('/')
                        }),
                        //      O
                        //     /
                        (U, Condition{
                                loc: BottomLeft,
                                can: ConnectTo(E, Strong)
                        }),
                        //     O
                        //      \
                        (Y, Condition{
                                loc: BottomRight,
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                        (C, Medium, vec![OpenCircle(M,3)]),
                        (W, Medium, vec![OpenCircle(M,3)]),
                        //TODO: properties should be able to consume
                        // some elements
                        (K, Medium, vec![OpenCircle(M,3)]),
                        (O, Medium, vec![OpenCircle(M,3)]),
                        (A, Medium, vec![Line(A,G),OpenCircle(M,3)]),
                        (E, Medium, vec![OpenCircle(M,3)]),
                        (U, Medium, vec![OpenCircle(M,3)]),
                        (Y, Medium, vec![OpenCircle(M,3)]),
                    ]
            })
        }
        else if self.is('<'){
            Some(Characteristic{
                intensify: vec![
                        // <-   <+   <'  <.
                        (O,Condition{
                            loc: Right,
                            can: ConnectTo(K, Weak)
                        }),
                        ],
                intended_behavior:vec![],
                properties: vec![
                        (O, Medium, vec![ArrowLine(O,M)]),
                        (K, Weak, vec![StartArrowLine(O,M)]),
                        (E, Weak, vec![Line(M,E)]),
                        (Y, Weak, vec![Line(M,Y)])
                    ]
            })
        }
        else if self.is('>'){
            Some(Characteristic{
                intensify: vec![
                        //  ->  +>  .>  '>
                        (K, Condition{
                            loc: Left,
                            can: ConnectTo(O,Weak)
                        })
                        ],
                intended_behavior:vec![],
                properties: vec![
                        (K, Medium, vec![ArrowLine(K,M)]),
                        (O, Weak, vec![StartArrowLine(K,M)]),
                        (A, Weak, vec![Line(M,A)]),
                        (U, Weak, vec![Line(M,U)]),
                    ]
            })
        }
        else if self.is('^'){
            Some(Characteristic{
                intensify: vec![
                        //    ^
                        //    |
                        (W, Condition{
                                loc: Bottom,
                                can: ConnectTo(C, Strong)
                        }),
                        //      ^
                        //     /
                        (U, Condition{
                                loc: BottomLeft,
                                can: ConnectTo(E, Strong)
                        }),
                        //     _^ 
                        //      
                        (U, Condition{
                                loc: Left,
                                can: ConnectTo(Y, Strong)
                        }),
                        //      ^_ 
                        //      
                        (Y, Condition{
                                loc: Right,
                                can: ConnectTo(U, Strong)
                        }),
                        //     -^ 
                        //      
                        (K, Condition{
                                loc: Left,
                                can: ConnectTo(O, Strong)
                        }),
                        //      ^- 
                        //      
                        (O, Condition{
                                loc: Right,
                                can: ConnectTo(K, Strong)
                        }),
                        //     ^
                        //      \
                        (Y, Condition{
                                loc: BottomRight,
                                can: ConnectTo(A, Strong)
                        }),
                        ],
                intended_behavior:vec![
                        //      ^
                        //     / \
                        (vec![U,Y], vec![Line(U,M), Line(M,Y)]),
                        ],
                properties: vec![
                    (W, Medium, vec![ArrowLine(W, H)]),
                    (U, Medium , vec![ArrowLine(U, I)]),
                    (Y, Medium, vec![ArrowLine(Y, G)]),
                    (K, Weak, vec![ArrowLine(K,I)]),
                    (O, Weak, vec![ArrowLine(O,G)]),
                    ]
            })
        }
        else if self.any("vV"){
            Some(Characteristic{
                intensify: vec![
                        //    |    .
                        //    V    V
                        (C, Condition{
                                loc: Top,
                                can: ConnectTo(W, Medium)
                        }),
                        //    \
                        //     V
                        (A, Condition{
                                loc: TopLeft,
                                can: ConnectTo(Y, Strong)
                        }),
                        //     /
                        //    V
                        (E, Condition{
                                loc: TopRight,
                                can: ConnectTo(U, Strong)
                        }),
                        ],
                intended_behavior:vec![
                        //    \ /
                        //     V
                        (vec![A,E], vec![Line(A,M),Line(M,E)]),
                        ],
                properties: vec![
                    (C, Medium, vec![ArrowLine(C, R)]),
                    (A, Medium, vec![ArrowLine(A, S)]),
                    (E, Medium, vec![ArrowLine(E, Q)])
                    ]
            })
        }
        else if self.is('('){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (E, Strong, vec![Arc(E,Y,8)]),
                    (Y, Strong, vec![Arc(E,Y,8)]),
                    (K, Weak, vec![Arc(C,W,4)]),
                    (O, Weak, vec![Arc(C,W,4)]),
                    (C, Weak, vec![Arc(C,W,4)]),
                    (W, Weak, vec![Arc(C,W,4)])
                    ]
            })
        }
        else if self.is(')'){
            Some(Characteristic{
                intensify: vec![
                        ],
                intended_behavior:vec![],
                properties: vec![
                    (A, Strong, vec![Arc(U,A,8)]),
                    (U, Strong, vec![Arc(U,A,8)]),
                    (K, Weak, vec![Arc(W,C,4)]),
                    (O, Weak, vec![Arc(W,C,4)]),
                    (C, Weak, vec![Arc(W,C,4)]),
                    (W, Weak, vec![Arc(W,C,4)])
                    ]
            })
        }
        // HACK: remove this after transitioning to 
        //  Location, and PointBlock
        else if self.is(' '){
            Some(Characteristic{
                intensify: vec![
                    //      .                  .'
                    //     '   when used in   '     
                    (T, Condition{
                        loc: Right,
                        can: Any(".,")
                    }),
                    (V, Condition{
                        loc: Bottom,
                        can: Any("`'")
                    }),
                    //   .▒   used in  .▒
                    //    '             '.
                    // if the side is at the right side of the line in
                    // a string, mostly there is no space to the right
                    (P, Condition{
                        loc: Left,
                        can: Any(".,")
                    }),
                    (X, Condition{
                        loc: Bottom,
                        can: Any("`'")
                    }),
                        ],
                intended_behavior:vec![
                    (vec![T,V], vec![Line(T,V)]),
                    (vec![P,X], vec![Line(P,X)]),
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

    /*
    /// get the intended behavior
    /// if all LocConnectsTo evaluates to True
    /// Then the fragement will be used
    fn get_intended_behavior(&self) -> Vec<(Vec<Condition>,Vec<Fragment>)>{

        if self.any(".,"){
            vec![
                // -._
                //
                (vec![
                    Condition{
                        loc: Left,
                        can: ConnectTo(O, Strong)
                    },
                    Condition{
                        loc: Right,
                        can: ConnectTo(U, Medium)
                    }
                ],
                vec![
                    Line(K,L),
                    Line(L,X),
                    Line(X,Y),
                ]),
                // _.-
                //
                (vec![
                    Condition{
                        loc: Right,
                        can: ConnectTo(K, Strong)
                    },
                    Condition{
                        loc: Left,
                        can: ConnectTo(Y, Medium)
                    }
                ],
                vec![
                    Line(U,V),
                    Line(V,N),
                    Line(N,O),
                ]),
                //    .-   .-
                //    |    '
                //
                // (vec![(W,Strong),(O,Strong)],vec![Arc(O,R,2),Line(R,W)])
                (vec![
                    Condition{
                        loc: Right,
                        can: ConnectTo(K, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        can: ConnectTo(C, Medium)
                    }
                ],
                vec![
                    Arc(O,R,2),
                    Line(R,W)
                ]),

                //   -. -.
                //    |  '
                //
                (vec![
                    Condition{
                        loc: Left,
                        can: ConnectTo(O, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        can: ConnectTo(C, Medium)
                    }
                ],
                vec![
                    Arc(R,K,2),
                    Line(R,W)
                ]),
                //    .-
                //   / 
                //
                (vec![
                    Condition{
                        loc: Right,
                        can: ConnectTo(K, Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        can: ConnectTo(E, Strong)
                    }
                ],
                vec![
                    Arc(O,Q,4),
                    Line(Q,U)
                ]),

                //     -.
                //       \
                //
                (vec![
                    Condition{
                        loc: Left,
                        can: ConnectTo(O, Strong)
                    },
                    Condition{
                        loc: BottomRight,
                        can: ConnectTo(A, Strong)
                    }
                ],
                vec![
                    Arc(S,K,4),
                    Line(S,Y)
                ]),

                //  .-    
                //   \    
                //
                (vec![
                    Condition{
                        loc: Right,
                        can: ConnectTo(K,Strong)
                    },
                    Condition{
                        loc: BottomRight,
                        can: ConnectTo(A, Strong)
                    }
                ],
                vec![
                    Arc(O,S,2),
                    Line(S,Y)
                ]),
                //   -.
                //   /    
                //
                (vec![
                    Condition{
                        loc: Left,
                        can: ConnectTo(O,Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        can: ConnectTo(E, Strong)
                    }
                ],
                vec![
                    Arc(Q,K,2),
                    Line(Q,U)
                ]),
                //     /
                //    .
                //     \
                (vec![
                    Condition{
                        loc: TopRight,
                        can: ConnectTo(U, Strong)
                    },
                    Condition{
                        loc: BottomRight,
                        can: ConnectTo(A, Strong)
                    }
                ],
                vec![
                    Arc(I,S,4),
                    Line(E,I),
                    Line(S,Y)
                ]),

                //   \  
                //    .
                //   /
                (vec![
                    Condition{
                        loc: TopLeft,
                        can: ConnectTo(Y, Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        can: ConnectTo(E, Strong)
                    }
                ],
                vec![
                    Arc(Q,G,4),
                    Line(A,G),
                    Line(U,Q)
                ]),

                //     /
                //    .
                //   /
                (vec![
                    Condition{
                        loc: TopRight,
                        can: ConnectTo(U, Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        can: ConnectTo(E, Strong)
                    }
                ],
                vec![
                    Line(E,U)
                ]),

                //    | 
                //    .
                //    |
                (vec![
                    Condition{
                        loc: Top,
                        can: ConnectTo(W, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        can: ConnectTo(C, Strong)
                    }
                ],
                vec![
                    Line(C,W)
                ]),
                //   \ 
                //    .
                //    |
                (vec![
                    Condition{
                        loc: TopLeft,
                        can: ConnectTo(Y, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        can: ConnectTo(C, Strong)
                    }
                ],
                vec![
                    Line(A,G),
                    Arc(R,G,8),
                    Line(R,W)
                ]),

                //     /
                //    .
                //    | 
                (vec![
                    Condition{
                        loc: TopRight,
                        can: ConnectTo(U, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        can: ConnectTo(C, Strong)
                    }
                ],
                vec![
                    Arc(I,R,8),
                    Line(E,I),
                    Line(R,W)
                ]),

                //    | 
                //    .
                //   /
                (vec![
                    Condition{
                        loc: Top,
                        can: ConnectTo(W, Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        can: ConnectTo(E, Strong)
                    }
                ],
                vec![
                    Line(C,H),
                    Arc(Q,H,8),
                    Line(Q,U),
                ]),
                //    | 
                //    .
                //     \
                (vec![
                    Condition{
                        loc: Top,
                        can: ConnectTo(W, Strong)
                    },
                    Condition{
                        loc: BottomRight,
                        can: ConnectTo(A, Strong)
                    }
                ],
                vec![
                    Line(C,H),
                    Arc(H,S,8),
                    Line(S,Y)
                ]),
            ]
        }
        else if self.any("'`"){
            vec![
                //   |    .
                //   '-   '-
                //
                (vec![
                    Condition{
                        loc: Right,
                        can: ConnectTo(K,Strong)
                    },
                    Condition{
                        loc: Top,
                        can: ConnectTo(W, Medium)
                    }
                ],
                vec![
                    Arc(H,O,2),
                    Line(C,H)
                ]),
                //    |   .
                //   -'  -'
                //
                (vec![
                    Condition{
                        loc: Left,
                        can: ConnectTo(O,Strong)
                    },
                    Condition{
                        loc: Top,
                        can: ConnectTo(W, Medium)
                    }
                ],
                vec![
                    Arc(K,H,2),
                    Line(C,H)
                ]),
                //  \ 
                //   '-
                //
                (vec![
                    Condition{
                        loc: Right,
                        can: ConnectTo(K, Strong)
                    },
                    Condition{
                        loc: TopLeft,
                        can: ConnectTo(Y, Strong)
                    }
                ],
                vec![
                    Arc(G,O,4),
                    Line(G,A)
                ]),
                //      /
                //    -'
                //
                (vec![
                    Condition{
                        loc: Left,
                        can: ConnectTo(O,Strong)
                    },
                    Condition{
                        loc: TopRight,
                        can: ConnectTo(U, Strong)
                    }
                ],
                vec![
                    Arc(K,I,4),
                    Line(I,E)
                ]),
                //    /
                //   '-
                //
                (vec![
                    Condition{
                        loc: Right,
                        can: ConnectTo(K,Strong)
                    },
                    Condition{
                        loc: TopRight,
                        can: ConnectTo(U, Strong)
                    }
                ],
                vec![
                    Arc(I,O,2),
                    Line(E,I)
                ]),

                //    \
                //    -'
                //
                (vec![
                    Condition{
                        loc: Left,
                        can: ConnectTo(O,Strong)
                    },
                    Condition{
                        loc: TopLeft,
                        can: ConnectTo(Y, Strong)
                    }
                ],
                vec![
                    Arc(K,G,2),
                    Line(A,G)
                ]),
            ]
        }
        else if self.is('|'){
            vec![
                //     /
                //    |
                //
                (vec![
                    Condition{
                        loc: TopRight,
                        can: ConnectTo(U, Strong)
                    }
                ],
                vec![
                    Line(W,M),
                    Line(M,E)
                ]),
                //   \  
                //    |
                //
                (vec![
                    Condition{
                        loc: TopLeft,
                        can: ConnectTo(Y, Strong)
                    }
                ],
                vec![
                    Line(W,M),
                    Line(A,M),
                ]),
                //    | 
                //     \
                //
                (vec![
                    Condition{
                        loc: BottomRight,
                        can: ConnectTo(A, Strong)
                    }
                ],
                vec![
                    Line(C,M),
                    Line(M,Y)
                ]),
                //    | 
                //   /  
                //
                (vec![
                    Condition{
                        loc: BottomLeft,
                        can: ConnectTo(E, Strong)
                    },
                ],
                vec![
                    Line(C,M),
                    Line(M,U),
                ]),
            ]
        }
        else{
            vec![(vec![], vec![])]
        }
    }
    */

    /*

    /// signal modifier, if conditions are met, then the signal is modified
    /// to this new signal, this will be the newly used signal
    fn get_signal_modifier(&self) -> Vec<(Condition, (Block, Signal))>{
        if self.is('+'){
            vec![
                (Condition{
                    loc: TopLeft,
                    can: ConnectTo(Y, Strong)
                },
                (A, Strong)
                )
            ]
        }
        else if self.is('_'){
            vec![
                (Condition{
                    loc: Left,
                    can: ConnectTo(Y, Strong)
                },
                (U,Strong)
                ),
                (Condition{
                    loc: Right,
                    can: ConnectTo(U, Strong)
                },
                (Y,Strong)
                )
            ]
        }
        else{
            vec![]
        }
    }

    /// get the behavior of the character and
    /// enumerate the direction that character can connect to 
    /// and their equivalent connection signal strength
    /// need to return Vec<Fragment>  since there is a character like O- which
    /// just connects but don't need any additional fragment
    /// and there are character that needs more fragment like the static
    /// double lined box drawing
    fn get_properties(&self) -> Vec<(Block, Signal, Vec<Fragment>)>{
        if self.is('+'){
            vec![
                (C, Strong, vec![Line(M,C)]),
                (K, Strong, vec![Line(M,K)]),
                (K, Strong, vec![Line(M,K)]),
                (O, Strong, vec![Line(M,O)]),
                (W, Strong, vec![Line(M,W)]),
                (A, Medium, vec![Line(M,A)]),
                (E, Medium, vec![Line(M,E)]),
                (U, Medium, vec![Line(M,U)]),
                (Y, Medium, vec![Line(M,Y)])
            ]
        }
        else if self.any("xX"){
            vec![
                (A, Medium, vec![Line(M,A)]),
                (E, Medium, vec![Line(M,E)]),
                (U, Medium, vec![Line(M,U)]),
                (Y, Medium, vec![Line(M,Y)])
            ]
        }
        //   \|/ 
        //   -*-
        //   /|\
        else if self.any("*"){
            vec![
                (C, Medium, vec![Line(M,C), SolidCircle(M,2)]),
                (W, Medium, vec![Line(M,W), SolidCircle(M,2)]),
                (K, Medium, vec![Line(M,K), SolidCircle(M,2)]),
                (O, Medium, vec![Line(M,O), SolidCircle(M,2)]),
                (A, Medium, vec![Line(M,A), SolidCircle(M,2)]),
                (E, Medium, vec![Line(M,E), SolidCircle(M,2)]),
                (U, Medium, vec![Line(M,U), SolidCircle(M,2)]),
                (Y, Medium, vec![Line(M,Y), SolidCircle(M,2)]),
            ]
        }
        //   \|/  
        //   -o- 
        //   /|\ 
        //
        //   Issue:     \   Strong Y
        //             o
        //  This shouldn't connect
        else if self.is('o'){
            vec![
                (C, Medium, vec![Line(H,C), OpenCircle(M,2)]),
                (W, Medium, vec![Line(R,W), OpenCircle(M,2)]),
                (K, Medium, vec![OpenCircle(M,2)]),
                (O, Medium, vec![OpenCircle(M,2)]),
                (A, Medium, vec![Line(G,A), OpenCircle(M,2)]),
                (E, Medium, vec![Line(I,E), OpenCircle(M,2)]),
                (U, Medium, vec![Line(Q,U), OpenCircle(M,2)]),
                (Y, Medium, vec![Line(S,Y), OpenCircle(M,2)]),
            ]
        }
        else if self.is('O'){
            vec![
                (C, Medium, vec![OpenCircle(M,3)]),
                (W, Medium, vec![OpenCircle(M,3)]),
                (K, Medium, vec![OpenCircle(M,3)]),
                (O, Medium, vec![OpenCircle(M,3)]),
                (A, Medium, vec![Line(A,G),OpenCircle(M,3)]),
                (E, Medium, vec![OpenCircle(M,3)]),
                (U, Medium, vec![OpenCircle(M,3)]),
                (Y, Medium, vec![OpenCircle(M,3)]),
            ]
        }
        //                                              \ \    / /
        //   .-  ,-  -. -,  .  ,   -.  -,  .  ,  -. -,   . ,  . ,
        //                 /  /    /   /   |  |    \  \  | |  | |
        else if self.any(".,"){
            vec![
                (O, Weak, vec![Arc(O,R,2)]),
                (K, Weak, vec![Arc(R,K,2)]),
                (W, Medium, vec![Line(R,W)]),
                (U, Weak,vec![Line(Q,U)]),
                (Y, Weak, vec![Line(S,Y)]),
                (A, Weak, vec![Line(M,A)]),
                (E, Weak, vec![Line(M,E)]),
                (F, Weak, vec![Line(M,F)]),
                (J, Weak, vec![Line(M,J)]),
                /*
                 * Modifier:
                 * If conditions BottomLeft can strongly connect to its E
                 * and Right can strongly connect to K
                 * then this elements connection to O and U will be strong with the 
                 * fragment as Arc(O,Q) and Line(Q,U)
                (vec![
                    Condition{
                        loc: BottomLeft,
                        can: ConnectTo(E, Strong)
                    },
                    Condition{
                        loc: Right,
                        can: ConnectTo(K, Strong)
                    },
                ], 
                (vec![O,U], Strong, vec![Arc(O,Q,4),Line(Q,U)])
                )
                */
                // If U and O are both strong, then have the nice curve and line
                //(vec![(U,Strong), (O,Strong)], vec![Arc(O,Q,4), Line(Q,U)])
            ]
        }
        //   |  |    \ \   / /    |  |   /  /
        //   `- '-    ` ' ' `    -` -' -` -'
        else if self.any("`'"){
            vec![
                (C, Medium, vec![Line(H,C)]),
                (O, Weak, vec![Arc(H,O,2)]),
                (K, Weak, vec![Arc(K,H,2)]),
                (A, Weak, vec![Line(A,G)]),
                (E, Weak, vec![Line(E,I)]),
                (F, Weak, vec![Line(C,F)]),
                (J, Weak, vec![Line(C,J)]),
            ]
        }
        else if self.is('-'){
            vec![
                (K, Strong, vec![Line(K,O)]),
                (O, Strong, vec![Line(K,O)])
            ]
        }
        else if self.is('_'){
            vec![
                (U, Medium, vec![Line(U,Y)]),
                (Y, Medium, vec![Line(Y,U)])
            ]
        }
        else if self.is('|'){
            vec![
                (C, Strong, vec![Line(C,W)]),
                (W, Strong, vec![Line(C,W)]),
                (U, Silent, vec![]),
                (Y, Silent, vec![]),
                (A, Silent, vec![]),
                (E, Silent, vec![])
            ]
        }
        else if self.is('\\'){
            vec![
                (A, Strong, vec![Line(Y,A)]),
                (Y, Strong, vec![Line(A,Y)])
            ]
        }
        else if self.is('/'){
            vec![
                (E, Strong, vec![Line(U,E)]),
                (U, Strong, vec![Line(E,U)])
            ]
        }
        //    ^     ^    ^
        //    |    /      \
        else if self.any("^"){
            vec![
                (W, Medium, vec![ArrowLine(W, H)]),
                (U, Medium , vec![ArrowLine(U, I)]),
                (Y, Medium, vec![ArrowLine(Y, G)])
            ]
        }
        //    |  |    \    /
        //    v  V     V  V
        else if self.any("vV"){
            vec![
                (C, Medium, vec![ArrowLine(C, R)]),
                (A, Medium, vec![ArrowLine(A, S)]),
                (E, Medium, vec![ArrowLine(E, Q)])
            ]
        }  
        //         /  )          /
        //   <-  <' <'  <. <.   <  -<
        //                )  \   \
        else if self.is('<'){
             vec![
                (O, Medium, vec![ArrowLine(O,M)]),
                (K, Weak, vec![StartArrowLine(O,M)]),
                (E, Weak, vec![Line(M,E)]),
                (Y, Weak, vec![Line(M,Y)])
            ]
        }
        //      \    (             \
        //  ->   `>   `>   .>  ,>   >   >-
        //                (   (    /
        else if self.is('>'){
            vec![
                (K, Medium, vec![ArrowLine(K,M)]),
                (O, Weak, vec![StartArrowLine(K,M)]),
                (A, Weak, vec![Line(M,A)]),
                (U, Weak, vec![Line(M,U)]),
            ]
        }
        //    /         |
        //   (    -(-   (
        //    \         |
        else if self.is('('){
            vec![
                (E, Strong, vec![Arc(E,Y,8)]),
                (Y, Strong, vec![Arc(E,Y,8)]),
                (K, Weak, vec![Arc(C,W,4)]),
                (O, Weak, vec![Arc(C,W,4)]),
                (C, Weak, vec![Arc(C,W,4)]),
                (W, Weak, vec![Arc(C,W,4)])
            ]
        }
        //                 s   w
        //  \           |    \|
        //   )   -)-    )     )  example of strong and weak signal
        //  /           |     |  situation, however this can connect both
        else if self.is(')'){
            vec![
                (A, Strong, vec![Arc(U,A,8)]),
                (U, Strong, vec![Arc(U,A,8)]),
                (K, Weak, vec![Arc(W,C,4)]),
                (O, Weak, vec![Arc(W,C,4)]),
                (C, Weak, vec![Arc(W,C,4)]),
                (W, Weak, vec![Arc(W,C,4)])
            ]
        }
        else{
            vec![]
        }
    }


    fn can_connect_static(&self, block: &Block) -> bool {
        let (blocks, frags) = box_drawing::box_drawing(&self);
        if blocks.contains(block){
            return true;
        }
        false
    }
    */

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
    


    /*

    ///
    /// get the fragment when this element connects to
    ///
    fn get_frag_to(&self, block: &Block) -> Vec<Fragment> {
        let properties = self.get_properties();
        for (conn_block, signal, frag) in properties{
            if *block == conn_block{
                return frag
            }
        }
        vec![]
    }
    */



}
