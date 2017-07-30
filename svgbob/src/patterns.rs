use Element;
use Point;
use Loc;
use Grid;
use Settings;
use properties::{Behavior, Signal};
use properties::{Condition, Characteristic};
use box_drawing;

use properties::Location;
use fragments::{Block};
use fragments::Block::{
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y
};
use properties::PointBlock;

use fragments::Fragment;
use fragments::Fragment::{
    Line,
    ArrowLine,
    StartArrowLine,
    Arc,
    Text,
};

use fragments::Direction;
use fragments::Direction::{
    Top,Bottom,
    Left,Right,
    TopLeft,TopRight,
    BottomLeft,BottomRight
};
use properties::Behavior::{Static,Dynamic};
use properties::Signal::{Silent, Weak,Medium,Strong};
use properties::Properties;
use ::{
    line, solid_circle,
    arrow_arc, arrow_sweep_arc,
    arc, open_circle, arrow_line,
    text, blank_text
};

use properties::Can::{self,ConnectTo,Is,Any};



struct LocBlock{
    loc: Loc, 
    settings: Settings
}

impl LocBlock{

    fn text_width(&self) -> f32 {
        self.settings.text_width
    }

    fn text_height(&self) -> f32 {
        self.settings.text_height
    }

    fn loc_x(&self) -> f32 {
        self.loc.x as f32
    }

    fn loc_y(&self) -> f32 {
        self.loc.y as f32
    }

    fn loc(&self) -> Loc {
        self.loc.clone()
    }

    fn tw1(&self) -> f32 {
        self.text_width() * 1.0/4.0
    }

    fn tw2(&self) -> f32 {
        self.text_width() * 1.0/2.0
    }

    fn tw3(&self) -> f32 {
        self.text_width() * 3.0/4.0
    }

    fn tw4(&self) -> f32 {
        self.text_width() * 1.0
    }

    fn th1(&self) -> f32 {
        self.text_height() * 1.0/4.0
    }

    fn th2(&self) -> f32 {
        self.text_height() * 1.0/2.0
    }

    fn th3(&self) -> f32 {
        self.text_height() * 3.0/4.0
    }


    fn th4(&self) -> f32 {
        self.text_height() * 1.0
    }

    /// x coordinate on increment of 1/4 of text width
    fn x0(&self) -> f32 {
        self.loc_x() * self.text_width()
    }

    fn x1(&self) -> f32 {
        (self.loc_x() + 1.0/4.0) * self.text_width()
    }

    fn x2(&self) -> f32 {
        (self.loc_x() + 1.0/2.0) * self.text_width()
    }

    fn x3(&self) -> f32 {
        (self.loc_x() + 3.0/4.0) * self.text_width()
    }

    fn x4(&self) -> f32 {
        (self.loc_x() + 1.0) * self.text_width()
    }

    /// y coordinate on increment of 1/4 of text_height
    fn y0(&self) -> f32 {
        self.loc_y() * self.text_height()
    }

    fn y1(&self) -> f32 {
        (self.loc_y() + 1.0/4.0) * self.text_height()
    }

    fn y2(&self) -> f32 {
        (self.loc_y() + 1.0/2.0) * self.text_height()
    }

    fn y3(&self) -> f32 {
        (self.loc_y() + 3.0/4.0) * self.text_height()
    }

    fn y4(&self) -> f32 {
        (self.loc_y() + 1.0) * self.text_height()
    }

    /// 1st row a,b,c,d,e
    fn a(&self) -> Point{
         Point::new( self.x0(), self.y0() )
    }

    fn b(&self) -> Point{
        Point::new( self.x1(), self.y0() )
    }
    
    fn c(&self) -> Point {
        Point::new( self.x2(), self.y0() )
    }
    
    fn d(&self) -> Point{
        Point::new( self.x3(), self.y0() )
    }

    fn e(&self) -> Point{
        Point::new( self.x4(), self.y0() )
    }

    /// 2nd row f,g,h,i,j
    fn f(&self) -> Point{
        Point::new( self.x0(), self.y1() )
    }

