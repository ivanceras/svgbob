
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
use properties::Behavior;
use properties::Behavior::{Static,Dynamic};
use properties::Signal;
use properties::Signal::{Silent,Weak,Medium,Strong};
use properties::Properties;


pub fn box_drawing(ch: &char) -> (Vec<Block>, Vec<Fragment>) {
    //////////////////////////////
    //
    //  Static are all Strong signal
    //  and are used Box Drawing
    //
    ////////////////////////////////
    if ch.in_any(vec!['─','━']){
        (vec![K,O],
         vec![Line(K,O)]
        )
    }
    else if ch.any("│┃"){
        (vec![C,W],
         vec![Line(C,W)]
        )
    }
    else if ch.is('╭'){
        (vec![O,W],
         vec![
            Arc(O,R,2),
            Line(R,W)
            ]
        )
    }
    else if ch.any("┌┍┎┏"){
        (vec![W,O],
         vec![
            Line(M,W),
            Line(M,O)
        ])
    }
    else if ch.is('╮'){
        (vec![K,W],
         vec![
            Line(W,R),
            Arc(R,K,2)
         ])
    }
    else if ch.any("┐┑┒┓"){
        (vec![W,K],
         vec![
             Line(M,W),
             Line(M,K)
        ])
    }
    else if ch.is('╰'){
        (vec![C,O],
         vec![Line(C,H),
            Arc(H,O,2)
            ]
        )
    }
    else if ch.any("┗└┕┖"){
        (vec![C,O],
         vec![Line(M,C),
            Line(M,O)
        ])
    }
    else if ch.is('╯'){
        (vec![C,K],
         vec![Line(C,H),
            Arc(K,H,2)
            ]
        )
    }
    else if ch.any("┘┙┚┛"){
        (vec![C,K],
         vec![Line(M,C),
            Line(M,K)
        ])
    }
    else if ch.any("┼┽┾┿╀╁╂╃╄╅╆╇╈╉╊╋"){
        (vec![C,W,K,O],
         vec![Line(M,C),
            Line(M,W),
            Line(M,K),
            Line(M,O),
        ])
    }
    else if ch.any("┬┭┮┯┰┱┲┳"){
        (vec![W,K,O],
         vec![Line(M,W),
            Line(M,K),
            Line(M,O)
        ])
    }
    else if ch.any("┴┵┶┷┸┹┺┻"){
        (vec![C,K,O],
         vec![Line(M,C),
            Line(M,K),
            Line(M,O)
        ])
    }
    else if ch.any("├┝┞┟┠┡┢┣"){
        (vec![C,W,O],
         vec![Line(M,C),
            Line(M,W),
            Line(M,O)
        ])
    }
    else if ch.any("┤┥┦┧┨┩┪┫"){
        (vec![C,W,K],
         vec![Line(M,C),
            Line(M,W),
            Line(M,K)
        ])
    }
    else if ch.is('║'){
        (vec![B,V,D,X],
         vec![Line(B,V),
            Line(V,B),
            Line(D,X),
            Line(X,D),
        ])
    }
    else if ch.is('═'){
        (vec![K,O,P,T],
         vec![Line(K,O),
            Line(K,O),
            Line(P,T),
        ])
    }
    else if ch.is('╔'){
        (vec![O,V,T,X],
         vec![Line(O,L),
            Line(L,V),
            Line(T,S),
            Line(S,X)
        ])
    }
    else if ch.is('╗'){
        (vec![K,X,P,V],
         vec![Line(K,N),
            Line(N,X),
            Line(P,Q),
            Line(Q,V)
        ])
    }
    else if ch.is('╚'){
        (vec![B,T,D,O],
         vec![Line(B,Q),
            Line(Q,T),
            Line(D,N),
            Line(N,O)
        ])
    }
    else if ch.is('╝'){
        (vec![P,K,B,D],
         vec![Line(P,S),
            Line(S,D),
            Line(K,L),
            Line(L,B)
        ])
    }
    else if ch.is('╒'){
        (vec![W,O,T],
         vec![
            Line(M,W),
            Line(M,O),
            Line(R,T),
        ])
    }
    else if ch.is('╓'){
        (vec![O,V,X],
          vec![
              Line(L,O),
              Line(L,V),
              Line(N,X)
        ])
    }
    else if ch.is('╬'){
        (vec![B,D,V,X,K,P,O,T],
         vec![
            Line(B,L), Line(L,K),
            Line(P,Q), Line(Q,V),
            Line(D,N), Line(N,O),
            Line(T,S), Line(S,X)
         ])
    }
    else if ch.is('╦'){
        (vec![K,O,P,V,T,X],
         vec![
            Line(K,O),
            Line(P,Q),
            Line(Q,V),
            Line(T,S),
            Line(S,X)
        ])
    }
    else if ch.is('╩'){
        (vec![P,T,K,B,D,O],
         vec![
            Line(P,T),
            Line(K,L),
            Line(L,B),
            Line(D,N),
            Line(N,O)
        ])
    }
    else if ch.is('╠'){
        (vec![B,V,D,O,T,X],
         vec![
            Line(B,V),
            Line(D,N),
            Line(N,O),
            Line(T,S),
            Line(S,X)
         ])
    }
    else if ch.is('╣'){
        (vec![D,X,B,K,P,V],
         vec![
            Line(D,X),
            Line(B,L), Line(L,K),
            Line(P,Q), Line(Q,V)
         ])
    }
    /*
    else if ch.any("╙╟"){
         vec![
            (O, Strong, vec![Line(M,O)])
        ])
    }
    else if ch.any("╡╪╞"){
         vec![
            (C, Strong, vec![Line(M,C)]),
            (W, Strong, vec![Line(M,W)])
        ])
    }
    else if ch.any("╕╤"){
        vec![
            (W, Strong, vec![Line(M,W)])
        ])
    }
    else if ch.any("╥╫"){
        vec![
            (K, Strong, vec![Line(M,K)]),
            (O, Strong, vec![Line(M,O)])
        ])
    }
    else if ch.any("╖╜╢"){
        vec![
            (K, Strong, vec![Line(M,K)])
        ])
    }
    else if ch.any("╛╘"){
        vec![
            (C, Strong, vec![Line(M,C)])
        ])
    }
    */
    else{
        (vec![], vec![])
    }
}
