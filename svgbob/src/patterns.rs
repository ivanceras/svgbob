use enhance_circles::Round;
use properties::Characteristic;
use properties::Signal;
use Element;
use Grid;
use Loc;
use Point;
use Settings;

use fragments::Block;
use fragments::Block::{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y};
use properties::Location;
use properties::PointBlock;

use fragments::Fragment;
use fragments::Fragment::Text;

use enhance::Enhance;
use properties::Can;
use properties::Properties;
use properties::Signal::{Medium, Strong, Weak};

use {arc, arrow_line, line, open_circle, solid_circle, text};

use properties::Can::{ConnectTo, Is, IsStrongAll};

struct LocBlock {
    loc: Loc,
    settings: Settings,
}

impl LocBlock {
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

    /// 1 unit in x dimension is 1/4 of the textwidth
    /// used in calculating the radius, adjustments
    /// in the Fragment PointBlock
    fn unit_x(&self) -> f32 {
        self.text_width() * 1.0 / 4.0
    }

    /// 1 unit along y dimension is 1/4 of the textwidth
    #[allow(unused)]
    fn unit_y(&self) -> f32 {
        self.text_height() * 1.0 / 4.0
    }

    /// x coordinate on increment of 1/4 of text width
    fn x0(&self) -> f32 {
        self.loc_x() * self.text_width()
    }

    fn x1(&self) -> f32 {
        (self.loc_x() + 1.0 / 4.0) * self.text_width()
    }

    fn x2(&self) -> f32 {
        (self.loc_x() + 1.0 / 2.0) * self.text_width()
    }

    fn x3(&self) -> f32 {
        (self.loc_x() + 3.0 / 4.0) * self.text_width()
    }

    fn x4(&self) -> f32 {
        (self.loc_x() + 1.0) * self.text_width()
    }

    /// y coordinate on increment of 1/4 of text_height
    fn y0(&self) -> f32 {
        self.loc_y() * self.text_height()
    }

    fn y1(&self) -> f32 {
        (self.loc_y() + 1.0 / 4.0) * self.text_height()
    }

    fn y2(&self) -> f32 {
        (self.loc_y() + 1.0 / 2.0) * self.text_height()
    }

    fn y3(&self) -> f32 {
        (self.loc_y() + 3.0 / 4.0) * self.text_height()
    }

    fn y4(&self) -> f32 {
        (self.loc_y() + 1.0) * self.text_height()
    }

    /// 1st row a,b,c,d,e
    fn a(&self) -> Point {
        Point::new(self.x0(), self.y0())
    }

    fn b(&self) -> Point {
        Point::new(self.x1(), self.y0())
    }

    fn c(&self) -> Point {
        Point::new(self.x2(), self.y0())
    }

    fn d(&self) -> Point {
        Point::new(self.x3(), self.y0())
    }

    fn e(&self) -> Point {
        Point::new(self.x4(), self.y0())
    }

    /// 2nd row f,g,h,i,j
    fn f(&self) -> Point {
        Point::new(self.x0(), self.y1())
    }

    fn g(&self) -> Point {
        Point::new(self.x1(), self.y1())
    }

    fn h(&self) -> Point {
        Point::new(self.x2(), self.y1())
    }

    fn i(&self) -> Point {
        Point::new(self.x3(), self.y1())
    }

    fn j(&self) -> Point {
        Point::new(self.x4(), self.y1())
    }

    /// 3rd row k,l,m,n,o
    fn k(&self) -> Point {
        Point::new(self.x0(), self.y2())
    }

    fn l(&self) -> Point {
        Point::new(self.x1(), self.y2())
    }

    fn m(&self) -> Point {
        Point::new(self.x2(), self.y2())
    }

    fn n(&self) -> Point {
        Point::new(self.x3(), self.y2())
    }

    fn o(&self) -> Point {
        Point::new(self.x4(), self.y2())
    }

    /// 4th row p,q,r,s,t
    fn p(&self) -> Point {
        Point::new(self.x0(), self.y3())
    }

    fn q(&self) -> Point {
        Point::new(self.x1(), self.y3())
    }

    fn r(&self) -> Point {
        Point::new(self.x2(), self.y3())
    }

    fn s(&self) -> Point {
        Point::new(self.x3(), self.y3())
    }

    fn t(&self) -> Point {
        Point::new(self.x4(), self.y3())
    }

    /// 5th row u,v,w,x,y
    fn u(&self) -> Point {
        Point::new(self.x0(), self.y4())
    }

    fn v(&self) -> Point {
        Point::new(self.x1(), self.y4())
    }

    fn w(&self) -> Point {
        Point::new(self.x2(), self.y4())
    }

    fn x(&self) -> Point {
        Point::new(self.x3(), self.y4())
    }