    fn g(&self) -> Point{
        Point::new( self.x1(), self.y1() )
    }

    fn h(&self) -> Point{
        Point::new( self.x2(), self.y1() )
    }

    fn i(&self) -> Point {
        Point::new( self.x3(), self.y1() )
    }

    fn j(&self) -> Point {
        Point::new( self.x4(), self.y1() )
    }

    /// 3rd row k,l,m,n,o
    fn k(&self) -> Point {
        Point::new( self.x0(), self.y2() )
    }

    fn l(&self) -> Point {
        Point::new( self.x1(), self.y2() )
    }

    fn m(&self) -> Point {
        Point::new( self.x2(), self.y2() )
    }

    fn n(&self) -> Point {
        Point::new( self.x3(), self.y2() )
    }

    fn o(&self) -> Point {
        Point::new( self.x4(), self.y2() )
    }

    /// 4th row p,q,r,s,t
    fn p(&self) -> Point {
        Point::new( self.x0(), self.y3() )
    }

    fn q(&self) -> Point {
        Point::new( self.x1(), self.y3() )
    }

    fn r(&self) -> Point {
        Point::new( self.x2(), self.y3() )
    }

    fn s(&self) -> Point {
        Point::new( self.x3(), self.y3() )
    }

    fn t(&self) -> Point {
        Point::new( self.x4(), self.y3() )
    }

    /// 5th row u,v,w,x,y
    fn u(&self) -> Point {
        Point::new( self.x0(), self.y4() )
    }

    fn v(&self) -> Point {
        Point::new( self.x1(), self.y4() )
    }

    fn w(&self) -> Point {
        Point::new( self.x2(), self.y4() )
    }

    fn x(&self) -> Point {
        Point::new( self.x3(), self.y4() )
    }

    fn y(&self) -> Point {
        Point::new( self.x4(), self.y4() )
    }
    
    pub fn to_point(&self, pb: &PointBlock) -> Point {
        let mut p = match pb.block{
            A => self.a(),
            B => self.b(),
            C => self.c(),
            D => self.d(),
            E => self.e(),
            F => self.f(),
            G => self.g(),
            H => self.h(),
            I => self.i(),
            J => self.j(),
            K => self.k(),
            L => self.l(),
            M => self.m(),
            N => self.n(),
            O => self.o(),
            P => self.p(),
            Q => self.q(),
            R => self.r(),
            S => self.s(),
            T => self.t(),
            U => self.u(),
            V => self.v(),
            W => self.w(),
            X => self.x(),
            Y => self.y()
        };
        let unit = self.tw1();
        p.adjust(pb.adjust.0 * unit, pb.adjust.1 * unit);
        p
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct FocusChar<'g>{
    loc: Loc,
    ch: char,
    grid: &'g Grid
}


impl <'g>FocusChar<'g>{

    pub fn new(loc: &Loc, grid: &'g Grid) -> Self {
        let s:Option<&String> = grid.get(loc);
        /// if there is a text in this location, take the first char as the focus char
        let ch = match s{
            Some(s) => {s.chars().nth(0).unwrap_or('\0')}
            None => {'\0'}
        };

        Self{
            loc: loc.clone(),
            ch: ch,
            grid: grid
        }
    }


    /// get the text of self char, including complex block
    /// concatenated with multiple strings in utf8 encoding
    fn text(&self) -> String {
        match self.grid.get(&self.loc){
            Some(s) => s.to_owned(),
            None => "".to_string()
        }
    }

    /// get the focus char at this location
    fn get(&self, loc: &Loc) -> Self{
        FocusChar::new(loc, self.grid)
    }


    /// if the character matches given argument
    pub fn is(&self, ch: char) -> bool {
        self.ch == ch
    }

    /// if character is any character in the string
    fn any(&self, ch: &str) -> bool {
        ch.contains(self.ch)
    }

    fn in_any(&self, chars: Vec<char>) -> bool {
        chars.contains(&self.ch)
    }

