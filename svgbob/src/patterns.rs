use Element;
use Point;
use Loc;
use Grid;

use self::Direction::{
    Top,Bottom,
    Left,Right,
    TopLeft,TopRight,
    BottomLeft,BottomRight
};
use self::Behavior::{Static,Dynamic};



fn line(a:&Point, b:&Point) -> Element {
    Element::solid_line(a,b)
}

fn arrow_line(a: &Point, b: &Point) -> Element {
    Element::arrow_line(a,b)
}

fn arrow_arc(a: &Point, b: &Point, r: f32) -> Element{
    Element::arrow_arc(a, b, r, false)
}

fn arrow_sweep_arc(a: &Point, b: &Point, r: f32) -> Element {
    Element::arrow_arc(a, b, r, true)
}


fn arc(a: &Point, b: &Point, r: f32) -> Element{
    Element::arc(a, b, r, false)
}


fn open_circle(c:&Point, r:f32) -> Element {
    Element::open_circle(c, r)
}

fn solid_circle(c: &Point, r: f32) -> Element {
    Element::solid_circle(c, r)
}


fn text(loc: &Loc, txt:String) -> Element {
    Element::Text(loc.clone(), txt)
}

fn blank_text(loc: &Loc) -> Element {
    text(loc," ".into())
}


/// whether or not characters react to neighoring character
#[derive(PartialEq)]
enum Behavior{
    Static,  //stable
    Dynamic  //reactive
}



/// 8 directions which a character can connect to
///   \|/
///   -+-
///   /|\
#[derive(PartialEq)]
enum Direction{
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

impl Direction{
    
    fn all() -> Vec<Self> {
        vec![
            Top,
            Bottom,
            Left,
            Right,
            TopLeft,
            TopRight,
            BottomLeft,
            BottomRight
        ]
    }
}

#[derive(Debug)]
pub struct FocusChar<'g>{
    pub loc: Loc,
    pub grid: &'g Grid
}


impl <'g>FocusChar<'g>{

    /// get the text of self char, including complex block
    /// concatenated with multiple strings in utf8 encoding
    fn text(&self) -> String {
        self.grid.get_string(&self.loc)
    }

    /// get the focus char at this location
    fn get(&self, loc: &Loc) -> Self{
        Self{
            loc: loc.clone(),
            grid: self.grid
        }
    }


    fn get_char(&self) -> char {
        self.text().chars().nth(0).unwrap_or('\0')
    }

    /// if the character matches given argument
    fn is(&self, ch: char) -> bool {
        self.get_char() == ch
    }

    /// if character is any character in the string
    fn any(&self, ch: &str) -> bool {
        ch.contains(self.get_char())
    }

    fn in_any(&self, chars: Vec<char>) -> bool {
        chars.contains(&self.get_char())
    }

