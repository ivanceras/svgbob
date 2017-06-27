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
#![deny(warnings)]
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
    let settings = Settings::default();
    Grid::from_str(&input,settings).get_svg()
}

pub fn to_svg_with_size(input: &str, text_width: f32, text_height: f32) -> SVG {
    let settings = Settings::with_size(text_width, text_height);
    Grid::from_str(&input, settings).get_svg()
}

pub fn to_svg_with_size_nooptimization(input: &str, text_width: f32, text_height: f32) -> SVG {
    let mut settings = Settings::no_optimization();
    settings.text_width = text_width;
    settings.text_height = text_height;
    Grid::from_str(&input, settings).get_svg()
}


///  optimization options:
///  1. None -> Fastest, but not correct looking (paths and text are not reduced)
///  2. Fast -> Fast and correct looking (text are reduced)
///  3. All -> Correct looking but slow (paths and text are reduced)
#[derive(Debug)]
pub struct Settings {
    text_width: f32,
    text_height: f32,
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
    x: isize,
    y: isize,
}

impl Loc {
    fn new(x: isize, y: isize) -> Loc {
        Loc { x: x, y: y }
    }

    fn top(&self) -> Loc {
        Loc {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn bottom(&self) -> Loc {
        Loc {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn right(&self) -> Loc {
        Loc {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn top_left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    fn top_right(&self) -> Loc {
        Loc {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    fn bottom_left(&self) -> Loc {
        Loc {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn bottom_right(&self) -> Loc {
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
                        let uwidth = text.width() as isize;
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

#[derive(Debug)]
#[derive(PartialEq)]
pub struct GChar {
    /// the characters in this Element
    string: String,
    /// total width of all characters in chars
    width: usize,
}

impl GChar{
    fn new(ch:char) -> Self {
        GChar::from_str(&format!("{}",ch))
    }

    fn from_str(s:&str) -> Self{
        let width =UnicodeWidthStr::width(s);
        GChar{
            string: s.into(),
            width: width
        }
    }
    
}


#[derive(Debug)]
pub struct Grid {
    rows: usize,
    columns: usize,
    lines: Vec<Vec<GChar>>,
    settings: Settings,
}
impl Grid {
    /// instantiate a grid from input ascii text
    pub fn from_str(s: &str, settings: Settings) -> Grid {
        let lines: Vec<&str> = s.lines().collect();
        let mut line_gchars = Vec::with_capacity(lines.len());
        
        for line in lines{
            let mut gchars = Vec::with_capacity(line.len());
            for ch in line.chars(){
                if ch.width() == Some(0) {
                }
                else{
                    gchars.push(GChar::new(ch));
                }
            } 
            line_gchars.push(gchars);
        }
        let mut max = 0;
        for lg in &line_gchars{
            let mut line_width = 0;
            for gchar in lg{
                line_width += gchar.width; 
            } 
            if line_width >= max{
                max = line_width;
            }
        }

        Grid {
            rows: line_gchars.len(),
            columns: max,
            lines: line_gchars,
            settings: settings
        }
    }

    fn get(&self, loc: &Loc) -> Option<&GChar> {
        match self.lines.get(loc.y as usize) {
            Some(line) => {
                let mut total_width = 0;
                for gchar in line{
                    if total_width == loc.x{
                        return Some(gchar)
                    }
                    total_width += gchar.width as isize;
                }
                None
            }
            None => None,
        }
    }



    



    /// get the elements on this location
    /// variable names:
    /// the grid is 8x8 divided into 4 equal parts at each vertical and horizontal dimension.
    /// a,b,c,d,e  is start,quater,center,3quarters, end respectively
    ///
    /// combining [a,b,c,d,e] * h]
    /// ah,bh,ch,dh,eh are horizontal increments derived from dividing the textwidth into 4 equal parts.
    ///
    /// combining [a,b,c,d,e] * [v]
    /// av,bv,cv,dv,ev are vertical increments derived from diving the textheight into 4 equal parts
    ///
    /// combining [a,b,c,d,e] * [x] and [a,b,c,d,e] * [y]
    /// and you will get the location of the points in the grid that describe the relative location
    /// of the point from the starting location of the elements
    /// all intersection and junction points fall exactly to any of the grid points
    ///
    fn get_elements(&self, x: isize, y: isize) -> Option<Vec<Element>> {
        let ch = self.get_focus_char(x, y);
        match ch{
            Some(ch) => {
                Some(ch.get_elements())
            },
            None => None
        }
    }

    fn get_focus_char(&self, x: isize, y: isize) -> Option<FocusChar> {
        let loc = Loc::new(x,y);
        let gch = self.get(&loc);
        match gch{
            Some(gch) => {
                if gch.string.chars().count() > 1{
                    println!("gch: {:?}", gch);
                }
                assert_eq!(gch.string.chars().count(), 1);
                let fc = FocusChar{
                        loc: loc,
                        grid: self
                    };
                Some(fc)
            },
            None => None
        }
    }

    fn get_all_elements(&self) -> Vec<(Loc, Vec<Element>)> {
        fn get_line_width(line: &Vec<GChar>) -> usize{
            let mut total_width = 0;
            for gch in line{
               total_width += gch.width;
            }
            total_width
        }
        let mut all_paths = vec![];
        for row in 0..self.lines.len() {
            let line = &self.lines[row];
            let line_width = get_line_width(line);
            for column in 0..line_width {
                let x = column as isize;
                let y = row as isize;
                match self.get_elements(x, y) {
                    Some(paths) => {
                        all_paths.push((Loc::new(x, y), paths));
                    }
                    None => {
                        ();
                    }
                }
            }
        }
        all_paths
    }

    // each component has its relative location retain
    // use this info for optimizing svg by checking closest neigbor
    fn get_svg_nodes(&self, settings: &Settings) -> Vec<SvgElement> {
        let mut nodes = vec![];
        let elements = self.get_all_elements();
        let input = if settings.optimize {
            let optimizer = Optimizer::new(elements);
            let optimized_elements = optimizer.optimize(settings);
            optimized_elements
        } else {
            elements.into_iter().flat_map(|(_, elm)| elm).collect()
        };
        for elem in input {
            let element = elem.to_svg(settings);
            nodes.push(element);
        }
        nodes
    }


    /// get the generated svg according to the settings specified
    pub fn get_svg(&self) -> SVG {
        let nodes = self.get_svg_nodes(&self.settings);
        let width = self.settings.text_width * (self.columns + 4) as f32;
        let height = self.settings.text_height * (self.rows + 2)as f32;
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

