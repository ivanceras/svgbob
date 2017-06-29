//! generate an SVG from the ascii text using the default settings
//!
//! ```
//! let input = r#"
//! .-------------------------------------.
//! | Hello here and there and everywhere |
//! '-------------------------------------'
//! "#;
//! println!("svg: {}",svgbob::to_svg(input));
//! ```
//! 
//! <svg font-family="Electrolize,Titillium Web, Trebuchet MS, Arial" font-size="14" height="80" width="344" xmlns="http://www.w3.org/2000/svg">
//! <defs>
//! <marker id="triangle" markerHeight="10" markerUnits="strokeWidth" markerWidth="10" orient="auto" refX="0" refY="5" viewBox="0 0 14 14">
//! <path d="M 0 0 L 10 5 L 0 10 z"/>
//! </marker>
//! </defs>
//! <style>
//!     line, path {
//!       stroke: black;
//!       stroke-width: 1;
//!     }
//! </style>
//! <path d=" M 36 28 L 36 48 M 40 24 A 4 4 0 0 0 36 28 M 40 24 L 336 24 M 340 28 L 340 48 M 340 28 A 4 4 0 0 0 336 24 M 36 32 L 36 48 M 340 32 L 340 48 M 36 48 L 36 52 A 4 4 0 0 0 40 56 L 336 56 M 340 48 L 340 52 M 336 56 A 4 4 0 0 0 340 52" fill="none"/>
//! <path d="" fill="none" stroke-dasharray="3 3"/>
//! <text x="50" y="44">
//! Hello here and there and everywhere
//! </text>
//! </svg>
//! 
//! 
//#![deny(warnings)]
#![feature(test)]
extern crate svg;
extern crate unicode_width;
#[cfg(test)]
#[macro_use] 
extern crate pretty_assertions;



use svg::Node;
use svg::node::element::Circle as SvgCircle;
use svg::node::element::Path as SvgPath;
use svg::node::element::Line as SvgLine;
use svg::node::element::Text as SvgText;
use svg::node::element::Style;
use svg::node::element::SVG;
use svg::node::element::Definitions;
use svg::node::element::Marker;
use optimizer::Optimizer;
use self::Feature::Arrow;
use self::Feature::Circle;
use self::Feature::Nothing;
use self::Stroke::Solid;
use self::Stroke::Dashed;
use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;
use patterns::FocusChar;

mod optimizer;
mod patterns;


/// generate an SVG from the ascii text input
///
/// Usage:
/// 
/// ```
/// let input = "------->";
/// println!("svg: {}", svgbob::to_svg(input));
/// ``` 
/// 
/// commercial version enhances memes automatically
pub fn to_svg(input: &str) -> SVG {
    Grid::from_str(&input, &Settings::default()).get_svg()
}

pub fn to_svg_with_size(input: &str, text_width: f32, text_height: f32) -> SVG {
    let settings = Settings::with_size(text_width, text_height);
    Grid::from_str(&input, &settings).get_svg()
}

pub fn to_svg_with_size_nooptimization(input: &str, text_width: f32, text_height: f32) -> SVG {
    let mut settings = Settings::no_optimization();
    settings.text_width = text_width;
    settings.text_height = text_height;
    Grid::from_str(&input, &settings).get_svg()
}


///  optimization options:
///  1. None -> Fastest, but not correct looking (paths and text are not reduced)
///  2. Fast -> Fast and correct looking (text are reduced)
///  3. All -> Correct looking but slow (paths and text are reduced)
#[derive(Debug)]
#[derive(Clone)]
pub struct Settings {
    pub text_width: f32,
    pub text_height: f32,
    /// do optimization? if false then every piece are disconnected
    optimize: bool,
    /// if optmization is enabled,
    /// true means all reduceable paths will be in 1 path definition
    compact_path: bool,
}

impl Settings {