    fn used_as_text(&self) -> bool {
        if self.is_text_surrounded(){
            true
        }
        else{
            false
        }
    }

    fn is_text_surrounded(&self) -> bool {
        self.left().ch.is_alphanumeric()
        || self.right().ch.is_alphanumeric()
        
    }



    pub fn is_null(&self) -> bool {
        self.is('\0')
    }

    pub fn is_blank(&self) -> bool {
        self.is_null() || self.is(' ')
    }


    ///////////////////////////////////
    //
    //  can strongly or mediumly connect
    //
    ///////////////////////////////////
    
    fn can_pass_medium_connect(&self, block: &Block) -> bool {
        self.can_strongly_connect(block)
        || self.can_medium_connect(block)
    }


    fn can_pass_weakly_connect(&self, block: &Block) -> bool {
        self.can_strongly_connect(block)
        || self.can_medium_connect(block)
        || self.can_weakly_connect(block)
    }

    fn can_pass_silently_connect(&self, block: &Block) -> bool {
        self.can_strongly_connect(block)
        || self.can_medium_connect(block)
        || self.can_weakly_connect(block)
        || self.can_silently_connect(block)
    }

    fn can_strongly_connect(&self, block: &Block) -> bool {
        self.ch.can_connect(&Strong, block)
    }

    fn can_medium_connect(&self, block: &Block) -> bool {
        self.ch.can_connect(&Medium, block)
    }

    fn can_weakly_connect(&self, block: &Block) -> bool {
        self.ch.can_connect(&Weak, block)
    }

    fn can_silently_connect(&self, block: &Block) -> bool {
        self.ch.can_connect(&Silent, block)
    }

    fn point(&self, pb: &PointBlock) -> Point {
       self.loc_block().to_point(pb) 
    }

    fn loc_block(&self) -> LocBlock{
        LocBlock{
            loc: self.loc.to_owned(),
            settings: self.get_settings(),
        }
    }

    fn to_element(&self, frag: Fragment) -> Element {
        let tw1 = self.loc_block().tw1();
        match frag{
            Fragment::Line(p1, p2) => {
                line(&self.point(&p1),
                    &self.point(&p2),
                )
            },
            Fragment::ArrowLine(p1, p2) => {
                arrow_line(&self.point(&p1),
                    &self.point(&p2)
                )
            },

            Fragment::StartArrowLine(p1, p2) => {
                arrow_line(&self.point(&p1),
                    &self.point(&p2)
                )
            },

            Fragment::Arc(p1, p2, m) => {
                arc(&self.point(&p1),
                    &self.point(&p2),
                    m as f32 *  tw1
                )
            },
            
            Fragment::OpenCircle(c, m) => {
                open_circle(&self.point(&c), m as f32 * tw1)
            },

            Fragment::SolidCircle(c, m) => {
                solid_circle(&self.point(&c), m as f32 * tw1)
            }
            Fragment::Text(s) => {
                text(&self.loc, &s)
            }
        }
    }


    /// TODO: optimize this by getting accumulating the location
    /// and convert it into loc in 1 call
    /// then get the focus char at this location;
    fn from_location(&self, location: &Location) -> FocusChar<'g>{
        let loc = self.loc.from_location(location);
        self.get(&loc)
    }


    fn can_block_pass_connect(&self,
        block: &Block, signal: &Signal) -> bool {
        match *signal{
            Strong => self.can_strongly_connect(block),
            Medium => self.can_pass_medium_connect(block),
            Weak => self.can_pass_weakly_connect(block),
            Silent => self.can_pass_silently_connect(block),
        }
    }

    pub fn get_elements(&self) -> (Vec<Element>, Vec<Loc>){
        let (fragments,location) = self.get_fragments(); 
        let elements: Vec<Element> = fragments.into_iter()
            .map(|frag| self.to_element(frag) )
            .collect();
        (elements, vec![])
    }

