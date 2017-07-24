use fragments::Direction;

use fragments::Block;
use fragments::Block::{
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y
};

use fragments::{
    line_to,
    line_from,
    middle_line,
    center_line,
};
use fragments::Fragment;
use fragments::Fragment::{
    Line,
    ArrowLine,
    StartArrowLine,
    Arc,
};

use Element;


use fragments::Direction::{
    Top,Bottom,
    Left,Right,
    TopLeft,TopRight,
    BottomLeft,BottomRight
};
use self::Behavior::{Static,Dynamic};
use self::Signal::{Weak,Medium,Strong};


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
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Signal{
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

pub trait Properties{
    fn get_properties(&self) -> (Behavior, Vec<(Direction, Signal, Fragment)>);

    fn is(&self, ch: char) -> bool;

    fn any(&self, s: &str) -> bool;

    fn in_any(&self, ch: Vec<char>) -> bool;

    fn can_strongly_connect(&self, dir: &Direction) -> bool;

    fn can_weakly_connect(&self, dir: &Direction) -> bool;

    fn can_medium_connect(&self, dir: &Direction) -> bool;

    fn get_frag_to(&self, dir: &Direction) -> Option<Fragment>;
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

    /// get the behavior of the character and
    /// enumerate the direction that character can connect to 
    /// and their equivalent connection signal strength
    /// need to return Vec<Fragment>  since there is a character like O- which
    /// just connects but don't need any additional fragment
    /// and there are character that needs more fragment like the static
    /// double lined box drawing
    fn get_properties(&self) -> (Behavior, Vec<(Direction, Signal, Fragment)>) {
        if self.is('+'){
            (Dynamic,
            vec![
                (Top, Medium, line_to(Top)),
                (Left, Medium, line_to(Left)),
                (Left, Medium, line_to(Left)),
                (Right, Medium, line_to(Right)),
                (Bottom, Medium, line_to(Bottom)),
                (TopLeft, Weak, line_to(TopLeft)),
                (TopRight,Weak, line_to(TopRight)),
                (BottomLeft, Weak, line_to(BottomLeft)),
                (BottomRight, Weak, line_to(BottomRight))
            ])
        }
        else if self.any("xX"){
            (Dynamic,
            vec![
                (TopLeft, Medium, line_to(TopLeft)),
                (TopRight, Medium, line_to(TopRight)),
                (BottomLeft, Medium, line_to(BottomLeft)),
                (BottomRight, Medium, line_to(BottomRight))
            ])
        }
        //   \|/ 
        //   -*-
        //   /|\
        else if self.any("*"){
            (Dynamic,
            vec![
                (Top, Weak, line_to(Top)),
                (Bottom, Weak, line_to(Bottom)),
                (Left, Weak, line_to(Left)),
                (Right, Weak, line_to(Right)),
                (TopLeft, Weak, line_to(TopLeft)),
                (TopRight, Weak, line_to(TopRight)),
                (BottomLeft, Weak, line_to(BottomLeft)),
                (BottomRight, Weak, line_to(BottomRight)),
            ])
        }
        //   \|/  
        //   -o- 
        //   /|\ 
        else if self.any("oO"){
            (Dynamic,
            vec![
                (Top, Weak, Line(H,C)),
                (Bottom, Weak, Line(R,W)),
                (Left, Weak, line_to(Left)),
                (Right, Weak, line_to(Right)),
                (TopLeft, Weak, line_to(TopLeft)),
                (TopRight, Weak, line_to(TopRight)),
                (BottomLeft, Weak, line_to(BottomLeft)),
                (BottomRight, Weak, line_to(BottomRight)),
            ])
        }
        //                                              \ \    / /
        //   .-  ,-  -. -,  .  ,   -.  -,  .  ,  -. -,   . ,  . ,
        //                 /  /    /   /   |  |    \  \  | |  | |
        else if self.any(".,"){
            (Dynamic,
            vec![
                (Right, Medium, Arc(O,R,2)),
                (Left, Medium, Arc(R,K,2)),
                (Bottom, Medium, Line(R,W)),
                (BottomLeft, Medium, Line(Q,U)),
                (BottomRight, Medium, Line(S,Y)),
                (TopLeft, Weak, line_to(TopLeft)),
                (TopRight, Weak, line_to(TopRight))
            ])
        }
        //   |  |    \ \   / /    |  |   /  /
        //   `- '-    ` ' ' `    -` -' -` -'
        else if self.any("`'"){
            (Dynamic,
            vec![
                (Top, Medium, Line(H,C)),
                (Right, Medium, Arc(H,O,2)),
                (Left, Medium, Arc(K,H,2)),
                (TopLeft, Medium, Line(A,G)),
                (TopRight, Medium, Line(E,I))
            ])
        }
        else if self.is('-'){
            (Dynamic,
            vec![
                (Left, Strong, middle_line()),
                (Right, Strong, middle_line())
            ])
        }
        else if self.is('_'){
            (Dynamic,
            vec![
                (BottomLeft, Strong, Line(U,Y)),
                (BottomRight, Strong, Line(Y,U))
            ])
        }
        else if self.is('|'){
            (Dynamic,
            vec![
                (Top, Strong, center_line()),
                (Bottom, Strong, center_line())
            ])
        }
        else if self.is('\\'){
            (Dynamic,
            vec![
                (TopLeft, Strong, Line(Y,A)),
                (BottomRight, Strong, Line(A,Y))
            ])
        }
        else if self.is('/'){
            (Dynamic,
            vec![
                (TopRight, Strong, Line(U,E)),
                (BottomLeft, Strong, Line(E,U))
            ])
        }
        //    ^     ^    ^
        //    |    /      \
        else if self.any("^"){
            (Dynamic, 
            vec![
                (Bottom, Medium, ArrowLine(W, H)),
                (BottomLeft, Medium , ArrowLine(U, I)),
                (BottomRight, Medium, ArrowLine(Y, G))
            ])
        }
        //    |  |    \    /
        //    v  V     V  V
        else if self.any("vV"){
            (Dynamic, 
            vec![
                (Top, Medium, ArrowLine(C, R)),
                (TopLeft, Medium, ArrowLine(A, S)),
                (TopRight, Medium, ArrowLine(E, Q))
            ])
        }  
        //         /  )          /
        //   <-  <' <'  <. <.   <  -<
        //                )  \   \
        else if self.is('<'){
            (Dynamic, 
             vec![
                (Right, Medium, ArrowLine(O,M)),
                (Left, Weak, StartArrowLine(O,M)),
                (TopRight, Weak, line_to(TopRight)),
                (BottomRight, Weak, line_to(BottomRight))
            ])
        }
        //      \    (             \
        //  ->   `>   `>   .>  ,>   >   >-
        //                (   (    /
        else if self.is('>'){
            (Dynamic, 
            vec![
                (Left, Medium, ArrowLine(K,M)),
                (Right, Weak, StartArrowLine(K,M)),
                (TopLeft, Weak, line_from(TopLeft)),
                (BottomLeft, Weak, line_from(BottomLeft)),
            ])
        }
        //    /         |
        //   (    -(-   (
        //    \         |
        else if self.is('('){
            (Dynamic, 
            vec![
                (TopRight, Medium, Arc(E,Y,8)),
                (BottomRight, Medium, Arc(E,Y,8)),
                (Left, Weak, Arc(C,W,4)),
                (Right, Weak, Arc(C,W,4)),
                (Top, Weak, Arc(C,W,4)),
                (Bottom, Weak, Arc(C,W,4))
            ])
        }
        //                 s   w
        //  \           |    \|
        //   )   -)-    )     )  example of strong and weak signal
        //  /           |     |  situation, however this can connect both
        else if self.is(')'){
            (Dynamic, 
            vec![
                (TopLeft, Medium, Arc(U,A,8)),
                (BottomLeft, Medium, Arc(U,A,8)),
                (Left, Weak, Arc(W,C,4)),
                (Right, Weak, Arc(W,C,4)),
                (Top, Weak, Arc(W,C,4)),
                (Bottom, Weak, Arc(W,C,4))
            ])
        }
        //////////////////////////////
        //
        //  Static are all Strong signal
        //  and are used Box Drawing
        //
        ////////////////////////////////
        else if self.in_any(vec!['─','━']){
            (Static,
            vec![
                (Left, Strong, middle_line()),
                (Right, Strong, middle_line())
            ])
        }
        else if self.any("│┃"){
            (Static,
            vec![
                (Top, Strong, center_line()),
                (Bottom, Strong, center_line())
            ])
        }
        else if self.any("╭┌┍┎┏"){
            (Static, 
            vec![
                (Bottom, Strong, line_to(Bottom)),
                (Right, Strong, line_to(Right))
            ])
        }
        else if self.any("╮┐┑┒┓"){
            (Static, 
            vec![
                (Bottom, Strong, line_to(Bottom)),
                (Left, Strong, line_to(Left))
            ])
        }
        else if self.any("╰┗└┕┖"){
            (Static, 
            vec![
                (Top, Strong, line_to(Top)),
                (Right, Strong, line_to(Right))
            ])
        }
        else if self.any("╯┘┙┚┛"){
            (Static, 
            vec![
                (Top, Strong, line_to(Top)),
                (Left, Strong, line_to(Left))
            ])
        }
        else if self.any("┼┽┾┿╀╁╂╃╄╅╆╇╈╉╊╋"){
            (Static, 
            vec![
                (Top, Strong, line_to(Top)),
                (Bottom, Strong, line_to(Bottom)),
                (Left, Strong, line_to(Left)),
                (Right, Strong, line_to(Right))
            ])
        }
        else if self.any("┬┭┮┯┰┱┲┳"){
            (Static, 
            vec![
                (Bottom, Strong, line_to(Bottom)),
                (Left, Strong, line_to(Left)),
                (Right, Strong, line_to(Right))
            ])
        }
        else if self.any("┴┵┶┷┸┹┺┻"){
            (Static, 
            vec![
                (Top, Strong, line_to(Top)),
                (Left, Strong, line_to(Left)),
                (Right, Strong, line_to(Right))
            ])
        }
        else if self.any("├┝┞┟┠┡┢┣"){
            (Static, 
            vec![
                (Top, Strong, line_to(Top)),
                (Bottom, Strong, line_to(Bottom)),
                (Right, Strong, line_to(Right))
            ])
        }
        else if self.any("┤┥┦┧┨┩┪┫"){
            (Static, 
            vec![
                (Top, Strong, line_to(Top)),
                (Bottom, Strong, line_to(Bottom)),
                (Left, Strong, line_to(Left))
            ])
        }
        else if self.any("╒"){
            (Static, 
            vec![
                (Bottom, Strong, line_to(Bottom))
            ])
        }
        else if self.any("╓╙╟"){
            (Static, 
             vec![
                (Right, Strong, line_to(Right))
            ])
        }
        else if self.any("╡╪╞"){
            (Static, 
             vec![
                (Top, Strong, line_to(Top)),
                (Bottom, Strong, line_to(Bottom))
            ])
        }
        else if self.any("╕╤"){
            (Static, 
            vec![
                (Bottom, Strong, line_to(Bottom))
            ])
        }
        else if self.any("╥╫"){
            (Static, 
            vec![
                (Left, Strong, line_to(Left)),
                (Right, Strong, line_to(Right))
            ])
        }
        else if self.any("╖╜╢"){
            (Static, 
            vec![
                (Left, Strong, line_to(Left))
            ])
        }
        else if self.any("╛╘"){
            (Static, 
            vec![
                (Top, Strong, line_to(Top))
            ])
        }
        else{
            (Dynamic, vec![])
        }
    }

    /// self character is dynamic and can connect to
    /// self enumerated direction
    fn can_medium_connect(&self, dir: &Direction) -> bool{
        let (beh, dirs_signal) = self.get_properties();
        for (con_dir, signal, frag) in dirs_signal{
            if signal == Medium && dir.to_block() == con_dir.to_block(){
                return true;
            }
        }
        false
    }

    fn can_weakly_connect(&self, dir: &Direction) -> bool {
        let (beh, dirs_signal) = self.get_properties();
        for (con_dir, signal, frag) in dirs_signal{
            if signal == Weak && dir.to_block() == con_dir.to_block(){
                return true;
            }
        }
        false
    }
    

    fn can_strongly_connect(&self, dir: &Direction) -> bool {
        let (beh, dirs_signal) = self.get_properties();
        for (con_dir, signal, frag) in dirs_signal{
            if signal == Strong && dir.to_block() == con_dir.to_block(){
                return true;
            }
        }
        false
    }

    ///
    /// get the fragment when this element connects to
    ///
    fn get_frag_to(&self, dir: &Direction) -> Option<Fragment> {
        let (beh, dirs_signal) = self.get_properties();
        for (conn_dir, signal, frag) in dirs_signal{
            /*
            if *dir == conn_dir {
                return Some(frag)
            }
            else{
                */
                let dir_block = &dir.to_block();
                let conn_block = conn_dir.to_block();
                if *dir_block == conn_block{
                    return Some(frag)
                }
            //}
        }
        None
    }



}