    pub fn with_size(text_width: f32, text_height: f32) -> Self{
         Settings{
            text_width: text_width,
            text_height: text_height,
            optimize: true,
            compact_path: true,
         }
    }
    pub fn no_optimization() -> Settings {
        let mut settings = Settings::default();
        settings.optimize = false;
        settings.compact_path = false;
        settings
    }

    pub fn separate_lines() -> Settings {
        let mut settings = Settings::default();
        settings.optimize = true;
        settings.compact_path = false;
        settings
    }

    pub fn compact() -> Settings {
        let mut settings = Settings::default();
        settings.optimize = true;
        settings.compact_path = true;
        settings
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            text_width: 8.0,
            text_height: 16.0,
            optimize: true,
            compact_path: true,
        }
    }
}

enum SvgElement {
    Circle(SvgCircle),
    Line(SvgLine),
    Path(SvgPath),
    Text(SvgText),
}


#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Stroke {
    Solid,
    Dashed,
}


#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Feature {
    Arrow, //end
    Circle, //start
    Nothing,
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Point {
    x: f32,
    y: f32,
}
impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Loc {
    pub x: i32,
    pub y: i32,
}

impl Loc {
    pub fn new(x: i32, y: i32) -> Loc {
        Loc { x: x, y: y }
    }

