use Element;
use Point;
use Loc;
use Grid;
use Settings;

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

use fragments::Direction;
use fragments::Direction::{
    Top,Bottom,
    Left,Right,
    TopLeft,TopRight,
    BottomLeft,BottomRight
};
use properties::Behavior::{Static,Dynamic};
use properties::Signal::{Weak,Medium,Strong};
use properties::Properties;
use ::{
    line, solid_circle,
    arrow_arc, arrow_sweep_arc,
    arc, open_circle, arrow_line,
    text, blank_text
};



struct LocBlock{
    loc: Loc, 
    block: Block,
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
    
    pub fn to_point(&self) -> Point {
        match self.block{
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
        }
    }
}

#[derive(Debug)]
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




    fn is_thin_horizontal(&self) -> bool {
        self.in_any(vec!['─'])
    }

    fn is_thick_horizontal(&self) -> bool {
        self.is('━')
    }


    fn is_horizontal(&self) -> bool {
        self.is_dynamic_horizontal()
        || self.is_static_horizontal()
    }

    fn is_dynamic_horizontal(&self) -> bool {
        self.is('-')
    }

    fn is_static_horizontal(&self) -> bool {
        self.is_thin_horizontal() 
        || self.is_thick_horizontal()
    }

    fn is_thin_vertical(&self) -> bool {
        self.is('│')
    }
    fn is_thick_vertical(&self) -> bool {
        self.is('┃')
    }

    fn is_vertical(&self) -> bool {
        self.is('|') 
        || self.is_thin_vertical() 
        || self.is_thick_vertical()
    }


    ///   .-   ,-
    ///   |    |
    fn is_dynamic_rounded_top_left(&self) -> bool {
        self.any(".,")
    }

    ///  ╭-
    ///  |
    fn is_static_rounded_top_left(&self) -> bool {
        self.is('╭')
    }

    ///   -.
    ///    |
    fn is_dynamic_rounded_top_right(&self) -> bool {
        self.is('.')
    }
    ///   -╮
    ///    |
    fn is_static_rounded_top_right(&self) -> bool {
        self.is('╮')
    }

    /// |      |   
    /// '-     `-   
    fn is_dynamic_rounded_bottom_left(&self) -> bool {
        self.any("'`")
    }

    ///   |
    ///   ╰-
    fn is_static_rounded_bottom_left(&self) -> bool {
        self.is('╰')
    }

    ///     |    |
    ///    -'   -╯
    fn is_rounded_bottom_right(&self) -> bool {
        self.is_dynamic_rounded_bottom_right()
        || self.is_static_rounded_bottom_right()
    }

    ///     |
    ///    -'
    fn is_dynamic_rounded_bottom_right(&self) -> bool {
        self.is('\'')
    }
    ///    |
    ///   -╯
    fn is_static_rounded_bottom_right(&self) -> bool {
        self.is('╯')
    }


    ///  ┌-  ┍-  ┎-  ┏-
    ///  |   |   |   |
    fn is_static_corner_top_left(&self) -> bool {
        self.any("┌┍┎┏")
    }
    /// -+  -┐  -┑  -┒  -┓ 
    ///  |   |   |   |   |
    fn is_static_corner_top_right(&self) -> bool {
        self.any("┐┑┒┓")
    }



    ///   |   |   |   |
    ///   ┗-  └-  ┕-  ┖-
    fn is_static_corner_bottom_left(&self) -> bool {
        self.any("┗└┕┖")
    }

    ///    |   |  |  |
    ///   -┘  -┙ -┚ -┛
    fn is_static_corner_bottom_right(&self) -> bool {
        self.any("┘┙┚┛")
    }

    ///   +  ┼  ┽   ┾   ┿  ╀   ╁   ╂   ╃   ╄   ╅   ╆   ╇   ╈   ╉   ╊   ╋
    fn is_center_intersection(&self) -> bool {
        self.any("+┼┽┾┿╀╁╂╃╄╅╆╇╈╉╊╋")
    }
    
    ///  ┬   ┭   ┮   ┯   ┰   ┱   ┲   ┳
    fn is_static_bottom_intersection(&self) -> bool {
        self.any("┬┭┮┯┰┱┲┳")
    }


    ///  ┴   ┵   ┶   ┷   ┸   ┹   ┺   ┻
    fn is_static_top_intersection(&self) -> bool {
        self.any("┴┵┶┷┸┹┺┻")
    }


    /// ├   ┝   ┞   ┟  ┠   ┡   ┢   ┣
    fn is_static_right_intersection(&self) -> bool {
        self.any("├┝┞┟┠┡┢┣")
    }

    ///  ┤   ┥   ┦   ┧   ┨   ┩   ┪   ┫ 
    fn is_static_left_intersection(&self) -> bool {
        self.any("┤┥┦┧┨┩┪┫")
    }

    pub fn is_blank(&self) -> bool {
        self.in_any(vec!['\0', ' '])
    }

    pub fn is_null(&self) -> bool {
        self.is('\0')
    }

    fn is_slant_left(&self) -> bool {
        self.is('\\')
    }

    fn is_slant_right(&self) -> bool {
        self.is('/')
    }

    fn is_low_horizontal(&self) -> bool {
        self.is('_')
    }

    fn is_open_round_marker(&self) -> bool {
        self.any("oO")
    }

    fn is_solid_round_marker(&self) -> bool {
        self.is('*')
    }


    ///////////////////////////////////
    //
    //  can strongly or mediumly connect
    //
    ///////////////////////////////////
    
    fn can_pass_medium_connect(&self, dir: &Direction) -> bool {
        self.ch.can_strongly_connect(dir)
        || self.ch.can_medium_connect(dir)
    }


    fn can_pass_weakly_connect(&self, dir: &Direction) -> bool {
        self.can_strongly_connect(dir)
        || self.can_medium_connect(dir)
        || self.can_weakly_connect(dir)
    }
    fn can_strongly_connect(&self, dir: &Direction) -> bool {
        self.ch.can_strongly_connect(dir)
    }

    fn can_medium_connect(&self, dir: &Direction) -> bool {
        self.ch.can_medium_connect(dir)
    }

    fn can_weakly_connect(&self, dir: &Direction) -> bool {
        self.ch.can_weakly_connect(dir)
    }

    fn get_default_element(&self, dir: &Direction) -> Element {
        let frag = self.ch.get_frag_to(dir);
        assert!(frag.is_some(), "There should be 1 frag");
        self.to_element(frag.unwrap())
    }

    fn loc_block(&self, block: Block) -> LocBlock{
        LocBlock{
            loc: self.loc.clone(),
            block: block,
            settings: self.get_settings(),
        }
    }

    fn point(&self, block: Block) -> Point {
        let lb = self.loc_block(block);
        lb.to_point()
    }

    fn to_element(&self, frag: Fragment) -> Element {
        match frag{
            Fragment::Line(p1, p2) => {
                line(&self.point(p1),
                    &self.point(p2),
                )
            },
            Fragment::ArrowLine(p1, p2) => {
                arrow_line(&self.point(p1),
                    &self.point(p2)
                )
            },

            Fragment::StartArrowLine(p1, p2) => {
                arrow_line(&self.point(p1),
                    &self.point(p2)
                )
            }
            Fragment::Arc(p1, p2, m) => {
                arc(&self.point(p1),
                    &self.point(p2),
                    m as f32 * self.loc_block(A).tw1()
                )
            }
        }
    }

    pub fn get_elements(&self) ->  (Vec<Element>, Vec<Loc>){
        let a = &self.point(A);
        let b = &self.point(B);
        let c = &self.point(C);
        let d = &self.point(D);
        let e = &self.point(E);

        let f = &self.point(F);
        let g = &self.point(G);
        let h = &self.point(H);
        let i = &self.point(I);
        let j = &self.point(J);

        let k = &self.point(K);
        let l = &self.point(L);
        let m = &self.point(M);
        let n = &self.point(N);
        let o = &self.point(O);

        let p = &self.point(P);
        let q = &self.point(Q);
        let r = &self.point(R);
        let s = &self.point(S);
        let t = &self.point(T);

        let u = &self.point(U);
        let v = &self.point(V);
        let w = &self.point(W);
        let x = &self.point(X);
        let y = &self.point(Y);
        let lb = &self.loc_block(A);
        let tw2 = lb.tw2();

        let mut elm = vec![];
        let mut consumed = vec![];
        let mut overriden = false;//manually enumerated use-case

        if self.any(".,"){
            //   .-    ,-
            //  /     /
            if self.right().can_strongly_connect(&Left)
                && self.bottom().left().can_strongly_connect(&TopRight){
                elm.push(self.to_element(Arc(O,Q,4)));
                elm.push(self.to_element(Line(Q,U)));
                overriden = true;
            }
            //   -.   -,
            //     \    \
            else if self.left().can_strongly_connect(&Right)
                && self.bottom().right().can_strongly_connect(&TopLeft){
                elm.push(self.to_element(Arc(S,K,4)));
                elm.push(self.to_element(Line(S,Y)));
                overriden = true;
            }
        }
        if self.any("`'"){
            //    /   /
            //  -'  -`
            if self.left().can_strongly_connect(&Right)
                && self.top().right().can_strongly_connect(&BottomLeft){
                elm.push(self.to_element(Arc(K,I,4)));
                elm.push(self.to_element(Line(I,E)));
                overriden = true;
            }
            //   \    \
            //    `-   '-
            else if self.right().can_strongly_connect(&Left)
                && self.top().left().can_strongly_connect(&BottomRight){
                elm.push(self.to_element(Arc(G,O,4)));
                elm.push(self.to_element(Line(A,G)));
                overriden = true;
            }
        }
        
        if !overriden{
            let mut in_weak_strong = false;
            /////////////////////////////////
            //
            //  Weak/Medium/Strong + Strong
            //
            //////////////////////////////////
            if self.can_pass_medium_connect(&Left)
                && self.left().can_strongly_connect(&Right){
                elm.push(self.get_default_element(&Left));
                in_weak_strong = true;
            }
            if self.can_pass_medium_connect(&Right)
                && self.right().can_strongly_connect(&Left){
                elm.push(self.get_default_element(&Right));
                in_weak_strong = true;
            }
            if self.can_pass_medium_connect(&Top)
                && self.top().can_strongly_connect(&Bottom){
                elm.push(self.get_default_element(&Top));
                in_weak_strong = true;
            }
            if self.can_pass_medium_connect(&Bottom)
                && self.bottom().can_strongly_connect(&Top){
                elm.push(self.get_default_element(&Bottom));
                in_weak_strong = true;
            }
            if self.can_pass_medium_connect(&TopRight)
                && self.top_right().can_strongly_connect(&BottomLeft){
                elm.push(self.get_default_element(&TopRight));
                in_weak_strong = true;
            }
            if self.can_pass_medium_connect(&TopLeft)
                && self.top_left().can_strongly_connect(&BottomRight){
                elm.push(self.get_default_element(&TopLeft));
                in_weak_strong = true;
            }
            if self.can_pass_medium_connect(&BottomLeft)
                && self.bottom_left().can_strongly_connect(&TopRight){
                elm.push(self.get_default_element(&BottomLeft));
                in_weak_strong = true;
            }
            if self.can_pass_medium_connect(&BottomRight)
                && self.bottom_right().can_strongly_connect(&TopLeft){
                elm.push(self.get_default_element(&BottomRight));
                in_weak_strong = true;
            }
            //  
            // __   \_   \/
            if self.can_pass_medium_connect(&BottomLeft)
                && self.left().can_pass_weakly_connect(&BottomRight){
                elm.push(self.get_default_element(&BottomLeft));
                in_weak_strong = true;
            }
            //
            //  __  _/  \/
            if self.can_pass_medium_connect(&BottomRight)
                && self.right().can_pass_weakly_connect(&BottomLeft){
                elm.push(self.get_default_element(&BottomRight));
                in_weak_strong = true;
            }
            //  /
            //  \
            if self.can_pass_medium_connect(&TopLeft)
                && self.top().can_strongly_connect(&BottomLeft){
                elm.push(self.get_default_element(&TopLeft));
                in_weak_strong = true;
            }
            //  /
            //  \
            if self.can_pass_medium_connect(&BottomLeft)
                && self.bottom().can_strongly_connect(&TopLeft){
                elm.push(self.get_default_element(&BottomLeft));
                in_weak_strong = true;
            }
            //  \
            //  /
            if self.can_pass_medium_connect(&TopRight)
                && self.top().can_strongly_connect(&BottomRight){
                elm.push(self.get_default_element(&TopRight));
                in_weak_strong = true;
            }
            //  \
            //  /
            if self.can_pass_medium_connect(&BottomRight)
                && self.bottom().can_strongly_connect(&TopRight){
                elm.push(self.get_default_element(&BottomRight));
                in_weak_strong = true;
            }
            ////////////////////////////////////////////
            //
            // Medium + Medium
            //
            /////////////////////////////////////////////
            if self.can_medium_connect(&Left)
                && self.left().can_medium_connect(&Right){
                elm.push(self.get_default_element(&Left));
            }
            if self.can_medium_connect(&Right)
                && self.right().can_medium_connect(&Left){
                elm.push(self.get_default_element(&Right));
            }
            if self.can_medium_connect(&Top)
                && self.top().can_medium_connect(&Bottom){
                elm.push(self.get_default_element(&Top));
            }
            if self.can_medium_connect(&Bottom)
                && self.bottom().can_medium_connect(&Top){
                elm.push(self.get_default_element(&Bottom));
            }
            if self.can_medium_connect(&TopRight)
                && self.top_right().can_medium_connect(&BottomLeft){
                elm.push(self.get_default_element(&TopRight));
            }
            if self.can_medium_connect(&TopLeft)
                && self.top_left().can_medium_connect(&BottomRight){
                elm.push(self.get_default_element(&TopLeft));
            }
            if self.can_medium_connect(&BottomLeft)
                && self.bottom_left().can_medium_connect(&TopRight){
                elm.push(self.get_default_element(&BottomLeft));
            }
            if self.can_medium_connect(&BottomRight)
                && self.bottom_right().can_medium_connect(&TopLeft){
                elm.push(self.get_default_element(&BottomRight));
            }
            /////////////////////////////////
            //
            //  Strong + Weak/Medium/Strong
            //
            //////////////////////////////////
            if !in_weak_strong{
                if self.can_strongly_connect(&Left)
                    && self.left().can_pass_medium_connect(&Right){
                    elm.push(self.get_default_element(&Left));
                }
                if self.can_strongly_connect(&Right)
                    && self.right().can_pass_medium_connect(&Left){
                    elm.push(self.get_default_element(&Right));
                }
                if self.can_strongly_connect(&Top)
                    && self.top().can_pass_medium_connect(&Bottom){
                    elm.push(self.get_default_element(&Top));
                }
                if self.can_strongly_connect(&Bottom)
                    && self.bottom().can_pass_medium_connect(&Top){
                    elm.push(self.get_default_element(&Bottom));
                }
                if self.can_strongly_connect(&TopRight)
                    && self.top_right().can_pass_medium_connect(&BottomLeft){
                    elm.push(self.get_default_element(&TopRight));
                }
                if self.can_strongly_connect(&TopLeft)
                    && self.top_left().can_pass_medium_connect(&BottomRight){
                    elm.push(self.get_default_element(&TopLeft));
                }
                if self.can_strongly_connect(&BottomLeft)
                    && self.bottom_left().can_pass_medium_connect(&TopRight){
                    elm.push(self.get_default_element(&BottomLeft));
                }
                if self.can_strongly_connect(&BottomRight)
                    && self.bottom_right().can_pass_medium_connect(&TopLeft){
                    elm.push(self.get_default_element(&BottomRight));
                }
            }
            if elm.is_empty(){
                if !self.is_blank(){
                    elm.push(text(&self.loc, &self.text()))
                }
            }
        }
        (elm, consumed)
    }

    fn get_settings(&self) -> Settings {
        self.grid.settings.clone()
    }

