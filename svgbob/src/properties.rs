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
pub enum Signal{
    Silent,
    Weak,
    Medium,
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

pub struct Characteristic{
    //if condition is met, the block becomes a strong signal
    intensifier: Vec<(Condition, Block)>,
    // these are the default behavior of the fragment
    properties: Vec<(Block, Signal, Vec<Fragment>)>,
    // after modification, if all the blocks are strong
    // then use these fragments instead
    intended_behavior: (Vec<Block>, Vec<Fragment>)
}

pub trait Properties{

    fn get_signal_modifier(&self) -> Vec<(Condition, (Block, Signal))>;
    /// the default behavior of the character
    /// and the default fragment when it connect to 
    /// its block
    fn get_properties(&self) -> Vec<(Block, Signal, Vec<Fragment>)>;

    /// the intended behavior of the character
    /// when all the conditions are met
    fn get_intended_behavior(&self) -> Vec<(Vec<Condition>,Vec<Fragment>)>;

    fn is(&self, ch: char) -> bool;

    fn any(&self, s: &str) -> bool;

    fn in_any(&self, ch: Vec<char>) -> bool;

    fn can_connect_static(&self, block: &Block) -> bool;

    fn can_connect(&self, signal: &Signal, block: &Block) -> bool;

    fn get_frag_to(&self, block: &Block) -> Vec<Fragment>;

}


/// Behavioral condition
pub struct Condition{
    pub loc: Direction,
    pub connects_to: (Block, Signal)
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
                        connects_to: (O, Strong)
                    },
                    Condition{
                        loc: Right,
                        connects_to: (U, Medium)
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
                        connects_to: (K, Strong)
                    },
                    Condition{
                        loc: Left,
                        connects_to: (Y, Medium)
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
                        connects_to: (K, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        connects_to: (C, Medium)
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
                        connects_to: (O, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        connects_to: (C, Medium)
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
                        connects_to: (K, Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        connects_to: (E, Strong)
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
                        connects_to: (O, Strong)
                    },
                    Condition{
                        loc: BottomRight,
                        connects_to: (A, Strong)
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
                        connects_to: (K,Strong)
                    },
                    Condition{
                        loc: BottomRight,
                        connects_to: (A, Strong)
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
                        connects_to: (O,Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        connects_to: (E, Strong)
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
                        connects_to: (U, Strong)
                    },
                    Condition{
                        loc: BottomRight,
                        connects_to: (A, Strong)
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
                        connects_to: (Y, Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        connects_to: (E, Strong)
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
                        connects_to: (U, Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        connects_to: (E, Strong)
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
                        connects_to: (W, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        connects_to: (C, Strong)
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
                        connects_to: (Y, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        connects_to: (C, Strong)
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
                        connects_to: (U, Strong)
                    },
                    Condition{
                        loc: Bottom,
                        connects_to: (C, Strong)
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
                        connects_to: (W, Strong)
                    },
                    Condition{
                        loc: BottomLeft,
                        connects_to: (E, Strong)
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
                        connects_to: (W, Strong)
                    },
                    Condition{
                        loc: BottomRight,
                        connects_to: (A, Strong)
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
                        connects_to: (K,Strong)
                    },
                    Condition{
                        loc: Top,
                        connects_to: (W, Medium)
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
                        connects_to: (O,Strong)
                    },
                    Condition{
                        loc: Top,
                        connects_to: (W, Medium)
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
                        connects_to: (K, Strong)
                    },
                    Condition{
                        loc: TopLeft,
                        connects_to: (Y, Strong)
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
                        connects_to: (O,Strong)
                    },
                    Condition{
                        loc: TopRight,
                        connects_to: (U, Strong)
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
                        connects_to: (K,Strong)
                    },
                    Condition{
                        loc: TopRight,
                        connects_to: (U, Strong)
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
                        connects_to: (O,Strong)
                    },
                    Condition{
                        loc: TopLeft,
                        connects_to: (Y, Strong)
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
                        connects_to: (U, Strong)
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
                        connects_to: (Y, Strong)
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
                        connects_to: (A, Strong)
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
                        connects_to: (E, Strong)
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

    /// signal modifier, if conditions are met, then the signal is modified
    /// to this new signal, this will be the newly used signal
    fn get_signal_modifier(&self) -> Vec<(Condition, (Block, Signal))>{
        if self.is('+'){
            vec![
                (Condition{
                    loc: TopLeft,
                    connects_to: (Y, Strong)
                },
                (A, Strong)
                )
            ]
        }
        else if self.is('_'){
            vec![
                (Condition{
                    loc: Left,
                    connects_to: (Y, Strong)
                },
                (U,Strong)
                ),
                (Condition{
                    loc: Right,
                    connects_to: (U, Strong)
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
                        connects_to: (E, Strong)
                    },
                    Condition{
                        loc: Right,
                        connects_to: (K, Strong)
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

    fn can_connect(&self, signal: &Signal, block: &Block) -> bool {
        if self.can_connect_static(block){
            return true;
        }
        let properties = self.get_properties();
        for (conn_block, sig, frag) in properties{
            if sig == *signal && *block == conn_block{
                return true;
            }
        }
        false
    }
    



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



}