    /// enumerate the direction self character can connect to
    fn can_connect_to_direction(&self) -> (Behavior, Vec<Direction>) {
        if self.is('+'){
            (Static, vec![Top,Bottom, Left, Right])
        }
        //   \|/
        //   -*-
        //   /|\
        else if self.any(".*oO"){
            (Dynamic, Direction::all())
        }
        //       ,-     ,      -,  
        //       |     /       /   
        else if self.is(','){
            (Dynamic, vec![Left,Right,BottomLeft,BottomRight])
        }
        //   |      \
        //   `-      `
        else if self.is('`'){
            (Dynamic, vec![Top,Right,TopRight])
        }
        //   |      |   \       /
        //   '-    -'    '-   -'
        else if self.is('\''){
            (Dynamic, vec![Top,Left, Right, TopLeft, TopRight])
        }
        else if self.in_any(vec!['-','─','━']){
            (Static, vec![Left,Right])
        }
        else if self.any("|│┃"){
            (Static, vec![Top,Bottom])
        }
        else if self.any("\\"){
            (Static, vec![TopLeft, BottomRight])
        }
        else if self.any("/"){
            (Static, vec![TopRight, BottomLeft])
        }
        else if self.any("╭┌┍┎┏"){
            (Static, vec![Bottom,Right])
        }
        else if self.any("╮┐┑┒┓"){
            (Static, vec![Bottom,Left])
        }
        else if self.any("╰┗└┕┖"){
            (Static, vec![Top,Right])
        }
        else if self.any("╯┘┙┚┛"){
            (Static, vec![Top,Left])
        }
        else if self.any("┼┽┾┿╀╁╂╃╄╅╆╇╈╉╊╋"){
            (Static, vec![Top,Bottom,Left,Right])
        }
        else if self.any("┬┭┮┯┰┱┲┳"){
            (Static, vec![Bottom, Left, Right])
        }
        else if self.any("┴┵┶┷┸┹┺┻"){
            (Static, vec![Top, Left, Right])
        }
        else if self.any("├┝┞┟┠┡┢┣"){
            (Static, vec![Top,Bottom,Right])
        }
        else if self.any("┤┥┦┧┨┩┪┫"){
            (Static, vec![Top,Bottom,Left])
        }
        else if self.any("╒"){
            (Static, vec![Bottom])
        }
        else if self.any("╓╙╟"){
            (Static, vec![Right])
        }
        else if self.any("╡╪╞"){
            (Static, vec![Top,Bottom])
        }
        else if self.any("╕╤"){
            (Static, vec![Bottom])
        }
        else if self.any("╥╫"){
            (Static, vec![Left, Right])
        }
        else if self.any("╖╜╢"){
            (Static, vec![Left])
        }
        else if self.any("╛╘"){
            (Static, vec![Top])
        }
        //    ^     ^    ^
        //    |    /      \
        else if self.any("^"){
            (Dynamic, vec![Bottom, BottomLeft, BottomRight])
        }
        //    |  |    \    /
        //    v  V     V  V
        else if self.any("vV"){
            (Dynamic, vec![Top, TopLeft, TopRight])
        }
        //   <-  ~-  '-
        else if self.any("<~'"){
            (Dynamic, vec![Left])
        }
        //   ->   -~  -'
        else if self.any("'~>"){
            (Dynamic, vec![Right])
        }
        //    /
        //   (
        //    \
        else if self.is('('){
            (Dynamic, vec![TopRight, BottomRight])
        }
        //  \
        //   )
        //  /
        else if self.is(')'){
            (Dynamic, vec![TopLeft, BottomLeft])
        }
        else{
            (Dynamic, vec![])
        }
    }

    fn can_connect_to_with_behavior(&self, behavior: Behavior ) -> Vec<Direction>{
        let (beh, dirs) = self.can_connect_to_direction();
        if beh == behavior{
            dirs
        }
        else{
            vec![]
        }
    }

    /// self character is dynamic and can connect to
    /// self enumerated direction
    fn can_dynamic_connect(&self, dir: &Direction) -> bool{
        self.can_connect_to_with_behavior(Dynamic)
            .contains(dir)
    }

    

    fn can_static_connect(&self, dir: &Direction) -> bool {
        self.can_connect_to_with_behavior(Static)
            .contains(dir)
    }