    fn y(&self) -> Point {
        Point::new(self.x4(), self.y4())
    }

    pub fn to_point(&self, pb: &PointBlock) -> Point {
        // move loc to the additional location relative to itself
        let loc = if let Some(ref pbloc) = pb.location {
            self.loc.from_location(&pbloc)
        } else {
            self.loc.clone()
        };
        let lb = LocBlock {
            loc: loc,
            settings: self.settings.clone(),
        };
        let mut p = match pb.block {
            A => lb.a(),
            B => lb.b(),
            C => lb.c(),
            D => lb.d(),
            E => lb.e(),
            F => lb.f(),
            G => lb.g(),
            H => lb.h(),
            I => lb.i(),
            J => lb.j(),
            K => lb.k(),
            L => lb.l(),
            M => lb.m(),
            N => lb.n(),
            O => lb.o(),
            P => lb.p(),
            Q => lb.q(),
            R => lb.r(),
            S => lb.s(),
            T => lb.t(),
            U => lb.u(),
            V => lb.v(),
            W => lb.w(),
            X => lb.x(),
            Y => lb.y(),
        };
        let unit = self.unit_x();
        p.adjust(pb.adjust.0 * unit, pb.adjust.1 * unit);
        p
    }
}

#[derive(Debug, Clone)]
pub struct FocusChar<'g> {
    loc: Loc,
    ch: char,
    grid: &'g Grid,
}

impl<'g> FocusChar<'g> {
    pub fn new(loc: &Loc, grid: &'g Grid) -> Self {
        let s: Option<&String> = grid.get(loc);
        // if there is a text in this location, take the first char as the focus char
        let ch = match s {
            Some(s) => s.chars().nth(0).unwrap_or('\0'),
            None => '\0',
        };

        Self {
            loc: loc.clone(),
            ch: ch,
            grid: grid,
        }
    }

    /// get the text of self char, including complex block
    /// concatenated with multiple strings in utf8 encoding
    fn text(&self) -> String {
        match self.grid.get(&self.loc) {
            Some(s) => s.to_owned(),
            None => "".to_string(),
        }
    }

    /// get the focus char at this location
    fn get(&self, loc: &Loc) -> Self {
        FocusChar::new(loc, self.grid)
    }

    /// if the character matches given argument
    pub fn is(&self, ch: char) -> bool {
        self.ch.is(ch)
    }

    /// if character is any character in the string
    pub fn any(&self, s: &str) -> bool {
        self.ch.any(s)
    }