    /// check to see if this specified block for this focused
    /// char is intensified to be strong
    fn is_intensified(&self, arg_block: &Block) -> bool {
        let character:Option<Characteristic> = self.ch.get_characteristic(); 
        if let Some(character) = character{
            //println!("intensify : {:#?}", character.intensify);
            for &(ref block, ref cond) in &character.intensify{
                if block == arg_block{
                    match cond.can{
                        ConnectTo(ref cond_block, ref signal) => {
                            let fc = self.from_location(&cond.loc);
                            if fc.can_block_pass_connect(&cond_block, signal){
                                return true; 
                            }
                        },
                        Is(char) => {
                            let fc = self.from_location(&cond.loc);
                            if fc.is(char){
                                return true;
                            }
                        },
                        Any(s) => {
                            let fc = self.from_location(&cond.loc);
                            if fc.any(s){
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn can_be_strong_block(&self, block: &Block) -> bool {
        if self.is_strong_block(block){
            true
        }
        else if self.is_intensified(block){
            true
        }
        else{
            false
        }
    }

    fn is_strong_block(&self, block: &Block) -> bool {
        let character:Option<Characteristic> = self.ch.get_characteristic(); 
        if let Some(character) = character{
            if character.is_strong_block(block){
                return true;
            }
        }
        false
    }

    fn get_block_signal(&self, block: &Block) -> Option<Signal> {
        let character:Option<Characteristic> = self.ch.get_characteristic(); 
        if let Some(character) = character{
            character.get_block_signal(block)
        }else{
            None
        }
    }
    fn get_strong_signals(&self) -> Vec<Block> {
        let character:Option<Characteristic> = self.ch.get_characteristic(); 
        if let Some(character) = character{
            character.get_strong_signals()
        }else{
            vec![]
        }
    }

    fn get_fragments(&self) -> (Vec<Fragment>, Vec<Location>){
        let character:Option<Characteristic> = self.ch.get_characteristic(); 
        let mut elm: Vec<Fragment> = vec![];
        if let Some(character) = character{
            let mut matched_intended = false;
            // intended behaviors when signals are strong
            // after applying the intensifiers
            for &(ref blocks, ref fragments) in &character.intended_behavior{
                let meet = blocks.iter()
                    .all(|ref b| self.can_be_strong_block(&b));
                if meet{
                    elm.extend(fragments.clone());
                    matched_intended = true;
                }
            }
            // default behaviors
            // add only when signal is strong
            // or the signal has been intensified to strong
            let mut matched = false;
            if !matched_intended{
                for &(ref block, ref signal, ref fragments) in &character.properties{
                    // draw when a strong block and not used as text
                    if self.is_strong_block(&block)/* && !self.used_as_text()*/{
                        elm.extend(fragments.clone());
                        matched = true;
                    }
                    // draw when used as text but intensified 
                    else if self.is_intensified(&block){
                        elm.extend(fragments.clone());
                        matched = true;
                    }
                }
            }
            if !matched && !matched_intended && !self.is_blank(){
                elm.push(Text(self.text()));
            }
            (elm, vec![])
        }
        else{
            if !self.is_blank(){// This is to disconnect words
                elm.push(Text(self.text()));
            }
            (elm, vec![])
        }
    }


    fn get_settings(&self) -> Settings {
        self.grid.settings.clone()
    }

    pub fn top(&self) -> Self {
        self.get(&self.loc.top())
    }

    pub fn bottom(&self) -> Self {
       self.get(&self.loc.bottom())
    }

    pub fn left(&self) -> Self {
       self.get(&self.loc.left())
    }

    pub fn in_left(&self, n: usize) -> Self {
        let mut fc = self.left();
        for i in 0..n-1{
            fc = fc.left();
        }
        fc
    }
    pub fn in_right(&self, n: usize) -> Self {
        let mut fc = self.right();
        for i in 0..n-1{
            fc = fc.right();
        }
        fc
    }

    pub fn in_top(&self, n: usize) -> Self {
        let mut fc = self.top();
        for i in 0..n-1{
            fc = fc.top();
        }
        fc
    }
    pub fn in_bottom(&self, n: usize) -> Self {
        let mut fc = self.bottom();
        for i in 0..n-1{
            fc = fc.bottom();
        }
        fc
    }

    pub fn right(&self) -> Self {
       self.get(&self.loc.right())
    }

    pub fn top_left(&self) -> Self {
       self.get(&self.loc.top_left())
    }

    pub fn top_right(&self) -> Self {
       self.get(&self.loc.top_right())
    }

    pub fn bottom_left(&self) -> Self {
       self.get(&self.loc.bottom_left())
    }

    pub fn bottom_right(&self) -> Self {
       self.get(&self.loc.bottom_right())
    }



}


#[cfg(test)]
mod test{
use super::super::Settings;
use super::Grid;
use super::FocusChar;
use super::super::Loc;
use ::properties::Location;
use ::fragments::Direction::*;

use fragments::Block::{
        A,B,C,D,E,
        F,G,H,I,J,
        K,L,M,N,O,
        P,Q,R,S,T,
        U,V,W,X,Y
    };
use properties::Signal::{
        Weak,
        Medium,
        Strong
    };

    #[test]
    fn test_adjascent(){
            let g = Grid::from_str("a统öo͡͡͡", &Settings::compact());
            let fc = FocusChar::new(&Loc::new(1,0), &g);
            println!("{:?}", fc);
            assert!(fc.left().is('a'));
            assert!(fc.right().right().is('ö'));
    }

    #[test]
    fn test100(){
        //  ._
        let g = Grid::from_str(".-", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(0,0), &g);
        let (frags, consumed) = fc.get_fragments();
        println!("frags: {:?}", frags);
        assert!(fc.is_intensified(&O));
        assert!(fc.can_be_strong_block(&O));
    }

    #[test]
    fn test_location(){
        //  ._
        let g = Grid::from_str(".-", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(0,0), &g);
        let (frags, consumed) = fc.get_fragments();
        let loc = &Location::go(Right);
        let go_right = fc.from_location(&Location::go(Right));
        let right = fc.right();
        let right2 = fc.in_right(2);
        let mut right2_loop = fc.clone();
        for _ in 0..2{
            right2_loop = right2_loop.in_right(1);
        }
        println!("in right 2: {:?}", right2.loc);
        println!("in right 2 loop: {:?}", right2_loop.loc);
        assert_eq!(right2.loc, right2_loop.loc);
        assert_eq!(go_right.loc, right.loc);
    }

    #[test]
    fn test_loc(){
        let g = Grid::from_str("", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(0,0), &g);
        let right = fc.right();
        let in_right = fc.in_right(1);
        assert_eq!(Loc::new(1,0), right.loc);
        assert_eq!(Loc::new(1,0), in_right.loc);
    }

    #[test]
    fn test1(){
        //  ._
        let g = Grid::from_str("._", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(0,0), &g);
        println!("focus char: {:#?}", fc);
        let (frags, consumed) = fc.get_fragments();
        println!("frags: {:?}", frags);
        println!("block signal Y: {:?}",fc.get_block_signal(&Y));
        println!("strong signals: {:?}", fc.get_strong_signals());
        assert_eq!(Some(Weak), fc.get_block_signal(&Y));
        assert!(!fc.is_intensified(&U));
        assert!(fc.is_intensified(&Y));
    }
    #[test]
    fn test2(){
        //  ._
        let g = Grid::from_str("._", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(1,0), &g);
        println!("focus char: {:#?}", fc);
        let (frags, consumed) = fc.get_fragments();
        println!("frags: {:?}", frags);
        println!("block signal Y: {:?}",fc.get_block_signal(&Y));
        println!("strong signals: {:?}", fc.get_strong_signals());
        assert_eq!(Some(Strong), fc.get_block_signal(&Y));
        assert_eq!(Some(Strong), fc.get_block_signal(&U));
        assert!(!fc.is_intensified(&Y));
        assert!(!fc.is_intensified(&U));
        assert!(fc.can_be_strong_block(&Y));
        assert!(fc.can_be_strong_block(&U));
    }
}
