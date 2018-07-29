use block::Block;
use fragments::Fragment;
use loc::Loc;
use grid::Grid;
use properties::{
    Properties,
    Signal::{self,Weak,Medium,Strong},
    Can::{self,ConnectTo,Is,IsStrongAll},
    Characteristic,
};
use point_block::PointBlock;
use point::Point;
use loc_block::LocBlock;
use element::Element;
use fragments::Fragment::Text;
use element::{line,arrow_line,start_arrow_line,arc,open_circle,solid_circle,text};
use location::Location;
use settings::Settings;

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

            Fragment::StartArrowLine(p1, p2) => start_arrow_line(&self.point(&p1), &self.point(&p2)),

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
        let mut elements: Vec<Element> = fragments
            .into_iter()
            .map(|frag| self.to_element(frag))
            .collect();
        let consumed_loc: Vec<Loc> = consumed_location
            .into_iter()
            .map(|location| self.loc.from_location(&location))
            .collect();
        elements.sort();
        elements.dedup();
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

        let enable_intended_behavior = true;
        let enable_default_properties = true;

        if let Some(character) = character {
            // intended behaviors when signals are strong
            // after applying the intensifiers
            // do only when enhancements is not matched
            if enable_intended_behavior {
                for &(ref blocks, ref fragments) in &character.intended_behavior {
                    let meet = blocks.iter().all(|ref b| self.can_be_strong_block(&b));
                    if meet && !self.used_as_text() {
                        elm.extend(fragments.clone());
                        matched_intended = true;
                    }
                }
            }

            // default behaviors
            // add only when signal is strong
            // or the signal has been intensified to strong
            let mut matched = false;
            if enable_default_properties {
                if !matched_intended {
                    for &(ref block, ref signal, ref fragments) in &character.properties {
                        // draw when a strong block and not used as text
                        if self.is_strong_block(&block) && !self.used_as_text() {
                            elm.extend(fragments.clone());
                            matched = true;
                        }
                        // draw when used as text but intensified
                        else if self.is_intensified(&block) && !self.used_as_text() {
                            elm.extend(fragments.clone());
                            matched = true;
                        }
                    }
                }
            }
            if !matched && !matched_intended 
                && !self.is_blank()
            {
                elm.push(Text(self.text()));
            }
        } else {
            if !self.is_blank() {
                // This is to disconnect words
                elm.push(Text(self.text()));
            }
        }
        elm.sort();
        elm.dedup();
        consumed.sort();
        consumed.dedup();
        (elm, consumed)
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
