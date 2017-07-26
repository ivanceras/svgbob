
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
    else if ch.any("╭┌┍┎┏"){
        (vec![W,O],
         vec![
            Line(M,W),
            Line(M,O)
        ])
    }
    else if ch.any("╮┐┑┒┓"){
        (vec![W,K],
         vec![
             Line(M,W),
             Line(M,K)
        ])
    }
    else if ch.any("╰┗└┕┖"){
        (vec![C,O],
         vec![Line(M,C),
            Line(M,O)
        ])
    }
    else if ch.any("╯┘┙┚┛"){
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
            Line(P,T),
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