    fn used_as_text(&self) -> bool {
        if self.is_text_surrounded() {
            // not if it can strongly connect to 4 directions
            if self.can_strongly_connect(&Block::O) || self.can_strongly_connect(&Block::K)
                || self.can_strongly_connect(&Block::C)
                || self.can_strongly_connect(&Block::W)
                || self.can_strongly_connect(&Block::U)
                || self.can_strongly_connect(&Block::Y)
            {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    fn is_text_surrounded(&self) -> bool {
        self.left().ch.is_alphanumeric() || self.right().ch.is_alphanumeric()
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
        self.can_strongly_connect(block) || self.can_medium_connect(block)
    }

    fn can_pass_weakly_connect(&self, block: &Block) -> bool {
        self.can_strongly_connect(block) || self.can_medium_connect(block)
            || self.can_weakly_connect(block)
    }

    pub fn can_strongly_connect(&self, block: &Block) -> bool {
        self.ch.can_connect(&Strong, block)
    }

    fn can_medium_connect(&self, block: &Block) -> bool {
        self.ch.can_connect(&Medium, block)
    }

    fn can_weakly_connect(&self, block: &Block) -> bool {
        self.ch.can_connect(&Weak, block)
    }

    fn point(&self, pb: &PointBlock) -> Point {
        self.loc_block().to_point(pb)
    }

    fn loc_block(&self) -> LocBlock {
        LocBlock {
            loc: self.loc.to_owned(),
            settings: self.get_settings(),
        }
    }

    fn to_element(&self, frag: Fragment) -> Element {
        let unit_x = self.loc_block().unit_x();
        match frag {
            Fragment::Line(p1, p2) => line(&self.point(&p1), &self.point(&p2)),
            Fragment::ArrowLine(p1, p2) => arrow_line(&self.point(&p1), &self.point(&p2)),

            Fragment::StartArrowLine(p1, p2) => arrow_line(&self.point(&p1), &self.point(&p2)),

            Fragment::Arc(p1, p2, m) => arc(&self.point(&p1), &self.point(&p2), m as f32 * unit_x),

            Fragment::OpenCircle(c, m) => open_circle(&self.point(&c), m as f32 * unit_x),

            Fragment::SolidCircle(c, m) => solid_circle(&self.point(&c), m as f32 * unit_x),
            Fragment::Text(s) => text(&self.loc, &s),
        }
    }

    /// TODO: optimize this by getting accumulating the location
    /// and convert it into loc in 1 call
    /// then get the focus char at this location;
    fn from_location(&self, location: &Location) -> FocusChar<'g> {
        let loc = self.loc.from_location(location);
        self.get(&loc)
    }

    fn can_block_pass_connect(&self, block: &Block, signal: &Signal) -> bool {
        match *signal {
            Strong => self.can_strongly_connect(block),
            Medium => self.can_pass_medium_connect(block),
            Weak => self.can_pass_weakly_connect(block),
        }
    }

    pub fn get_elements(&self) -> (Vec<Element>, Vec<Loc>) {
        let (fragments, consumed_location) = self.get_fragments();
        let elements: Vec<Element> = fragments
            .into_iter()
            .map(|frag| self.to_element(frag))
            .collect();
        let consumed_loc: Vec<Loc> = consumed_location
            .into_iter()
            .map(|location| self.loc.from_location(&location))
            .collect();
        (elements, consumed_loc)
    }

    fn is_satisfied(&self, can: &Can) -> bool {
        match *can {
            ConnectTo(ref cond_block, ref signal) => {
                self.can_block_pass_connect(&cond_block, signal)
            }
            Is(char) => self.is(char),
            IsStrongAll(ref blocks) => blocks.iter().all(|b| self.is_strong_block(&b)),
        }
    }

    /// check to see if this specified block for this focused
    /// char is intensified to be strong
    fn is_intensified(&self, arg_block: &Block) -> bool {
        let character: Option<Characteristic> = self.ch.get_characteristic();
        if let Some(character) = character {
            character.intensify.iter().any(|&(ref block, ref cond)| {
                let fc = self.from_location(&cond.loc);
                block == arg_block && fc.is_satisfied(&cond.can)
            })
        } else {
            false
        }
    }

    fn can_be_strong_block(&self, block: &Block) -> bool {
        if self.is_strong_block(block) {
            true
        } else if self.is_intensified(block) {
            true
        } else {
            false
        }
    }

    fn is_strong_block(&self, block: &Block) -> bool {
        let character: Option<Characteristic> = self.ch.get_characteristic();
        if let Some(character) = character {
            if character.is_strong_block(block) {
                return true;
            }
        }
        false
    }

    fn get_fragments(&self) -> (Vec<Fragment>, Vec<Location>) {
        let character: Option<Characteristic> = self.ch.get_characteristic();
        let mut elm: Vec<Fragment> = vec![];
        let mut consumed: Vec<Location> = vec![];

        let mut matched_intended = false;
        let mut matched_enhance = false;
        let mut matched_circles = false;

        let enable_round_circles = true;
        let enable_enhancements = true;
        let enable_intended_behavior = true;
        let enable_default_properties = true;

        // spaces has no character
        // that's why enhance circle didn't work out well
        // Issue#1 circle: The circle is matched by testing from the center
        // however, each elements along the circle would also be
        // threated as some other characters that has some other behaviors
        // causing multiple artifacts as a result of multiple usage of the elements
        // emitting fragments and merge all together when the circle is also matched.
        // To solve the issue, checking of circle should be done first
        // and then the consumed elements are skipped and checked
        if enable_round_circles {
            let (circles, circles_consumed, along_arc) = self.round();
            if !circles.is_empty() && !self.used_as_text() {
                elm.extend(circles);
                consumed.extend(circles_consumed);
                // if circle is matched, and element is along the arc
                // skip processing of other,
                // otherwise, even if circle is matched and the element is NOT along arc
                // do process other enhancement, behaviors
                matched_circles = along_arc;
            }
        }
        if let Some(character) = character {
            // enhancements
            if enable_enhancements {
                if !matched_circles {
                    let (enhanced, enhance_consumed) = self.enhance();
                    if !enhanced.is_empty() && !self.used_as_text() {
                        elm.extend(enhanced);
                        consumed.extend(enhance_consumed);
                        matched_enhance = true;
                    }
                }
            }

            // intended behaviors when signals are strong
            // after applying the intensifiers
            // do only when enhancements is not matched
            if enable_intended_behavior {
                if !matched_enhance && !matched_circles {
                    for &(ref blocks, ref fragments) in &character.intended_behavior {
                        let meet = blocks.iter().all(|ref b| self.can_be_strong_block(&b));
                        if meet && !self.used_as_text() {
                            elm.extend(fragments.clone());
                            matched_intended = true;
                        }
                    }
                }
            }

            // default behaviors
            // add only when signal is strong
            // or the signal has been intensified to strong
            let mut matched = false;
            if enable_default_properties {
                if !matched_enhance && !matched_circles && !matched_intended {
                    for &(ref block, ref _signal, ref fragments) in &character.properties {
                        // draw when used as text but intensified
                        if self.is_intensified(&block) && !self.used_as_text() {
                            elm.extend(fragments.clone());
                            matched = true;
                        }
                        // draw when a strong block and not used as text
                        else if self.is_strong_block(&block) && !self.used_as_text() {
                            elm.extend(fragments.clone());
                            matched = true;
                        }
                    }
                }
            }
            if !matched && !matched_intended && !matched_enhance && !matched_circles
                && !self.is_blank()
            {
                elm.push(Text(self.text()));
            }
            (elm, consumed)
        } else {
            if !self.is_blank() {
                // This is to disconnect words
                elm.push(Text(self.text()));
            }
            (elm, consumed)
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
        for _i in 0..n - 1 {
            fc = fc.left();
        }
        fc
    }
    pub fn in_right(&self, n: usize) -> Self {
        let mut fc = self.right();
        for _i in 0..n - 1 {
            fc = fc.right();
        }
        fc
    }

    pub fn in_top(&self, n: usize) -> Self {
        let mut fc = self.top();
        for _i in 0..n - 1 {
            fc = fc.top();
        }
        fc
    }
    pub fn in_bottom(&self, n: usize) -> Self {
        let mut fc = self.bottom();
        for _i in 0..n - 1 {
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
mod test {
    use super::super::Loc;
    use super::super::Settings;
    use super::FocusChar;
    use super::Grid;
    use fragments::Direction::*;
    use properties::Location;

    use fragments::Block::{O, U, Y};

    #[test]
    fn test_adjascent() {
        let g = Grid::from_str("a统öo͡͡͡", &Settings::compact());
        let fc = FocusChar::new(&Loc::new(1, 0), &g);
        println!("{:?}", fc);
        assert!(fc.left().is('a'));
        assert!(fc.right().right().is('ö'));
    }

    #[test]
    fn test100() {
        //  ._
        let g = Grid::from_str(".-", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(0, 0), &g);
        let (frags, _consumed) = fc.get_fragments();
        println!("frags: {:?}", frags);
        assert!(fc.is_intensified(&O));
        assert!(fc.can_be_strong_block(&O));
    }

    #[test]
    fn test_location() {
        //  ._
        let g = Grid::from_str(".-", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(0, 0), &g);
        let (_frags, _consumed) = fc.get_fragments();
        let go_right = fc.from_location(&Location::go(Right));
        let right = fc.right();
        let right2 = fc.in_right(2);
        let mut right2_loop = fc.clone();
        for _ in 0..2 {
            right2_loop = right2_loop.in_right(1);
        }
        println!("in right 2: {:?}", right2.loc);
        println!("in right 2 loop: {:?}", right2_loop.loc);
        assert_eq!(right2.loc, right2_loop.loc);
        assert_eq!(go_right.loc, right.loc);
    }

    #[test]
    fn test_loc() {
        let g = Grid::from_str("", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(0, 0), &g);
        let right = fc.right();
        let in_right = fc.in_right(1);
        assert_eq!(Loc::new(1, 0), right.loc);
        assert_eq!(Loc::new(1, 0), in_right.loc);
    }

    #[test]
    fn test1() {
        //  ._
        let g = Grid::from_str("._", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(0, 0), &g);
        println!("focus char: {:#?}", fc);
        let (frags, _consumed) = fc.get_fragments();
        println!("frags: {:?}", frags);
        assert!(!fc.is_intensified(&U));
        assert!(fc.is_intensified(&Y));
    }
    #[test]
    fn test2() {
        //  ._
        let g = Grid::from_str("._", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(1, 0), &g);
        println!("focus char: {:#?}", fc);
        let (frags, _consumed) = fc.get_fragments();
        println!("frags: {:?}", frags);
        assert!(!fc.is_intensified(&Y));
        assert!(!fc.is_intensified(&U));
        assert!(fc.can_be_strong_block(&Y));
        assert!(fc.can_be_strong_block(&U));
    }

    #[test]
    fn test_no_character() {
        use properties::Properties;
        use {FocusChar, Grid, Loc, Settings};

        let g = Grid::from_str(".l", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(1, 0), &g);
        println!("focus char: {:#?}", fc);
        let character = fc.ch.get_characteristic();
        println!("character: {:#?}", character);
        assert!(character.is_none());
    }

    #[test]
    fn test_has_character() {
        use properties::Properties;
        use {FocusChar, Grid, Loc, Settings};

        let g = Grid::from_str(".╦", &Settings::separate_lines());
        let fc = FocusChar::new(&Loc::new(1, 0), &g);
        println!("focus char: {:#?}", fc);
        let character = fc.ch.get_characteristic();
        println!("character: {:#?}", character);
        assert!(character.is_some());
    }
}