/*
/// abcde
/// fghij
/// klmno
/// pqrst
/// uvwxy
///
/// In grid blocks:
///            ┌─┬─┬─┬─┬─┐
///            │a│b│c│d│e│
///            ├─┼─┼─┼─┼─┤
///            │f│g│h│i│j│
///            ├─┼─┼─┼─┼─┤
///            │k│l│m│n│o│
///            ├─┼─┼─┼─┼─┤
///            │p│q│r│s│t│
///            ├─┼─┼─┼─┼─┤
///            │u│v│w│x│y│
///            └─┴─┴─┴─┴─┘
///
///
/// Issue1: There are matches that should only take 1, instead of both
/// for characters that can be interchanged
///    
///  Intended use case for + and . interchang
///    \|/
///     . 
///    /|\
///
/// but not intended interchange here +-+   .-.   since + and . are interchanged
/// How do we mark a character for specific patterns that are not interchangeable
///
/// Issue2: There should be a way to put elements adjascent on the current location
///      .'
///    .'
///   
///   returns the drawing elements, and the neighboring consumed elements
    pub fn get_elements2(&self) -> (Vec<Element>, Vec<Loc>){
        let loc = &self.loc;
        let top = self.top();
        let bottom = self.bottom();
        let left = self.left();
        let right = self.right();
        
        let top_left = self.top_left();
        let top_right = self.top_right();
        let bottom_left = self.bottom_left();
        let bottom_right = self.bottom_right();

        let top_right_right = self.get(&loc.top().right().right());
        let top_left_left = self.get(&loc.top().left().left());
        let bottom_left_left = self.get(&loc.bottom().left().left());
        let bottom_right_right = self.get(&loc.bottom().right().right());

        let a = &self.a();
        let b = &self.b();
        let c = &self.c();
        let d = &self.d();
        let e = &self.e();

        let f = &self.f();
        let g = &self.g();
        let h = &self.h();
        let i = &self.i();
        let j = &self.j();

        let k = &self.k();
        let l = &self.l();
        let m = &self.m();
        let n = &self.n();
        let o = &self.o();

        let p = &self.p();
        let q = &self.q();
        let r = &self.r();
        let s = &self.s();
        let t = &self.t();

        let u = &self.u();
        let v = &self.v();
        let w = &self.w();
        let x = &self.x();
        let y = &self.y();

        let tw1 = self.tw1();
        let tw2 = self.tw2();
        let tw3 = self.tw3();
        let tw4 = self.tw4();

        let th1 = self.th1();
        let th2 = self.th2();
        let th3 = self.th3();
        let th4 = self.th4();

        let mut elm = vec![];

        // Issue: if not returned early for the non-spaced
        // whitespaced character, CJK, fullwidth chars will be going bonkers
        if self.is_null(){
            return (vec![],vec![]);
        }

        let mut consumed:Vec<Loc> = vec![];//consumed loc
        /////////////////////////////
        //
        //    .    ,     
        //
        /////////////////////////////
        if self.is_dynamic_rounded_top_left(){
            let mut deformed = false;
            //    .    ,  .   .
            //    |    |  +   '
            if bottom.can_pass_medium_connect(&Top)
                || bottom.any("+'"){
                elm.push(line(r,w));
            }
            //    .    ,   
            //   (    (   
            if bottom_left.is('('){
                elm.extend(vec![arc(o,u,th4)]);
                deformed = true;
            }
            if right.can_pass_medium_connect(&Left){
                //  .-     ,-   
                //   \      \    
                if bottom_right.can_pass_medium_connect(&TopLeft){
                    elm.extend(vec![arc(o,s,tw2), line(s,y)]);    
                    deformed = true;
                }
                //     .-     
                //    '       
                if bottom_left.any("\'"){
                    elm.push(line(o,m));
                    deformed = true;
                }
                if bottom_left.can_pass_medium_connect(&TopRight) || bottom_left.is('('){
                    //      /     /   
                    //     .-    ,-    
                    //    /     /
                    if top_right.can_pass_medium_connect(&BottomLeft){
                        elm.push(line(q,e));
                    }
                    //    .-    ,-    .+
                    //   /     /     /  
                    if bottom_left.can_pass_medium_connect(&TopRight){
                        elm.extend(vec![arc(o,q,tw4), line(q,u)]);
                    }
                    deformed = true;
                }
                // if not deformed, connect upright arc
                //    .-   ,-   .+
                if !deformed{
                    elm.push(arc(o,r,tw2));
                }
            }
        }
        /////////////////////////////////
        //
        //      -,   acute  rounded_top_right
        //      /
        //
        //////////////////////////////////
        if self.is(',') 
            && left.can_pass_medium_connect(&Right)
            && bottom_left.can_pass_medium_connect(&TopRight){
            elm.extend(vec![line(u,q), arc(q,k,tw2)]);
        }
        ///////////////////////////////////
        //    _
        //     `
        ///////////////////////////////////
        if self.is('`')
            && top_left.is('_'){
            elm.extend(vec![
                arc(h,a,tw4)
            ]);
        }
        //////////////////////////////////
        //   _
        //  '
        //
        //////////////////////////////////
        if self.is('\'')
            && top_right.is('_'){
            let mut deformed = false;
            //     _      _
            //   ,'     .'
            //  '      '
            if left.any(".,")
                && bottom_left_left.is('\'')
                {
                elm.push(line(e,c));
                deformed = true;
            }
            //
            //    _
            //   '
            if !deformed{
                elm.extend(vec![
                    arc(e,h,tw4)
                ]);
            }
        }
        /////////////////////////////////
        //
        //    _,
        //
        ////////////////////////////////
        if self.is(',')
            && left.is('_'){
            elm.extend(vec![ 
                arc(u,r,tw4)
            ]);
        }

        ///////////////////////////////////
        //
        //    .   
        //
        ///////////////////////////////////
        if self.is_dynamic_rounded_top_right(){
            let mut deformed = false;
            //    .
            //     )
            if bottom_right.is(')'){
                elm.push(arc(y,k,th4));
                deformed = true;
            }
            //    .    .   .
            //    |    +   '
            if bottom.can_pass_medium_connect(&Top){
                elm.push(line(r,w));
            }
            if left.can_pass_medium_connect(&Right){
                //    -.    +.
                //    /     /
                if bottom_left.can_pass_medium_connect(&TopRight){
                    elm.extend(vec![line(u,q), arc(q,k,tw2)]);
                    deformed = true;
                }
                //   -.    -.
                //     `     '
                if bottom_right.any("`\'"){
                    elm.extend(vec![
                        line(k,m)
                    ]);
                    deformed = true;
                }
                if bottom_right.can_pass_medium_connect(&TopLeft){
                    //  -.    +.
                    //    \     \
                    elm.extend(vec![arc(s,k,tw4), line(s,y)]);
                    //   \
                    //   -. 
                    //     \
                    if top_left.can_pass_medium_connect(&BottomRight){
                        elm.push(line(a,y));
                    }
                    deformed = true;
                }
                //    -.   +.  
                if !deformed{
                    elm.push(arc(r,k,tw2));
                }
            }
        }
        ////////////////////////////
        //
        //      .
        //
        ////////////////////////////
        if self.is('.'){ 
            //     .
            //    / \
            if bottom_left.can_pass_medium_connect(&TopRight) && bottom_right.can_pass_medium_connect(&TopLeft){
                elm.extend(vec![line(y,s), arc(s,q,tw2), line(q,u)]);
            }
            //    \
            //     .
            //    /
            if top_left.can_pass_medium_connect(&BottomRight) && bottom_left.can_pass_medium_connect(&TopRight){
                elm.extend(vec![line(u,q),arc(q,g,th2),line(g,a)]);
            }
            //      /
            //     .
            //      \
            if top_right.can_pass_medium_connect(&BottomLeft) && bottom_right.can_pass_medium_connect(&TopLeft){
                elm.extend(vec![line(e,i), arc(i,s,th2), line(s,y)]);
            }
            //     \
            //      .
            //      |
            if top_left.can_pass_medium_connect(&BottomRight) && bottom.is_vertical(){
                elm.extend(vec![line(w,r),arc(r,g,th4),line(g,a)]);
            }
            //      |
            //      .
            //     /
            if top.is_vertical() && bottom_left.can_pass_medium_connect(&TopRight){
                elm.extend(vec![line(u,q),arc(q,h,th4),line(h,c)]);
            }
            //     /
            //    .
            //    |
            if top_right.can_pass_medium_connect(&BottomLeft) && bottom.is_vertical(){
                elm.extend(vec![line(e,i),arc(i,r,th4),line(r,w)]);
            }
            //    |
            //    .
            //     \
            if top.is_vertical() && bottom_right.can_pass_medium_connect(&TopLeft){
                elm.extend(vec![line(c,h), arc(h,s,th4), line(s,y)]);
            }
            //  ._
            if right.is_low_horizontal(){
                elm.extend(vec![arc(r,y,tw2)]);
            }
            //   _.
            if left.is_low_horizontal(){
                elm.extend(vec![arc(u,r,tw2)]);
            }
            //  .
            //  |
            if bottom.can_pass_medium_connect(&Top)
                && !right.can_pass_medium_connect(&Left)
                && !left.can_pass_medium_connect(&Right){
                elm.push(line(m,w))
            }
        }
        ////////////////////////////////
        //  Big Arc
        //
        //////////////////////////////////
        if self.any(".,"){

            //        _         _ 
            //      ,'        .'  
            //     /         /    
            //    |         |     
            if right.is('\'') && top_right_right.is('_')
                && bottom_left.is('/') && bottom_left.bottom_left().is('|'){
                    elm.extend(vec![arc(&top_right_right.u(), 
                                        &bottom_left.bottom_left().c(),
                                        tw4 * 5.0),
                                 line(&bottom_left.bottom_left().c(), 
                                      &bottom_left.bottom_left().w())
                              ]);
                    
                    consumed.extend(vec![
                                    right.loc(), 
                                    bottom_left.loc(),
                                    bottom_left.bottom_left().loc()
                        ]);
             }
             //     _
             //      `.
             //        \
             //         |
             if left.is('`') && top_left_left.is('_')
                 && bottom_right.is('\\') && bottom_right.bottom_right().is('|'){
                 elm.extend(vec![
                         arc(&bottom_right.bottom_right().c(),
                             &top_left_left.y(),
                             tw4 * 5.0),
                         line(&bottom_right.bottom_right().c(),
                              &bottom_right.bottom_right().w())
                     ]); 
                 consumed.extend(vec![
                        left.loc(),
                        bottom_right.loc(),
                        bottom_right.bottom_right().loc()
                 ]);
             }

                
             
        }

        ////////////////////////////////
        // Bigarc bottom
        //
        //   |
        //    \
        //     `._
        //
        ///////////////////////////////
        if self.any("`'"){
            if right.is('.') && self.in_right(2).is('_')
                && top_left.is('\\') && top_left.top_left().is('|'){
                elm.extend(vec![
                    arc(&top_left.top_left().w(),
                        &self.in_right(2).u(),
                        tw4 * 5.0),
                    line(&top_left.top_left().w(),
                         &top_left.top_left().c())
                ]);

                consumed.extend(vec![
                      right.loc(),
                      top_left.loc(),
                      top_left.top_left().loc()
                ]);
            }
        }
        /////////////////////////////////
        //
        //            |         |
        //           /         /
        //        _.'       _,'
        //
        ////////////////////////////////
        if self.is('\'') && left.any(".,") && self.in_left(2).is('_')
            && top_right.is('/') && top_right.top_right().is('|'){
            elm.extend(vec![
                    arc(&self.in_left(2).y(),
                        &top_right.top_right().w(),
                        tw4 * 5.0),
                    line(&top_right.top_right().w(),
                         &top_right.top_right().c())
                ]);
            consumed.extend(vec![
                left.loc(),
                top_right.loc(),
                top_right.top_right().loc()
            ]);
        }
        //////////////////////////////
        //
        //  Circle1
        //  Eat all the chars here
        //  and make a circle on the middle loc
        //    .-.    .-.
        //   (   )  (   )
        //    `-'    '-'
        //
        //////////////////////////////
        if self.in_left(2).is('(')
            && self.in_right(2).is(')')
            && top.is('-')
            && bottom.is('-')
            && top_left.is('.')
            && top_right.is('.')
            && bottom_left.any("`'")
            && bottom_right.is('\''){
            elm.push(open_circle(m, th4));
            consumed.extend(vec![
                self.in_left(2).loc(),
                self.in_right(2).loc(),
                top.loc(),
                bottom.loc(),
                top_left.loc(),
                top_right.loc(),
                bottom_left.loc(),
                bottom_right.loc(),
            ]);
        }
        //////////////////////////////
        //  Circle 2
        //
        //     .--.
        //    ( *  )
        //     `--'
        //////////////////////////////
        if self.in_left(2).is('(')
            && self.in_right(2).right().is(')')
            && top.is('-')
            && top_left.is('.')
            && top_right.is('-')
            && top_right_right.is('.')
            && bottom_left.any("`'")
            && bottom.is('-')
            && bottom_right.is('-')
            && bottom_right.right().is('\''){
            elm.push(open_circle(o, th4+tw2));
            consumed.extend(vec![
                self.in_left(2).loc(),
                self.in_right(2).right().loc(),
                top.loc(),
                bottom.loc(),
                top_left.loc(),
                top_right.loc(),
                top_right.right().loc(),
                bottom_left.loc(),
                bottom_right.loc(),
                bottom_right.right().loc(),
            ]);
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
                quadrants.push(arc(&self.in_top(2).c(),
                            &self.in_left(5).m(),
                             tw4 * 5.0 ));
                top_left_arc_matched = true;
                consumed.extend(vec![
                    self.in_left(5).loc(),
                    self.in_left(4).top().loc(),
                    self.in_left(3).top().loc(),
                    self.in_left(2).in_top(2).loc(),
                    self.left().in_top(2).loc(),
                    self.in_top(2).loc()
                ]);
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
                    arc(&self.in_right(5).m(),
                        &self.in_top(2).c(),
                        tw4 * 5.0)
                );
                top_right_arc_matched = true;
                consumed.extend(vec![
                    self.in_right(5).loc(),
                    self.in_right(4).top().loc(),
                    self.in_right(3).top().loc(),
                    self.in_right(2).in_top(2).loc(),
                    self.right().in_top(2).loc(),
                    self.in_top(2).loc()
                ]);
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
                    arc(&self.in_left(5).m(),
                        &self.in_bottom(2).w(),
                        tw4 * 5.0)
                );
                bottom_left_arc_matched = true;
                consumed.extend(vec![
                    self.in_left(5).loc(),
                    self.in_left(4).bottom().loc(),
                    self.in_left(3).bottom().loc(),
                    self.in_left(2).bottom().loc(),
                    self.left().in_bottom(2).loc(),
                    self.in_bottom(2).loc()
                ]);
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
                    arc(&self.in_bottom(2).w(),
                        &self.in_right(5).m(),
                        tw4 * 5.0)
                );
                bottom_right_arc_matched = true;
                consumed.extend(vec![
                    self.in_right(5).loc(),
                    self.in_right(4).bottom().loc(),
                    self.in_right(3).bottom().loc(),
                    self.in_right(2).bottom().loc(),
                    self.right().in_bottom(2).loc(),
                    self.in_bottom(2).loc(),
                ]);
            }
            if top_left_arc_matched
                && top_right_arc_matched
                && bottom_left_arc_matched
                && bottom_right_arc_matched{
                elm.push(open_circle(&self.m(),tw4 * 5.0));
            }
            else{
                elm.extend(quadrants);
            }
        }
        //////////////////////////////
        //
        //     '
        //
        /////////////////////////////
        if self.is('\''){
            //  \ /
            //   '
            if top_left.can_pass_medium_connect(&BottomRight) 
                && top_right.can_pass_medium_connect(&BottomLeft){
                elm.extend(vec![
                    line(a,m),
                    line(m,e)
                ]);
            }
        }
        //////////////////////////////////
        //
        //       dynamic: `   '   
        //
        /////////////////////////////////
        if self.is_dynamic_rounded_bottom_left(){
            let mut deformed = false;
            if top_left.is('('){
                //    (   (
                //     `   '
                elm.push(arc(a,o,th4));
                deformed = true;
            }
            //     
            //    '__
            if right.is_low_horizontal(){
                elm.extend(vec![line(c,r), arc(r,y,tw2)]);
            }
            //      |     |   .   .
            //      `     '   '   `
            if top.is_vertical() || top.is('.'){
                elm.push(line(c,h));
            }
            if right.can_pass_medium_connect(&Left){
                //    /   /   
                //   '-   `-  
                if top_right.can_pass_medium_connect(&BottomLeft){
                    elm.extend(vec![line(e,i), arc(i,o,tw2)]);
                    deformed = true;
                }
                if top_left.can_pass_medium_connect(&BottomRight){
                    //   \
                    //    '-
                    //     \
                    if bottom_right.can_pass_medium_connect(&TopLeft){
                        elm.push(line(g,y));
                    }
                    //  \      \
                    //   `-     '-
                    elm.extend(vec![line(a, g), arc(g, o, tw4)]);
                    deformed = true;
                }
                //  _
                //   `-
                if top_left.is_low_horizontal(){
                    elm.extend(vec![arc(h,a,tw2)]);
                }
                //      `-    '-  '+   ╰-  ╰+
                if !deformed {
                    elm.push(arc(h, o, tw2))
                }
            }
        }
        /////////////////////////////////
        //
        //    dynamic:  '
        //
        /////////////////////////////////
        if self.is_dynamic_rounded_bottom_right(){
            let mut deformed = false;
            if top_right.is(')'){
                //
                //     )
                //    '
                elm.push(arc(k,e,th4));
                deformed = true;
            }
            //  |   |   +    .
            //  '   ╯   '    '
            if top.is_vertical() || top.is('+') || top.is('.'){
                elm.push(line(h,c));
            }
            if left.can_pass_medium_connect(&Right){
                //      \
                //      -'
                if top_left.can_pass_medium_connect(&BottomRight){
                    elm.extend(vec![arc(k,g,tw2), line(g,a)]);
                    deformed = true;
                }
                if top_right.can_pass_medium_connect(&BottomLeft){
                    //       /
                    //     -'
                    //     /
                    if bottom_left.can_pass_medium_connect(&TopRight){
                        elm.push(line(i,u));
                    }
                    //       /
                    //     -'
                    elm.extend(vec![arc(k,i,tw4), line(i,e)]);
                    deformed = true;
                }
                //       _
                //     -'
                if top_right.is_low_horizontal(){
                    elm.extend(vec![arc(k,h,tw2),arc(e,h,tw2)]);
                }
                //     -'     +'
                if !deformed {
                    elm.push(arc(k,h,tw2));
                }
            }
        }

        /////////////////////////////////
        //
        //   static: ┌  ┍  ┎  ┏
        // 
        ////////////////////////////////
        if self.is_static_corner_top_left(){
            elm.extend(vec![line(m,o), line(m,w)]);
        }

        //////////////////////////
        //
        //  ┐ ┑ ┒ ┓
        //
        //////////////////////////
        if self.is_static_corner_top_right(){
            elm.extend(vec![
                line(m,k),
                line(m,w)
            ]);
        }
        //  ┗ └ ┕ ┖
        if self.is_static_corner_bottom_left(){
            elm.extend(vec![
                line(m,o),
                line(c,m)
            ]);
            
        }
        //  + ┘ ┙ ┚ ┛
        if self.is_static_corner_bottom_right(){
            elm.extend(vec![
                line(k,m), 
                line(m,c)
                 ]);
        }
        //  +  ┼  ┽   ┾   ┿  ╀   ╁   ╂   ╃   ╄   ╅   ╆   ╇   ╈   ╉   ╊   ╋
        if self.is_center_intersection(){
            //   |   .
            //   +   +
            if top.can_pass_medium_connect(&Bottom){
                elm.push(line(c,m));
            }
            //  +
            //  |
            if bottom.can_pass_medium_connect(&Top){
                elm.push(line(m,w));
            }
            // -+
            if left.can_pass_medium_connect(&Right) {
                elm.push(line(m,k));
            }
            //  +-
            if right.can_pass_medium_connect(&Left) {
                elm.push(line(m,o));
            }
            //  \
            //   +
            if top_left.can_pass_medium_connect(&BottomRight){
                elm.push(line(m,a));
            }
            //    /
            //   +
            if top_right.can_pass_medium_connect(&BottomLeft){
                elm.push(line(m,e));
            }
            //   +
            //  /
            if bottom_left.can_pass_medium_connect(&TopRight){
                elm.push(line(m,u));
            }
            //   +
            //    \
            if bottom_right.can_pass_medium_connect(&TopLeft){
                elm.push(line(m,y));
            }
        }


        //////////////////////////////////////////
        //
        //   \ /      \ /
        //    x        X
        //   / \      / \
        //
        ///////////////////////////////////////////
        if self.any("xX"){
            //    \
            //     x
            if top_left.can_pass_medium_connect(&BottomRight){
               elm.push(line(a,m)); 
            }
            //    /
            //   x
            if top_right.can_pass_medium_connect(&BottomLeft){
               elm.push(line(m,e)); 
            }
            //    x
            //   /
            if bottom_left.can_pass_medium_connect(&TopRight){
                elm.push(line(m,u))
            }
            //   x
            //    \
            if bottom_right.can_pass_medium_connect(&TopLeft){
                elm.push(line(m,y));
            }
        }

        /////////////////////////////////////////
        //
        //   -┬- -┭- -┮- -┯- -┰- -┱- -┲- -┳-
        //    |   |   |   |   |   |   |   |
        //
        /////////////////////////////////////////
        if self.is_static_bottom_intersection(){
            elm.extend(vec![
                line(k,o),
                line(m,w)
            ]);
        }


        //////////////////////////////////////////
        //
        //  |    |    |    |    |    |    |    |
        // -┴-  -┵-  -┶-  -┷-  -┸-  -┹-  -┺-  -┻-
        //
        /////////////////////////////////////////
        if self.is_static_top_intersection(){
            elm.extend(vec![
                line(k,o),
                line(c,m)
            ]);
        }


        //////////////////////////////////////////
        //
        //  |    |   |    |   |    |    |    |
        //  ├-   ┝-  ┞-   ┟-  ┠-   ┡-   ┢-   ┣-
        //  |    |   |    |   |    |    |    |
        //
        ////////////////////////////////////////////
        if self.is_static_right_intersection(){
            elm.extend(vec![
                line(c,w),
                line(m,o)
            ]);
        }
        //  |    |   |   |   |   |   |   |
        // -┤   -┥  -┦  -┧  -┨  -┩  -┪  -┫
        //  |    |   |   |   |   |   |   |
        if self.is_static_left_intersection(){
            elm.extend(vec![
                line(c,w),
                line(m,k)
            ]);
        }

        ///////////////////////////////
        //    
        //     o
        // small-o circle
        //
        ///////////////////////////////
        if self.is('o'){
            let mut connects = false;
            //    |   |   +  ┌
            //    o   o   o  |
            if top.can_pass_medium_connect(&Bottom){
                elm.push(line(h,c));
                connects = true;
            }
            //    o   o   o  |
            //    |   |   +  ┘
            if bottom.can_pass_medium_connect(&Top){
                elm.push(line(r,w));
                connects = true;
            }
            //     o-  o- o+  o┘
            if right.can_pass_medium_connect(&Left){
                //elm.push(line(n,o));
                connects = true;
            }
            //    -o 
            if left.can_pass_medium_connect(&Right){
                //elm.push(line(l,k));
                connects = true;
            }
            //   \   
            //    o   
            if top_left.can_pass_medium_connect(&BottomRight){
                elm.push(line(a,g));
                connects = true;
            }
            //     /  
            //    o  
            if top_right.can_pass_medium_connect(&BottomLeft){
                elm.push(line(i,e));
                connects = true;
            }
            //     o  
            //    /   
            if bottom_left.can_pass_medium_connect(&TopRight){
                elm.push(line(q,u));
                connects = true;
            }
            //     o  
            //      \  
            if bottom_right.can_pass_medium_connect(&TopLeft){
                elm.push(line(s,y));
                connects = true;
            }
            if connects{
                    elm.push(open_circle(m,tw2));
            }
        }
        ///////////////////////////////
        //
        //    O
        // big-O circle
        // 
        /////////////////////////////////
        if self.is('O'){
            let mut connects = false;
            //    |   |   +  ┌
            //    O   O   O  |
            if top.can_pass_medium_connect(&Bottom){
                elm.push(line(c, &c.add_y(tw1)));
                connects = true;
            }
            //    O   O   O  |
            //    |   |   +  ┘
            if bottom.can_pass_medium_connect(&Top){
                elm.push(line(w, &w.add_y(-tw1)));
                connects = true;
            }
            //     O-  O- O+  O┘
            if right.can_pass_medium_connect(&Left){
                elm.push(line(&right.l(), &right.o()));
                consumed.push(right.loc());
                connects = true;
            }
            //    -O 
            if left.can_pass_medium_connect(&Right){
                elm.push(line(&left.n(), &left.k()));
                consumed.push(left.loc());
                connects = true;
            }
            //   \   
            //    O   
            if top_left.can_pass_medium_connect(&BottomRight){
                elm.push(line(a, &a.add(tw1/2.0,tw1)));
                connects = true;
            }
            //     /  
            //    O  
            if top_right.can_pass_medium_connect(&BottomLeft){
                elm.push(line(e, &e.add(-tw1/2.0,tw1)));
                connects = true;
            }
            //     O  
            //    /   
            if bottom_left.can_pass_medium_connect(&TopRight){
                elm.push(line(u, &u.add(tw1/2.0, -tw1)));
                connects = true;
            }
            //     O  
            //      \  
            if bottom_right.can_pass_medium_connect(&TopLeft){
                elm.push(line(y, &y.add(-tw1/2.0, -tw1)));
                connects = true;
            }
            if connects{
               elm.push(open_circle(m,tw3));
            }
        }
        ///////////////////////////////
        //    
        //     *  solid marker
        //
        ///////////////////////////////
        if self.is_solid_round_marker(){
            let mut connects = false;
            //    |   +  ┌
            //    *   *  *
            if top.can_pass_medium_connect(&Bottom){
                elm.push(line(m,c));
                connects = true;
            }
            //    *   *  *
            //    |   +  ┘
            if bottom.can_pass_medium_connect(&Top){
                elm.push(line(m,w));
                connects = true;
            }
            //     *-  *+  *┘
            if right.can_pass_medium_connect(&Left){
               elm.push(line(m,o));
                connects = true;
            }
            //    -* 
            if left.can_pass_medium_connect(&Right){
                elm.push(line(m,k));
                connects = true;
            }
            //   \   
            //    *   
            if top_left.can_pass_medium_connect(&BottomRight){
                elm.push(line(a,m));
                connects = true;
            }
            //     /  
            //    *  
            if top_right.can_pass_medium_connect(&BottomLeft){
                elm.push(line(e,m));
                connects = true;
            }
            //     *  
            //    /   
            if bottom_left.can_pass_medium_connect(&TopRight){
                elm.push(line(m,u));
                connects = true;
            }
            //     *  
            //      \  
            if bottom_right.can_pass_medium_connect(&TopLeft){
                elm.push(line(m,y));
                connects = true;
            }
            if connects{
                elm.push(solid_circle(m,tw2));
            }
        }
        if self.is('^'){
            //  ^
            //  |
            if bottom.is_vertical(){
                elm.push(arrow_line(w,h));
            }
            //  |
            //  ^
            if top.is_vertical(){
                elm.push(arrow_line(h,c));
            }
            //   ^
            //  /
            if bottom_left.can_pass_medium_connect(&TopRight){
                elm.push(arrow_line(u,m));
            }
            //   ^
            //    \
            if bottom_right.can_pass_medium_connect(&TopLeft){
                elm.push(arrow_line(y,m));
            }
        }
        if self.is('v') || self.is('V'){
            //   |    |
            //   v    V
            if top.is_vertical(){
                elm.push(arrow_line(c,r));
            }
            //    \ /
            //     v
            else if top_left.can_pass_medium_connect(&BottomRight) && top_right.can_pass_medium_connect(&BottomLeft){
                elm.extend(vec![line(a,m),line(m,e)]);
            }
            //   \    \
            //    v    V
            else if top_left.can_pass_medium_connect(&BottomRight){
                elm.push(arrow_line(a,m));
            }
            //     /   /
            //    v   V
            else if top_right.can_pass_medium_connect(&BottomLeft){
                elm.push(arrow_line(e,m));
            }
        }
        ///////////////////////////////////
        //
        //     (
        //      `>
        //
        ///////////////////////////////////
        if self.is('>')
            && left.is('`')
            && top_left_left.is('('){
            elm.push(
                arrow_arc(k,t,th4)
            )
        }
        ////////////////////////////////////
        //
        //        )
        //      <'
        //
        //////////////////////////////////
        if self.is('<') 
            &&right.is_rounded_bottom_right()
            && top_right_right.is(')'){
            elm.push(
                arrow_sweep_arc(o,p,th4)
            );
        }
        /////////////////////////////////
        //
        //     .>
        //    (
        //
        //////////////////////////////////
        if self.is('>')
            && left.is('.')
            && bottom_left_left.is('('){
            elm.push(
                arrow_sweep_arc(k,j,th4)
            );
        }

        /////////////////////////////////
        //
        //    <.
        //      )
        //
        //////////////////////////////////
        if self.is('<')
            && right.is('.')
            && bottom_right_right.is(')'){
            elm.push(
                arrow_arc(o,f,th4)
            );
        }
        /////////////////////////////////
        //  Big Arcs
        //        _
        //      ,' 
        //     /
        //    |
        //
        //  Issue 3: This needs further architecting
        //  drawing elements will be retrieve
        //  by their location (x,y)
        //  However other drawing patterns
        //  can push drawing elements for locations
        //  other than currently processed
        /////////////////////////////////

        //    /
        //  <'
        if self.is('\'')
            && left.is('<')
            && top_right.is_slant_right(){
            elm.push(arrow_line(e, &left.e()));
        }
        if self.is('<')
            && right.is('\'')
            && top_right_right.is_slant_right(){
            elm.push(blank_text(loc))
        }

        //////////////////////////////////
        //
        //      <
        //
        ///////////////////////////////////
        if self.is('<'){
            //   <-
            //
            if right.is_horizontal() {
                elm.push(arrow_line(o,l));
            }
            //  <<
            if right.is('<') || left.is('<'){
                elm.push(arrow_line(o,l));
            }
            //  /
            // <
            //  \
            if top_right.can_pass_medium_connect(&BottomLeft) 
                && bottom_right.can_pass_medium_connect(&TopLeft){
                elm.extend(vec![
                    line(e,m),
                    line(m,y)
                ]);
            }
            //   )
            //  <
            //   )
            if top_right.is(')') 
                && bottom_right.is(')'){
                elm.extend(vec![
                    line(k,i),
                    line(k,s)
                ]);
            }
            //  |
            // < 
            //  |
            if top_right.is_vertical() && bottom_right.is_vertical(){
                elm.extend(vec![
                    line(k,j),
                    line(k,t),
                    arc(j,&right.c(), tw4),
                    arc(&right.w(),t, tw4),
                ]);
            }
            // -<
            if left.is('-'){
                elm.push(arrow_line(l,k));
            }
        }
        if self.is('>'){
            //   ->
            //
            if left.is_horizontal(){
                elm.push(arrow_line(k,n));
            }
            // >>
            if left.is('>') || right.is('>'){
                elm.push(arrow_line(k,n));
            }
            // `>
            if left.is('`') && !self.in_left(2).top().is('('){
                elm.push(arrow_line(&self.in_left(2).c(),&left.t()));
                consumed.push(left.loc());
            }
            let mut deformed = false;
            //    \
            //     >
            //    /
            if top_left.can_pass_medium_connect(&BottomRight) && bottom_left.can_pass_medium_connect(&TopRight){
                elm.extend(vec![line(a,m),line(m,u),line(m,o)]);
                deformed = true;
            }
            //  (
            //   >
            //  (
            if top_left.is('(')
                && bottom_left.is('('){
                elm.extend(vec![
                    line(o,g),
                    line(o,q)
                ])
            }
            //  |
            //   > 
            //  |
            if top_left.is_vertical() && bottom_left.is_vertical(){
                elm.extend(vec![line(o,f),line(o,p)]);
                elm.extend(vec![arc(&left.c(), f, tw4)]);
                elm.extend(vec![arc(p, &left.w(), tw4)]);
            }
            //  >-
            //  not a priority alg, if the > is deformed, don't execute self
            if !deformed && right.can_pass_medium_connect(&Left){
                elm.push(arrow_line(n,o));
            }
        }
        if self.is('('){
            //    /      (
            //   (        \
            if top_right.can_pass_medium_connect(&BottomLeft) || bottom_right.can_pass_medium_connect(&TopLeft){
                elm.push(arc(e,y,th4));
            }
            //   .  , _   (   (   (_
            //  (  ( (     `   '   
            if (top_right.is('.') || top_right.is(',') || top_right.is('_'))
                || (bottom_right.is('`') || bottom_right.is('\'') || right.is('_')){
                elm.push(arc(e,y,th4));
            }
            //  |
            //  (
            //  |
            if top.is_vertical() && bottom.is_vertical(){
                elm.push(arc(c,w,tw4));
            }
            //  -)-
            if left.is_horizontal() && right.is_horizontal(){
                elm.push(line(k,o));
            }
            if bottom_right.is('>'){
                //  .
                //  (
                //   >
                if top.is('.'){
                    elm.push(line(c, &top.r()));
                }
                //
                //  (
                //   >
                elm.extend(vec![
                    arc(r, &bottom_right.g(),th4),
                    line(r,c)
                ]);
            }
            if top_right.is('>'){
                //   >
                //  (
                //  '
                if bottom.is('\''){
                    elm.push(line(w, &bottom.h()));
                }
                //   >
                //  (
                elm.extend(vec![
                    arc(&top_right.q(), h, th4),
                    line(h,w)
                ])
            }
        }
        if self.is(')'){ 
            //   \        )
            //    )      / 
            if top_left.can_pass_medium_connect(&BottomRight) || bottom_left.can_pass_medium_connect(&TopRight){
                elm.push(arc(u,a,th4));
            }
            //   .  _    )  _)
            //    )  )  '   
            if (top_left.is('.') || top_left.is('_')) 
                || (bottom_left.is('\'') || left.is('_')){
                elm.push(arc(u,a,th4));
            }
            //   |
            //  -)-
            //   |
            if top.is_vertical() && bottom.is_vertical()
                && left.is_horizontal() && right.is_horizontal(){
                elm.extend(vec![arc(w,c,tw4),line(k,o)]);
            }
            if bottom_left.is('<'){
                //     .
                //     )
                //    <
                if top.is('.'){
                    elm.push(line(c, &top.r()));
                }
                //      
                //     )
                //    <
                elm.extend(vec![
                    arc(&bottom_left.i(), r, th4),
                    line(r,c)
                ]);
            }
            if top_left.is('<'){
                //
                //   <
                //    )
                //    '
                if bottom.is('\''){
                    elm.push(line(w, &bottom.h()));
                }
                //
                //    <
                //     )
                //      
                elm.extend(vec![
                 arc(h, &top_left.s(), th4),
                 line(h,w)
                ]);
            }
        }
        if self.is('['){
            let mut interacted = false;
            //               _
            // []  _[  [_   [
            if right.is(']')
                || left.is_low_horizontal()
                || right.is_low_horizontal()
                || top_right.is_low_horizontal()
                {
                interacted = true;
            }
            // [
            if interacted{
                elm.extend(vec![line(e,c),line(c,w),line(w,y)]);
            }
        }
        if self.is(']'){
            let mut interacted = false;
            //             _
            // []  _]  ]_   ]
            if left.is('[')
                || left.is_low_horizontal()
                || right.is_low_horizontal()
                || top_left.is_low_horizontal()
                {
                interacted = true;
            }
            if interacted{
                elm.extend(vec![line(a,c),line(c,w),line(w,u)]);
            }
        }
        if self.is('~'){
            //   |
            //  -~-
            //   |
            if left.is_horizontal() && right.is_horizontal()
                && top.is_vertical() && bottom.is_vertical(){
                elm.extend(vec![arc(o,k,tw2),line(c,w)]);
            }
        }
        if self.is_vertical(){
            let mut trimmed = false;
            //   _
            //  |
            if top_right.is_low_horizontal(){
               elm.push(line(e,c));
            }
            //    _
            //     |
            if top_left.is_low_horizontal(){
                elm.push(line(a,c));
            }
            //  |_
            if right.is_low_horizontal(){
                elm.push(line(w,y));
            }
            //    _|
            if left.is_low_horizontal(){
                elm.push(line(w,u));
            }
            //     |-
            if right.is_horizontal(){
                elm.push(line(m,o));
            }
            //     -|
            if left.is_horizontal(){
                elm.push(line(k,m));
            }
            //     \
            //      |
            if top_left.can_pass_medium_connect(&BottomRight){
                elm.extend(vec![line(a,m),line(m,w)]);
                trimmed = true;
            }
            //       /
            //      |
            if top_right.can_pass_medium_connect(&BottomLeft){
                elm.extend(vec![line(e,m),line(m,w)]);
                trimmed = true;
                //    |/
                //    |
                if top.is_vertical(){
                    trimmed = false;
                }
            }
            //       |
            //      /
            if bottom_left.can_pass_medium_connect(&TopRight){
                elm.extend(vec![line(u,m),line(c,m)]);
                trimmed = true;
            }
            //     |
            //      \
            if bottom_right.can_pass_medium_connect(&TopLeft){
                elm.extend(vec![line(m,y),line(c,m)]);
                trimmed = true;
            }
            //   /|    |\ 
            if left.is_slant_right() || right.is_slant_left(){
                elm.push(line(c,&top.m()));
            }
            //   \|    |/
            if left.is_slant_left() || right.is_slant_right(){
                elm.push(line(w,&bottom.m()));
            }
            // add the vertical line only if NOT trimmed
            if !trimmed{
                elm.push(line(c,w));
            }
        }
        ///////////////////////
        // horizontal dash
        //    -
        //
        ////////////////////////
        if self.is_horizontal(){
            let mut deformed = false;
            let mut interacted = false;
            //    |
            //   .-.
            //    |
            if left.is('.') && right.is('.')
                && top.is_vertical() && bottom.is_vertical(){
                elm.extend(vec![arc(o,k,th4), line(c,w)]); 
                deformed = true;
                interacted = true;
            }
            //  --   +-
            if left.can_pass_weakly_connect(&Right){
                elm.push(line(m,k));
                deformed = false;
                interacted = true;
            }
            // --    -+
            if right.can_pass_weakly_connect(&Left){
                elm.push(line(m,o));
                deformed = false;
                interacted = true;
            }
            //   -\   -/
            if right.is_slant_left() || right.is_slant_right(){
                elm.push(line(o,&right.m()));
                deformed = false;
                interacted = true;
            }
            //   \-   /-
            if left.is_slant_left() || left.is_slant_right(){
                elm.push(line(k,&left.m()));
                deformed = false;
                interacted = true;
            }
            if interacted{ // if not deformed put as is
                if !deformed{
                    elm.push(line(k,o))
                }
            }
        }
        if self.is_static_horizontal(){
            elm.push(line(k,o));
        }
        ///////////////////////////////////
        //
        //    ╭
        //
        //////////////////////////////////
        if self.is_static_rounded_top_left(){
            elm.extend(vec![arc(o,r,tw2), line(r,w)]); 
        }
        /////////////////////////////////
        //
        //    ╮
        //
        /////////////////////////////////
        if self.is_static_rounded_top_right(){
            elm.extend(vec![arc(r,k,tw2), line(r,w)]);
        }
        //////////////////////////////////
        //
        //   ╰
        //
        //////////////////////////////////
        if self.is_static_rounded_bottom_left(){
            elm.extend(vec![line(c,h),arc(h,o,tw2)]);
        }
        /////////////////////////////////
        //
        //   ╯
        //
        /////////////////////////////////
        if self.is_static_rounded_bottom_right(){
            elm.extend(vec![arc(k,h,tw2),line(h,c)]);
        }
        if self.is('='){
            elm.extend(vec![line(f,j),line(p,t)]);
            //  -=
            if left.is('-'){
                elm.push(line(f,p));
            }
            // =-
            if right.is('-'){
                elm.push(line(j,t));
            }
        }
        if self.is('╬'){
            elm.extend(vec![
                line(k,l), line(n,o),
                line(p,q), line(s,t),
                line(b,l), line(q,v),
                line(d,n), line(s,x)
                ]);
        }
        if self.is('╔'){
            elm.extend(vec![
                line(o,l),line(l,v),
                line(t,s),line(s,x)
            ]);
        }
        if self.is('╗'){
            elm.extend(vec![
                line(k,n),line(n,x),
                line(p,q),line(q,v)
            ]);
        }
        if self.is('╝'){
            elm.extend(vec![
                line(k,l),line(l,b),
                line(p,s),line(s,d)
            ]);
        }
        if self.is('╚'){
            elm.extend(vec![
                line(b,q),line(q,t),
                line(d,n),line(n,o)
            ]);
        }
        if self.is('╠'){
            elm.extend(vec![
                line(b,v),
                line(d,n),line(n,o),
                line(t,s),line(s,x)
            ]);
        }
        if self.is('╣'){
            elm.extend(vec![
                line(d,x),
                line(v,q),
                line(p,q),
                line(k,l),
                line(l,b)
            ]);
        }
        if self.is('╦'){
            elm.extend(vec![
                line(k,o),
                line(p,q),line(q,v),
                line(s,t),line(s,x)
            ]);
        }
        if self.is('╩'){
            elm.extend(vec![
                line(p,t),
                line(k,l),
                line(l,b),
                line(d,n),
                line(n,o)
            ]);
        }

        // double horizontal
        if self.any("═"){
            elm.extend(vec![line(k,o),line(p,t)]);
        }
        // double vertical
        if self.is('║'){
            elm.extend(vec![line(b,v),line(d,x)]);
        }
        // combi
        if self.is('╒'){
            elm.extend(vec![
                line(m,w),
                line(m,o),
                line(r,t)
            ]);
        }
        if self.is('╓'){
            elm.extend(vec![
                line(l,o),
                line(l,v),
                line(n,x)
            ]);
        }
        if self.is('╞'){
            elm.extend(vec![
                line(c,w),
                line(m,o),
                line(r,t)
            ]);
        }
        if self.is('╡'){
            elm.extend(vec![
                line(c,w),
                line(k,m),
                line(p,r)
            ]);
        }
        if self.is('╤'){
            elm.extend(vec![
                line(k,o),
                line(p,t),
                line(r,w)
            ]);
        }
        if self.is('╥'){
            elm.extend(vec![
                line(k,o),
                line(l,v),
                line(n,x)
            ]);
        }
        if self.is('╖'){
            elm.extend(vec![
                line(k,n),
                line(n,x),
                line(l,v)
            ]);
        }
        if self.is('╙'){
            elm.extend(vec![
                line(l,o),
                line(l,b),
                line(n,d)
            ]);
        }
        if self.is('╜'){
            elm.extend(vec![
                line(k,n),
                line(l,b),
                line(n,d)
            ]);
        }
        if self.is('╕'){
            elm.extend(vec![
                line(m,w),
                line(k,m),
                line(p,r)
            ]);
        }
        if self.is('╛'){
            elm.extend(vec![
                line(c,r),
                line(r,p),
                line(k,m)
            ]);
        }
        if self.is('╘'){
            elm.extend(vec![
                line(c,r),
                line(m,o),
                line(r,t)
            ]);
        }
        if self.is('╢'){
            elm.extend(vec![
                line(d,x),
                line(b,v),
                line(k,l)
            ]);
        }
        if self.is('╟'){
            elm.extend(vec![
                line(d,x),
                line(b,v),
                line(n,o)
            ]);
        }
        if self.is('╪'){
            elm.extend(vec![
                line(c,w),
                line(k,o),
                line(p,t)
            ]);
        }
        if self.is('╧'){
            elm.extend(vec![
                line(k,o),
                line(p,t),
                line(c,m)
            ]);
        }
        if self.is('╫'){
            elm.extend(vec![
                line(k,o),
                line(b,v),
                line(d,x)
            ]);
        }
        if self.is('╨'){
            elm.extend(vec![
                line(k,o),
                line(b,l),
                line(d,n)
            ]);
        }
        ///////////////////////////////
        //
        //        \
        //
        ///////////////////////////////
        if self.is_slant_left(){
            let mut trimmed = false;
            let mut interacted = false;
            //     \      /   |     
            //      \     \    \     \   
            //       \                |  
            if top.can_pass_weakly_connect(&BottomLeft)
                || top_left.can_pass_weakly_connect(&Bottom)
                || top_left.can_pass_weakly_connect(&BottomRight)

                || bottom_right.can_pass_weakly_connect(&Top)
                || bottom_right.can_pass_weakly_connect(&TopLeft)
                {
                interacted = true;
            }
            //   /
            //   \    \
            //        /
            if top.is_slant_right()
                || bottom.is_slant_right(){
                interacted = true;
            }
            //   \_
            if right.is_low_horizontal(){
                interacted = true;
            }
            //   _
            //   \
            if top.is_low_horizontal(){
                interacted = true;
            }
            //  _
            //   \
            if top_left.is_low_horizontal(){
                interacted = true;
            }
            //  _\
            if left.is_low_horizontal(){
                interacted = true;
            }
            //    /\    \/
            if left.is_slant_right()
                || right.is_slant_right(){
                interacted = true;
            }

            //     _____\
            if left.is_low_horizontal(){
                elm.push(line(u,y));
                interacted = true;
            }
            //   |
            //   \
            if top.is_vertical(){
                elm.extend(vec![line(c,m),line(m,y)]);
                trimmed = true;
                interacted = true;
            }
            //   \
            //   |
            if bottom.is_vertical(){
                elm.extend(vec![line(a,m),line(m,w)]);
                trimmed = true;
                interacted = true;
            }
            //  |\
            if left.is_vertical(){
                elm.push(line(a,&top_left.m()));
                interacted = true;
            }
            //    \|
            if right.is_vertical(){
                elm.push(line(y,&bottom_right.m()));
                interacted = true;
            }
            if interacted && !trimmed{
                elm.push(line(a,y));
            }
        }
        //////////////////////////
        //
        //       /
        //
        /////////////////////////
        if self.is_slant_right(){
            let mut trimmed = false;
            let mut interacted = false;
            //     /                  |    _
            //    /     /    /       /    /
            //   /      \   |
            if top_left.can_pass_weakly_connect(&BottomRight)
                || top_right.can_pass_weakly_connect(&Bottom)
                || top_right.can_pass_weakly_connect(&BottomLeft)

                || bottom_left.can_pass_weakly_connect(&TopRight)
                || bottom_right.can_pass_weakly_connect(&Top)
                {
                interacted = true;
            }
            //   \   
            //   /   /
            //       \
            if top.is_slant_left()
                || bottom.is_slant_left(){
                interacted = true;
            }
            //     _
            //    /
            if top_right.is_low_horizontal(){
                interacted = true;
            }
            //  _/
            if left.is_low_horizontal(){
                interacted = true;
            }
            //   _
            //   /
            if top.is_low_horizontal(){
                interacted = true;
            }
            //   \/    /\
            if left.is_slant_left()
                || right.is_slant_left(){
                interacted = true;
            }
            //    /___
            if right.is_low_horizontal(){
                elm.push(line(u,y));
                interacted = true;
            }
            //   |
            //   /
            if top.is_vertical(){
                elm.extend(vec![line(c,m), line(m,u)]);
                trimmed = true;
                interacted = true;
            }
            //    /
            //    |
            if bottom.is_vertical(){
                elm.extend(vec![line(e,m),line(m,w)]);
                trimmed = true;
                interacted = true;
            }
            //   |/
            if left.is_vertical(){
                elm.push(line(u, &bottom_left.m()));
                interacted = true;
            }
            //    /|
            if right.is_vertical(){
                elm.push(line(e,&top_right.m()));
                interacted = true;
            }
            if interacted && !trimmed{
                elm.push(line(e,u));
            }
        }
        if self.is_low_horizontal(){
            let mut interacted = false;
            // ___
            if left.is_low_horizontal()
                || right.is_low_horizontal(){
                interacted = true;
            }
            //  _[
            if right.is('['){
                elm.push(line(y,&right.w()));
                interacted = true;
            }
            //  ]_
            if left.is(']'){
                elm.push(line(u, &left.w()));
            }
            //  (_  _)
            if left.is('(')
                || right.is(')'){
                interacted = true;
            }
            //   _  _
            //  (    )
            if bottom_left.is('(')
                || bottom_right.is('('){
                interacted = true;
            }

            //  |_  _|
            if left.is_vertical()
                || right.is_vertical(){
                interacted = true;
            }
            //   _    _
            //  |      |
            if bottom_left.is_vertical()
                || bottom_right.is_vertical()
                {
                interacted = true;
            }
            //  _/   _\  /_  \_
            if right.is_slant_right()
                || right.is_slant_left()
                || left.is_slant_right()
                || left.is_slant_left()
                {
                interacted = true;
            }
            //   _   _
            //  /     \
            if bottom_left.is_slant_right()
                || bottom_right.is_slant_left(){
                interacted = true;
            }
            //  ._  _.
            if left.is('.') || right.is('.'){
                interacted = true;
            }
            // _,
            if right.is(','){
                interacted = true;
            }
            //     _
            //      `
            if bottom_right.is('`'){
                interacted = true;
            }
            //       _
            //      '
            if bottom_left.is('\''){
                interacted = true;
            }
            if interacted{
                elm.push(line(u,y));
            }
        }
        ////////////////////////////////////
        //
        //   ...   // dot dot dot
        //
        ////////////////////////////////////
        if self.is('.'){
            if left.is('.')
                && right.is('.'){
                elm.push(line(p,t));
            }
            if left.is('.')
                && self.in_left(2).is('.'){
                elm.push(line(p,r));
            }
            if right.is('.')
                && self.in_right(2).is('.'){
                elm.push(line(r,t));
            }
        }

        /////////////////////////////////////
        //                      
        //  . ,   `.     .'   `,      ,`
        //          `   '       `    `
        /////////////////////////////////////
        if self.any(".,"){
            //  `.
            //    `
            if left.any("`\'")
                && bottom_right.any("`\'"){
                elm.extend(vec![
                    line(m,f),
                    line(m,t)
                ]);
            }
            //   .
            //    `.  connect edge to bottom_right
            if bottom_right.any("`\'")
                && bottom_right_right.any(".,"){
                elm.extend(vec![
                    line(m,t)
                ]);
            }
            //   .  connect edge to top_left
            //    `.
            if left.any("`\'")
                && top_left_left.any(".,"){
                elm.extend(vec![
                   line(m,f)
                ]);
            }
            //   .'    .`
            //  '     '
            if bottom_left.any("`\'")
                && right.any("`\'")
            {
                elm.extend(vec![
                    line(m,j),
                    line(m,p),
                ]);
            }
            //    .   . extend to bottom_left
            //  .'  ,'
            if  bottom_left.any("\'`")
                && bottom_left_left.any(".,"){
                elm.extend(vec![
                    line(m,p)
                ]);
            }
            //     .   , extend to top_right
            //   .'  .'
            if right.any("\'`")
                && top_right_right.any(".,"){
                elm.extend(vec![
                    line(m,j)
                ]);
            }

            //     .   ,
            //      `.' 
            if left.any("`\'")
                && top_left_left.any(".,")
                && right.any("`\'")
                && top_right_right.any(".,")
                {
                elm.extend(vec![
                    line(f,m),
                    line(m,j)
                ]);
            }
            //    .
            //  .' `.
            //  
            if bottom_left.any("\'`")
                && bottom_left_left.any(".,")
                && bottom_right.any("`\'")
                && bottom_right_right.any(".,")
                {
                elm.extend(vec![
                    line(m,p),
                    line(m,t)
                ]);
            }
            //     ,
            //   .'
            //    `.
            if right.any("\'`")
                && top_right_right.any(".,")
                && bottom_right.any("`\'")
                && bottom_right_right.any(".,")
                {
                elm.extend(vec![
                    line(j,m),
                    line(m,t)
                ]);
            }
            //   .
            //    `.
            //   ,'
            //
            if left.any("`\'")
                && top_left_left.any(".,")
                && bottom_left.any("\'`")
                && bottom_left_left.any(".,")
                {
                elm.extend(vec![
                    line(m,f),
                    line(m,p)
                ]);
            }
        }
        /////////////////////////////
        //
        //  pointed speech pointer to the bottom_left
        //      ,    
        //    /'    
        //
        /////////////////////////////
        if self.is('\'')
            && top_right.is(',')
            && left.is_slant_right()
            {
            elm.extend(vec![
                line(c,&left.m())
            ]);
        }
        ////////////////////////////////
        //
        //  pointed speech pointer to the bottom_right
        //    .        
        //     `\      
        //
        ////////////////////////////////
        if self.is('`')
            && top_left.is('.')
            && right.is_slant_left()
            {
            elm.push(line(c,&right.m()));
        }
        /////////////////////////////
        //        
        //   `  '    `.    .`  `,    ,'
        //             `  `      `  '
        //
        //////////////////////////////
        if self.any("\'`"){
            //     ,     ,     .
            //   ,'    .'    ,'
            if left.any(",.") 
                && top_right.any(",."){
                elm.extend(vec![
                    line(c,&top_right.p()),
                    line(c,f),
                    line(f,&left.m()),
                ]);
            }
            //    ,'      .'
            //   '       '
            //   extend to top_right 
            if top_right.any(",.")
                && top_right_right.any("\'`"){
                elm.extend(vec![
                    line(c,&top_right.p()),
                ]);
            }
            //    ,'     .'
            //   '      '
            //  extend to bottom_left 
            if left.any(",.")
                && bottom_left_left.any("\'`"){
                elm.extend(vec![
                    line(c,f)
                ]);
            }
            //  .      .
            //   `.     `,
            if top_left.any(".,")
                && right.any(".,"){
                elm.extend(vec![
                    line(c,j),
                    line(j,&right.m()),
                    line(c,&top_left.t())
                ]);
            }
            //  `.   extend to bottom_right
            //    `
            if right.any(".,")
                && bottom_right_right.any("`\'"){
                elm.extend(vec![
                    line(c,j),
                    line(j,&right.m()),
                ]);;
            }
            //  `.  extend to top_left
            //    `
            if top_left.any(".,")
                && top_left_left.any("`\'"){
                elm.extend(vec![
                    line(c,&top_left.t())
                ]);
            }

            //    
            //    `. ,'
            //      ` 
            if top_left.any(".,")
                && top_left_left.any("`\'")
                && top_right.any(".,")
                && top_right_right.any("`\'")
                {
                elm.extend(vec![
                    line(c,&top_right.p()),
                    line(c,&top_left.t())
                ]);
            }

            //     ,`.      .`.
            //    '   `    `   '
            if left.any(",.")
                && bottom_left_left.any("`\'")
                && right.any(".,")
                && bottom_right_right.any("`\'")
                {
                elm.extend(vec![
                    line(c,f),
                    line(c,j)
                ]);
            }
        }


        // if no element is formulated, then treat it as literal string
        if elm.len() < 1 && !self.is_blank(){
            let quoted = ::escape_char(&self.text());
            elm.push(text(loc,quoted));
        }
        (elm, consumed)
    }
    */


    


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

    #[test]
    fn test_adjascent(){


            let g = Grid::from_str("a统öo͡͡͡", &Settings::compact());
            let fc = FocusChar::new(&Loc::new(1,0), &g);
            println!("{:?}", fc);
            assert!(fc.left().is('a'));
            assert!(fc.right().right().is('ö'));
    }
}