    fn is_thin_horizontal(&self) -> bool {
        self.in_any(vec!['-','-','─'])
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

    fn is_blank(&self) -> bool {
        self.any("\0")
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
    //  Static + Dynamic connect
    //
    ///////////////////////////////////
    
    fn can_connect(&self, dir: &Direction) -> bool {
        self.can_static_connect(dir)
        || self.can_dynamic_connect(dir)
    }


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
    pub fn get_elements(&self) -> Vec<Element>{
        let loc = &self.loc;
        let top = self.top();
        let bottom = self.bottom();
        let left = self.left();
        let right = self.right();
        // skip blanks and spaces
        if self.is_blank() || self.is(' '){
            return vec![];
        }
        
        let top_left = self.top_left();
        let top_right = self.top_right();
        let bottom_left = self.bottom_left();
        let bottom_right = self.bottom_right();

        let left_left = self.get(&loc.left().left());
        let right_right = self.get(&loc.right().right());
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

        let tw2 = self.tw2();
        let tw4 = self.tw4();

        let th2 = self.th2();
        let th4 = self.th4();

        let mut elm = vec![];
        /////////////////////////////
        //
        //    .    ,     
        //
        /////////////////////////////
        if self.is_dynamic_rounded_top_left(){
            let mut deformed = false;
            //    .    ,  .   .
            //    |    |  +   '
            if bottom.can_static_connect(&Top)
                || bottom.any("+'"){
                elm.push(line(r,w));
            }
            //    .    ,   
            //   (    (   
            if bottom_left.is('('){
                elm.extend(vec![arc(o,u,th4)]);
                deformed = true;
            }
            if right.can_static_connect(&Left){
                //  .-     ,-   
                //   \      \    
                if bottom_right.can_static_connect(&TopLeft){
                    elm.extend(vec![arc(o,s,tw2), line(s,y)]);    
                    deformed = true;
                }
                //     .-     
                //    '       
                if bottom_left.any("\'"){
                    elm.push(line(o,m));
                    deformed = true;
                }
                if bottom_left.can_static_connect(&TopRight) || bottom_left.is('('){
                    //      /     /   
                    //     .-    ,-    
                    //    /     /
                    if top_right.can_static_connect(&BottomLeft){
                        elm.push(line(q,e));
                    }
                    //    .-    ,-    .+
                    //   /     /     /  
                    if bottom_left.can_static_connect(&TopRight){
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
            && left.can_static_connect(&Right)
            && bottom_left.can_static_connect(&TopRight){
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
            if bottom.can_static_connect(&Top){
                elm.push(line(r,w));
            }
            if left.can_static_connect(&Right){
                //    -.    +.
                //    /     /
                if bottom_left.can_static_connect(&TopRight){
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
                if bottom_right.can_static_connect(&TopLeft){
                    //  -.    +.
                    //    \     \
                    elm.extend(vec![arc(s,k,tw4), line(s,y)]);
                    //   \
                    //   -. 
                    //     \
                    if top_left.can_static_connect(&BottomRight){
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
            if bottom_left.can_static_connect(&TopRight) && bottom_right.can_static_connect(&TopLeft){
                elm.extend(vec![line(y,s), arc(s,q,tw2), line(q,u)]);
            }
            //    \
            //     .
            //    /
            if top_left.can_static_connect(&BottomRight) && bottom_left.can_static_connect(&TopRight){
                elm.extend(vec![line(u,q),arc(q,g,th2),line(g,a)]);
            }
            //      /
            //     .
            //      \
            if top_right.can_static_connect(&BottomLeft) && bottom_right.can_static_connect(&TopLeft){
                elm.extend(vec![line(e,i), arc(i,s,th2), line(s,y)]);
            }
            //     \
            //      .
            //      |
            if top_left.can_static_connect(&BottomRight) && bottom.is_vertical(){
                elm.extend(vec![line(w,r),arc(r,g,th4),line(g,a)]);
            }
            //      |
            //      .
            //     /
            if top.is_vertical() && bottom_left.can_static_connect(&TopRight){
                elm.extend(vec![line(u,q),arc(q,h,th4),line(h,c)]);
            }
            //     /
            //    .
            //    |
            if top_right.can_static_connect(&BottomLeft) && bottom.is_vertical(){
                elm.extend(vec![line(e,i),arc(i,r,th4),line(r,w)]);
            }
            //    |
            //    .
            //     \
            if top.is_vertical() && bottom_right.can_static_connect(&TopLeft){
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
            if bottom.can_static_connect(&Top)
                && !right.can_connect(&Left)
                && !left.can_connect(&Right){
                elm.push(line(m,w))
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
            if top_left.can_static_connect(&BottomRight) 
                && top_right.can_static_connect(&BottomLeft){
                elm.extend(vec![
                    line(a,m),
                    line(m,e)
                ]);
            }
        }
        ///////////////////////////
        //
        //    `
        //
        ////////////////////////////
        if self.is('`'){
            // `>
            //  don't do self when not used as counter clock wise arrow
            //
            //  NOT:  (
            //         `>
            if right.is('>') && !top_left.is('('){
                elm.push(arrow_line(&left.c(),t));
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
            if right.can_static_connect(&Left){
                //    /   /   
                //   '-   `-  
                if top_right.can_static_connect(&BottomLeft){
                    elm.extend(vec![line(e,i), arc(i,o,tw2)]);
                    deformed = true;
                }
                if top_left.can_static_connect(&BottomRight){
                    //   \
                    //    '-
                    //     \
                    if bottom_right.can_static_connect(&TopLeft){
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
            if left.can_static_connect(&Right){
                //      \
                //      -'
                if top_left.can_static_connect(&BottomRight){
                    elm.extend(vec![arc(k,g,tw2), line(g,a)]);
                    deformed = true;
                }
                if top_right.can_static_connect(&BottomLeft){
                    //       /
                    //     -'
                    //     /
                    if bottom_left.can_static_connect(&TopRight){
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
            if top.can_connect(&Bottom){
                elm.push(line(c,m));
            }
            //  +
            //  |
            if bottom.can_connect(&Top){
                elm.push(line(m,w));
            }
            // -+
            if left.can_connect(&Right) 
                || left.can_dynamic_connect(&Right){
                elm.push(line(m,k));
            }
            //  +-
            if right.can_connect(&Left) 
                || right.can_dynamic_connect(&Left){
                elm.push(line(m,o));
            }
            //  \
            //   +
            if top_left.can_static_connect(&BottomRight){
                elm.push(line(m,a));
            }
            //    /
            //   +
            if top_right.can_static_connect(&BottomLeft){
                elm.push(line(m,e));
            }
            //   +
            //  /
            if bottom_left.can_static_connect(&TopRight){
                elm.push(line(m,u));
            }
            //   +
            //    \
            if bottom_right.can_static_connect(&TopLeft){
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
        //     oO
        //
        ///////////////////////////////
        if self.is_open_round_marker(){
            let mut connects = false;
            //    |   |   +  ┌
            //    o   O   O  |
            if top.can_static_connect(&Bottom){
                elm.push(line(m,c));
                connects = true;
            }
            //    o   O   O  |
            //    |   |   +  ┘
            if bottom.can_static_connect(&Top){
                elm.push(line(m,w));
                connects = true;
            }
            //     o-  O- o+  o┘
            if right.can_static_connect(&Left){
               elm.push(line(m,o));
                connects = true;
            }
            //    -o 
            if left.can_static_connect(&Right){
                elm.push(line(m,k));
                connects = true;
            }
            //   \   
            //    o   
            if top_left.can_static_connect(&BottomRight){
                elm.push(line(a,m));
                connects = true;
            }
            //     /  
            //    o  
            if top_right.can_static_connect(&BottomLeft){
                elm.push(line(e,m));
                connects = true;
            }
            //     o  
            //    /   
            if bottom_left.can_static_connect(&TopRight){
                elm.push(line(m,u));
                connects = true;
            }
            //     o  
            //      \  
            if bottom_right.can_static_connect(&TopLeft){
                elm.push(line(m,y));
                connects = true;
            }
            if connects{
                elm.push(open_circle(m,tw2));
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
            if top.can_static_connect(&Bottom){
                elm.push(line(m,c));
                connects = true;
            }
            //    *   *  *
            //    |   +  ┘
            if bottom.can_static_connect(&Top){
                elm.push(line(m,w));
                connects = true;
            }
            //     *-  *+  *┘
            if right.can_static_connect(&Left){
               elm.push(line(m,o));
                connects = true;
            }
            //    -* 
            if left.can_static_connect(&Right){
                elm.push(line(m,k));
                connects = true;
            }
            //   \   
            //    *   
            if top_left.can_static_connect(&BottomRight){
                elm.push(line(a,m));
                connects = true;
            }
            //     /  
            //    *  
            if top_right.can_static_connect(&BottomLeft){
                elm.push(line(e,m));
                connects = true;
            }
            //     *  
            //    /   
            if bottom_left.can_static_connect(&TopRight){
                elm.push(line(m,u));
                connects = true;
            }
            //     *  
            //      \  
            if bottom_right.can_static_connect(&TopLeft){
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
            if bottom_left.can_static_connect(&TopRight){
                elm.push(arrow_line(u,m));
            }
            //   ^
            //    \
            if bottom_right.can_static_connect(&TopLeft){
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
            else if top_left.can_static_connect(&BottomRight) && top_right.can_static_connect(&BottomLeft){
                elm.extend(vec![line(a,m),line(m,e)]);
            }
            //   \    \
            //    v    V
            else if top_left.can_static_connect(&BottomRight){
                elm.push(arrow_line(a,m));
            }
            //     /   /
            //    v   V
            else if top_right.can_static_connect(&BottomLeft){
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
            if top_right.can_static_connect(&BottomLeft) 
                && bottom_right.can_static_connect(&TopLeft){
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
            if left.is('`'){
                elm.push(blank_text(loc));
            }
            let mut deformed = false;
            //    \
            //     >
            //    /
            if top_left.can_static_connect(&BottomRight) && bottom_left.can_static_connect(&TopRight){
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
            if !deformed && right.can_static_connect(&Left){
                elm.push(arrow_line(n,o));
            }
        }
        if self.is('('){
            //    /      (
            //   (        \
            if top_right.can_static_connect(&BottomLeft) || bottom_right.can_static_connect(&TopLeft){
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
            if top_left.can_static_connect(&BottomRight) || bottom_left.can_static_connect(&TopRight){
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
            if top_left.can_static_connect(&BottomRight){
                elm.extend(vec![line(a,m),line(m,w)]);
                trimmed = true;
            }
            //       /
            //      |
            if top_right.can_static_connect(&BottomLeft){
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
            if bottom_left.can_static_connect(&TopRight){
                elm.extend(vec![line(u,m),line(c,m)]);
                trimmed = true;
            }
            //     |
            //      \
            if bottom_right.can_static_connect(&TopLeft){
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
            if left.can_connect(&Right){
                elm.push(line(m,k));
                deformed = false;
                interacted = true;
            }
            // --    -+
            if right.can_connect(&Left){
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
            if top.can_static_connect(&BottomLeft)
                || top_left.can_connect(&Bottom)
                || top_left.can_connect(&BottomRight)

                || bottom_right.can_connect(&Top)
                || bottom_right.can_connect(&TopLeft)
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
            if top_left.can_connect(&BottomRight)
                || top_right.can_connect(&Bottom)
                || top_right.can_connect(&BottomLeft)

                || bottom_left.can_static_connect(&Top)
                || bottom_left.can_connect(&TopRight)
                || bottom_right.can_connect(&Top)
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
                && left_left.is('.'){
                elm.push(line(p,r));
            }
            if right.is('.')
                && right_right.is('.'){
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
        if elm.len() < 1 {
            let quoted = ::escape_char(&self.text());
            elm.push(text(loc,quoted));
        }
        elm
    }


    fn loc_x(&self) -> f32 {
        self.loc.x as f32
    }

    fn loc_y(&self) -> f32 {
        self.loc.y as f32
    }
    


    fn top(&self) -> Self {
        self.get(&self.loc.top())
    }

    fn bottom(&self) -> Self {
       self.get(&self.loc.bottom())
    }

    fn left(&self) -> Self {
       self.get(&self.loc.left())
    }

    fn right(&self) -> Self {
       self.get(&self.loc.right())
    }

    fn top_left(&self) -> Self {
       self.get(&self.loc.top_left())
    }

    fn top_right(&self) -> Self {
       self.get(&self.loc.top_right())
    }

    fn bottom_left(&self) -> Self {
       self.get(&self.loc.bottom_left())
    }

    fn bottom_right(&self) -> Self {
       self.get(&self.loc.bottom_right())
    }

    fn text_width(&self) -> f32 {
        self.grid.settings.text_width
    }

    fn text_height(&self) -> f32 {
        self.grid.settings.text_height
    }

    fn tw2(&self) -> f32 {
        self.text_width() * 1.0/2.0
    }

    fn tw4(&self) -> f32 {
        self.text_width() * 1.0
    }


    fn th2(&self) -> f32 {
        self.text_height() * 1.0/2.0
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


}