    pub fn top(&self) -> Loc {
        Loc {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn bottom(&self) -> Loc {
        Loc {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn right(&self) -> Loc {
        Loc {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn top_left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    pub fn top_right(&self) -> Loc {
        Loc {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    pub fn bottom_left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    pub fn bottom_right(&self) -> Loc {
        Loc {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Element {
    Circle(Point, f32, String),
    Line(Point, Point, Stroke, Feature),
    Arc(Point, Point, f32, bool, Stroke, Feature),
    Text(Loc, String),
    Path(Point, Point, String, Stroke),
}

impl Element {
    fn solid_circle(c: &Point, r: f32) -> Element{
        Element::Circle(c.clone(), r, "solid".into())
    }
    fn open_circle(c: &Point, r: f32) -> Element{
        Element::Circle(c.clone(), r, "open".into())
    }
    fn solid_line(s: &Point, e: &Point) -> Element {
        Element::line(s, e, Solid, Nothing)
    }

    fn arrow_line(s: &Point, e: &Point) -> Element {
        Element::line(s,e,Solid,Arrow)
    }

    fn line(s: &Point, e: &Point, stroke: Stroke, feature: Feature) -> Element {
        Element::Line(s.clone(), e.clone(), stroke, feature)
    }
    fn arc(s: &Point, e: &Point, radius: f32, sweep: bool) -> Element {
        Element::Arc(s.clone(), e.clone(), radius, sweep, Solid, Nothing)
    }

    fn arrow_arc(s: &Point, e: &Point, radius: f32, sweep: bool) -> Element {
        Element::Arc(s.clone(), e.clone(), radius, sweep, Solid, Arrow)
    }

    // if this element can reduce the other, return the new reduced element
    // for line it has to be collinear and in can connect start->end->start
    // for text, the other text should apear on the right side of this text
    fn reduce(&self, other: &Element) -> Option<Element> {
        match *self {
            Element::Line(ref s, ref e, ref stroke, ref feature) => {
                match *other {
                    Element::Line(ref s2, ref e2, ref stroke2, ref feature2) => {
                        // note: dual 3 point check for trully collinear lines
                        if collinear(s, e, s2) && collinear(s, e, e2) && e == s2 &&
                           stroke == stroke2 && *feature == Nothing
                           && *feature2 != Circle
                           {
                            let reduced = Some(Element::Line(s.clone(),
                                                             e2.clone(),
                                                             stroke.clone(),
                                                             feature2.clone()));
                            reduced
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            Element::Text(ref loc, ref text) => {
                match *other {
                    Element::Text(ref loc2, ref text2) => {
                        // reduce if other is next to it
                        let uwidth = text.width() as i32;
                        if loc.y == loc2.y && loc.x + uwidth == loc2.x {
                            let merged_text = text.clone() + text2;
                            let reduced = Some(Element::Text(loc.clone(), merged_text));
                            reduced
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// convert drawing element to SVG element
    fn to_svg(&self, settings: &Settings) -> SvgElement {
        match *self {
            Element::Circle(ref c, r, ref class) => {
                let svg_circle = SvgCircle::new()
                    .set("class",class.clone())
                    .set("cx", c.x)
                    .set("cy", c.y)
                    .set("r", r);

                SvgElement::Circle(svg_circle)
            },
            Element::Line(ref s, ref e, ref stroke, ref feature) => {
                let mut svg_line = SvgLine::new()
                    .set("x1", s.x)
                    .set("y1", s.y)
                    .set("x2", e.x)
                    .set("y2", e.y);

                match *feature {
                    Arrow => {
                        svg_line.assign("marker-end", "url(#triangle)");
                    },
                    Circle => {
                        svg_line.assign("marker-start", "url(#circle)");
                    },
                    Nothing => (),
                };
                match *stroke {
                    Solid => (),
                    Dashed => {
                        svg_line.assign("stroke-dasharray", (3, 3));
                        svg_line.assign("fill", "none");
                    }
                };

                SvgElement::Line(svg_line)
            }
            Element::Arc(ref s, ref e, radius, sweep, _, ref feature) => {
                let sweept = if sweep { "1" } else { "0" };
                let d = format!("M {} {} A {} {} 0 0 {} {} {}",
                                s.x,
                                s.y,
                                radius,
                                radius,
                                sweept,
                                e.x,
                                e.y);
                let mut svg_arc = SvgPath::new()
                    .set("d", d)
                    .set("fill", "none");
                match *feature {
                    Arrow => {
                        svg_arc.assign("marker-end", "url(#triangle)");
                    },
                    Circle => {
                        svg_arc.assign("marker-start", "url(#circle)");
                    },
                    Nothing => (),
                };
                SvgElement::Path(svg_arc)
            }
            Element::Text(ref loc, ref string) => {
                let sx = loc.x as f32 * settings.text_width + settings.text_width / 8.0;
                let sy = loc.y as f32 * settings.text_height + settings.text_height * 3.0 / 4.0;
                let mut svg_text = SvgText::new()
                    .set("x", sx)
                    .set("y", sy);
                let text_node = svg::node::Text::new(string.to_string());
                svg_text.append(text_node);
                SvgElement::Text(svg_text)
            }
            Element::Path(_, _, ref d, ref stroke) => {
                let mut path = SvgPath::new()
                    .set("d", d.to_owned())
                    .set("fill", "none");

                match *stroke {
                    Solid => (),
                    Dashed => {
                        path.assign("stroke-dasharray", (3, 3));
                    }
                };
                SvgElement::Path(path)
            }
        }
    }
}


// 3 points are collinear when the area of the triangle connecting them is 0;
fn collinear(a: &Point, b: &Point, c: &Point) -> bool {
    a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y) == 0.0
}


pub struct Neighbor{
    pub top_left_left: String,
    pub top_left: String,
    pub top: String,
    pub top_right: String,
    pub top_right_right: String,
    pub left_left: String,
    pub left: String,
    pub this: String,
    pub right: String,
    pub right_right: String,
    pub bottom_left_left: String,
    pub bottom_left: String,
    pub bottom: String,
    pub bottom_right: String,
    pub bottom_right_right: String
}


#[derive(Debug)]
pub struct Grid {
    pub rows: usize,
    pub columns: usize,
    settings: Settings,
    index: Vec<Vec<String>>
}
impl Grid {
    /// instantiate a grid from input ascii text
    /// Issues:
    /// 1. 2-width, 5 bytes, single character  i.e. 统
    /// 2. 1-width, 2 bytes, single character  i.e. ö 
    /// 3. 1-width, 3 bytes, single character  i.e. o͡͡͡
    pub fn from_str(s: &str, settings: &Settings) -> Grid {
        let mut max_column_width = 0;
        let lines: Vec<&str> = s.lines().collect();
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(lines.len());
        for line in lines{
            if line.width() > max_column_width{
                max_column_width = line.width();
            }
            let mut row: Vec<String> = Vec::with_capacity(line.chars().count());
            for ch in line.chars(){
                if let Some(1) = ch.width(){
                    row.push(format!("{}",ch));
                }
                else if let Some(2) = ch.width(){
                    row.push(format!("{}",ch));
                    row.push(format!("\0"));//push a blank
                }
                // if zero width char, append it to the previous string
                else if let Some(0) = ch.width(){
                    let prev: Option<String> = row.pop();
                    match prev{
                        Some(mut prev) => {
                            prev.push(ch);
                            row.push(prev);
                        },
                        None => (),
                    }
                }
            }
            rows.push(row);
        }
        Grid {
            rows: rows.len(), 
            columns: max_column_width,
            settings: settings.clone(),
            index: rows,
        }
    }


    /// get a character at this location
    /// widths are computed since there are
    /// characters that spans 2 columns
    /// and characters that has 0 width
    ///
    pub fn get(&self, loc: &Loc) -> Option<&String> {
        match self.index.get(loc.y as usize) {
            Some(row) => {
                row.get(loc.x as usize)
            }
            None => None,
        }
    }

    fn get_string(&self, loc: &Loc) -> String {
        match self.get(loc){
            Some(s) => s.clone(),
            None => "".to_string()
        }
    }

    pub fn get_neighbor_text(&self, loc: &Loc) -> Neighbor {
        Neighbor{
            top_left_left: self.get_string(&loc.top_left().left()),
            top_left: self.get_string(&loc.top_left()),
            top: self.get_string(&loc.top()),
            top_right: self.get_string(&loc.top_right()),
            top_right_right: self.get_string(&loc.top_right().right()),
            left_left: self.get_string(&loc.left().left()),
            left: self.get_string(&loc.left()),
            this: self.get_string(&loc),
            right: self.get_string(&loc.right()),
            right_right: self.get_string(&loc.right().right()),
            bottom_left_left: self.get_string(&loc.bottom_left().left()),
            bottom_left: self.get_string(&loc.bottom_left()),
            bottom: self.get_string(&loc.bottom()),
            bottom_right: self.get_string(&loc.bottom_right()),
            bottom_right_right: self.get_string(&loc.bottom_right().right()),
        }
    }

    



    /// vector of each elements arranged in rows x columns
    fn get_all_elements(&self) -> Vec<Vec<Vec<Element>>> {
        let mut rows: Vec<Vec<Vec<Element>>> = Vec::with_capacity(self.index.len());
        let mut y = 0;
        for line in &self.index{
            let mut x = 0;
            let mut row: Vec<Vec<Element>> = Vec::with_capacity(line.len());
            for _ in line {
                let loc = Loc::new(x,y);
                let focus_char = FocusChar::new(&loc,self);
                let cell_elements = focus_char.get_elements();
                row.push(cell_elements);
                x += 1;
            }
            rows.push(row);
            y += 1;
        }
        rows
    }

    /// each component has its relative location retain
    /// use this info for optimizing svg by checking closest neigbor
    fn get_svg_nodes(&self) -> Vec<SvgElement> {
        let mut nodes = vec![];
        let start = std::time::SystemTime::now();
        let elements = self.get_all_elements();
        eprintln!("getting elements took {:?} {} ms", start.elapsed().unwrap(), start.elapsed().unwrap().subsec_nanos() / 1_000_000);
        let input = if self.settings.optimize {
            let now = std::time::SystemTime::now();
            let optimizer = Optimizer::new(elements);
            let optimized_elements = optimizer.optimize(&self.settings);
            eprintln!("optimization took {:?} {} ms", now.elapsed().unwrap(), now.elapsed().unwrap().subsec_nanos() / 1_000_000);
            optimized_elements
        } else {
            // flatten Vec<Vec<Vec<Elements>>> to Vec<Element>
            elements.into_iter().flat_map(
                |elm| {
                    elm.into_iter().flat_map(|e2|e2)
                }
            ).collect()
        };
        for elem in input {
            let element:SvgElement = elem.to_svg(&self.settings);
            nodes.push(element);
        }
        nodes
    }


    /// get the generated svg according to the settings specified
    pub fn get_svg(&self) -> SVG {
        let nodes = self.get_svg_nodes();
        let width = self.settings.text_width * self.columns  as f32;
        let height = self.settings.text_height * self.rows as f32;
        let mut svg = SVG::new()
            .set("font-size", 14)
            .set("font-family",
                "arial"
                )
            .set("width", width)
            .set("height", height);

        svg.append(get_defs());
        svg.append(get_styles());

        for node in nodes {
            match node {
                SvgElement::Circle(circle) => {
                    svg.append(circle);
                }
                SvgElement::Line(line) => {
                    svg.append(line);
                }
                SvgElement::Path(path) => {
                    svg.append(path);
                }
                SvgElement::Text(text) => {
                    svg.append(text);
                }
            }
        }
        svg
    }
}

fn get_defs() -> Definitions {
    let mut defs = Definitions::new();
    defs.append(arrow_marker());
    defs
}

fn get_styles() -> Style {
    let style = r#"
    line, path {
      stroke: black;
      stroke-width: 2;
      stroke-opacity: 1;
      fill-opacity: 1;
      stroke-linecap: round;
      stroke-linejoin: miter;
    }
    circle {
      stroke: black;
      stroke-width: 2;
      stroke-opacity: 1;
      fill-opacity: 1;
      stroke-linecap: round;
      stroke-linejoin: miter;
      fill:white;
    }
    circle.solid {
      fill:black;
    }
    circle.open {
      fill:white;
    }
    tspan.head{
        fill: none;
        stroke: none;
    }
    "#;
    Style::new(style)
}

fn arrow_marker() -> Marker {
    let mut marker = Marker::new()
        .set("id", "triangle")
        .set("viewBox", "0 0 50 20")
        .set("refX", 15)
        .set("refY", 10)
        .set("markerUnits", "strokeWidth")
        .set("markerWidth", 10)
        .set("markerHeight", 10)
        .set("orient", "auto");

    let path = SvgPath::new().set("d", "M 0 0 L 30 10 L 0 20 z");
    marker.append(path);
    marker

}

fn escape_char(ch: &str) -> String {
    let escs = [("\"", "&quot;"), ("'", "&apos;"), ("<", "&lt;"), (">", "&gt;"), ("&", "&amp;")];
    let quote_match: Option<&(&str, &str)> = escs.iter()
        .find(|pair| {
            let &(e, _) = *pair;
            e == ch
        });
    let quoted: String = match quote_match {
        Some(&(_, quoted)) => String::from(quoted),
        None => {
            let mut s = String::new();
            s.push_str(&ch);
            s
        }
    };
    quoted

}

#[cfg(test)]
mod test_lib{
    use super::Grid;
    use super::Settings;

    #[test]
    fn test_grid(){
        let g = Grid::from_str("a统öo͡͡͡", &Settings::compact());
        println!("{:?}", g.index);
        assert_eq!(g.index, vec![vec!["a".to_string(), "统".to_string(), "\u{0}".to_string(), "ö".to_string(), "o͡͡͡".to_string()]]);
    }
}

#[cfg(test)]
mod benchmark{
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn convert(b: &mut Bencher) {
        let arg = ".-----.";
        b.iter(|| super::to_svg(arg))
    }
}
