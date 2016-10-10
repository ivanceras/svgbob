extern crate svg;

use svg::Node;
use svg::node::element::Path as SvgPath;
use svg::node::element::Line as SvgLine;
use svg::node::element::Text as SvgText;
use svg::node::element::Style;
use svg::node::element::SVG;
use svg::node::element::Definitions;
use svg::node::element::Marker;
use optimizer::Optimizer;

mod optimizer;

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

#[derive(Debug)]
#[derive(Clone)]
pub enum Feature {
    Arrow,
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

    /// get the 8 neighbors
    pub fn neighbors(&self) -> Vec<Loc> {
        vec![self.top(), 
             self.bottom(),
             self.left(),
             self.right(),
             self.top_left(),
             self.top_right(),
             self.bottom_left(),
             self.bottom_right(),
            ]
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Element {
    Line(Point, Point, Stroke, Option<Feature>),
    Arc(Point, Point, f32, bool),
    Text(Loc, String),
    Path(Point, Point, String, Stroke),
}

impl Element {
    fn solid_line(s: &Point, e: &Point) -> Element {
        Element::line(s, e, Stroke::Solid, None)
    }

    fn line(s: &Point, e: &Point, stroke: Stroke, feature: Option<Feature>) -> Element {
        Element::Line(s.clone(), e.clone(), stroke, feature)
    }
    fn arc(s: &Point, e: &Point, radius: f32, sweep: bool) -> Element {
        Element::Arc(s.clone(), e.clone(), radius, sweep)
    }
    // this path can chain to the other path
    // chain means the paths can be arranged and express in path definition
    // if self.end == path.start
    fn chain(&self, other: &Element) -> Option<Vec<Element>> {
        match *self {
            Element::Line(_, ref e, ref stroke, ref feature) => {
                match *other {
                    Element::Line(ref s2, _, ref stroke2, _) => {
                        if e == s2 && stroke == stroke2 //must have same stroke and no feature
                       && feature.is_none()
                        // no arrow on the first
                        {
                            Some(vec![self.clone(), other.clone()])
                        } else {
                            None
                        }
                    }
                    Element::Arc(ref s2, _, _, _) => {
                        if e == s2 && feature.is_none() {
                            Some(vec![self.clone(), other.clone()])
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            Element::Arc(_, ref e, _, _) => {
                match *other {
                    Element::Line(ref s2, _, ref stroke2, _) => {
                        match *stroke2 {
                            Stroke::Solid => {
                                // arcs are always solid, so match only solid line to arc
                                if e == s2 {
                                    Some(vec![self.clone(), other.clone()])
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    }
                    Element::Arc(ref s2, _, _, _) => {
                        if e == s2 {
                            Some(vec![self.clone(), other.clone()])
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            Element::Text(_, _) => {
                // text can reduce, but not chain
                None
            }
            Element::Path(_, _, _,_) => None,
        }
    }

    // if this element can reduce the other, return the new reduced element
    // for line it has to be collinear and in can connect start->end->start
    // for text, the other text should apear on the right side of this text
    fn reduce(&self, other: &Element) -> Option<Element> {
        match *self {
            Element::Line(ref s, ref e, ref stroke, ref feature) => {
                match *other {
                    Element::Line(ref s2, ref e2, ref stroke2, ref feature2) => {
                        // note* dual 3 point check for trully collinear lines
                        if collinear(s, e, s2) && collinear(s, e, e2) && e == s2 &&
                           stroke == stroke2 && feature.is_none() {
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
                        let len = text.len() as isize;
                        if loc.y == loc2.y && loc.x + len == loc2.x {
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
            Element::Line(ref s, ref e, ref stroke, ref feature) => {
                let mut svg_line = SvgLine::new()
                    .set("x1", s.x)
                    .set("y1", s.y)
                    .set("x2", e.x)
                    .set("y2", e.y);

                match *feature {
                    Some(Feature::Arrow) => {
                        svg_line.assign("marker-end", "url(#triangle)");
                    }
                    None => (),
                };
                match *stroke {
                    Stroke::Solid => (),
                    Stroke::Dashed => {
                        svg_line.assign("stroke-dasharray", (3, 3));
                    }
                };

                SvgElement::Line(svg_line)
            }
            Element::Arc(ref s, ref e, radius, sweep) => {
                let sweept = if sweep { "1" } else { "0" };
                let d = format!("M {} {} A {} {} 0 0 {} {} {}",
                                s.x,
                                s.y,
                                radius,
                                radius,
                                sweept,
                                e.x,
                                e.y);
                let svg_arc = SvgPath::new()
                    .set("d", d)
                    .set("fill", "none");
                SvgElement::Path(svg_arc)
            }
            Element::Text(ref loc, ref string) => {
                let sx = loc.x as f32 * settings.text_width + settings.text_width / 4.0;
                let sy = loc.y as f32 * settings.text_height + settings.text_height * 3.0 / 4.0;
                let mut svg_text = SvgText::new()
                    .set("x", sx)
                    .set("y", sy);
                let text_node = svg::node::Text::new(string.to_owned());
                svg_text.append(text_node);
                SvgElement::Text(svg_text)
            }
            Element::Path(_, _, ref d, ref stroke) => {
                let mut path = SvgPath::new()
                    .set("d", d.to_owned())
                    .set("fill", "none");

                match *stroke {
                    Stroke::Solid => (),
                    Stroke::Dashed => {
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
pub struct Grid {
    rows: usize,
    columns: usize,
    lines: Vec<Vec<char>>,
}
impl Grid {
    pub fn from_str(s: &str) -> Grid {
        let lines: Vec<Vec<char>> = s.split("\n")
            .map(|l| l.trim_right().chars().collect())
            .collect();

        let max = lines.iter()
            .fold(0, |acc, ref x| if x.len() > acc { x.len() } else { acc });

        Grid {
            rows: lines.len(),
            columns: max,
            lines: lines,
        }
    }

    fn get(&self, loc: &Loc) -> Option<&char> {
        match self.lines.get(loc.y as usize) {
            Some(line) => line.get(loc.x as usize),
            None => None,
        }
    }


    fn is_char<F>(&self, loc: &Loc, f: F) -> bool
        where F: Fn(&char) -> bool {
        let ch = self.get(loc);
        match ch {
            Some(ch) => f(ch),
            None => false,
        }
    }
    
    // if it is a simple piece of drawing element,
    // make sure it is next to an other draing element
    // else it will be treated as text
    fn next_to_drawing_element(&self, loc: &Loc) -> bool {
        loc.neighbors().iter()
            .find(|&x| self.is_char(x, is_drawing_element) )
                .map_or(false, |_| true)
    }
    
    // determine if the character in this location is a drawing element
    // but is used as text, such as a == b, cd to/path/file
    // it is used as text when it is a drawing element, and left and righ is alphanumeric
    fn used_as_text(&self, loc: &Loc) -> bool {
        self.is_char(loc, is_drawing_element)
            && 
            (self.is_char(&loc.left(), |&c| c.is_alphanumeric())
            || self.is_char(&loc.right(), |&c| c.is_alphanumeric())
            )
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
    fn get_elements(&self, x: isize, y: isize, settings: &Settings) -> Option<Vec<Element>> {
        let text_width = settings.text_width;
        let text_height = settings.text_height;
        let measurex = x as f32 * text_width;
        let measurey = y as f32 * text_height;
        let arc_radius = text_width / 2.0;

        // horizontal divide
        let ah = 0.0;
        let bh = text_width / 4.0;
        let ch = text_width / 2.0;
        let dh = text_width * 3.0 / 4.0;
        let eh = text_width;

        // vertical divide
        let av = 0.0;
        let bv = text_height / 4.0;
        let cv = text_height / 2.0;
        let dv = text_height * 3.0 / 4.0;
        let ev = text_height;

        let ax = measurex + ah;
        let ay = measurey + av;
        let bx = measurex + bh;
        let by = measurey + bv;
        let cx = measurex + ch;
        let cy = measurey + cv;
        let dx = measurex + dh;
        let dy = measurey + dv;
        let ex = measurex + eh;
        let ey = measurey + ev;


        // point locations
        let center_top = &Point::new(cx, ay);
        let center_bottom = &Point::new(cx, ey);
        let mid_left = &Point::new(ax, cy);
        let mid_right = &Point::new(ex, cy);
        let high_left = &Point::new(ax, ay);
        let high_right = &Point::new(ex, ay);
        let low_left = &Point::new(ax, ey);
        let low_right = &Point::new(ex, ey);

        // point 5x5 locations
        let axay = &Point::new(ax, ay);
        let bxby = &Point::new(bx, by);
        let cxcy = &Point::new(cx, cy);
        let dxdy = &Point::new(dx, dy);
        let exey = &Point::new(ex, ey);

        let axcy = &Point::new(ax, cy);
        let axey = &Point::new(ax, ey);
        let bxdy = &Point::new(bx, dy);
        let bxcy = &Point::new(bx, cy);
        let cxay = &Point::new(cx, ay);
        let cxey = &Point::new(cx, ey);
        let cxdy = &Point::new(cx, dy);
        let cxby = &Point::new(cx, by);
        let dxby = &Point::new(dx, by);
        let dxcy = &Point::new(dx, cy);
        let excy = &Point::new(ex, cy);
        let exay = &Point::new(ex, ay);
        let dxey = &Point::new(dx, ey);
        let dxay = &Point::new(dx, ay);
        let bxay = &Point::new(bx, ay);
        let bxey = &Point::new(bx, ey);

        //extended points
        let axbhey = &Point::new(ax - bh, ey);
        let exbhey = &Point::new(ex + bh, ey);
        let axbhay = &Point::new(ax - bh, ay);
        let exbhay = &Point::new(ex + bh, ay);


        // grid lines
        let axay_bxby = Element::solid_line(axay, bxby);
        let cxcy_axcy = Element::solid_line(cxcy, axcy);
        let cxcy_cxay = Element::solid_line(cxcy, cxay);
        let cxcy_cxey = Element::solid_line(cxcy, cxey);
        let cxcy_excy = Element::solid_line(cxcy, excy);

        let cxdy_cxey = Element::solid_line(cxdy, cxey);
        let cxay_cxby = Element::solid_line(cxay, cxby);
        let dxby_exay = Element::solid_line(dxby, exay);
        let axey_bxdy = Element::solid_line(axey, bxdy);
        let exey_dxdy = Element::solid_line(exey, dxdy);
        let dxcy_excy = Element::solid_line(dxcy, excy);
        let bxcy_axcy = Element::solid_line(bxcy, axcy);
        let exay_dxby = Element::solid_line(exay, dxby);
        let cxey_cxdy = Element::solid_line(cxey, cxdy);
        let dxdy_exey = Element::solid_line(dxdy, exey);

        // common arc
        let arc_axcy_cxby = Element::arc(axcy, cxby, arc_radius, false);
        let arc_axcy_dxby = Element::arc(axcy, dxby, arc_radius * 2.0, false);
        let arc_bxby_excy = Element::arc(bxby, excy, arc_radius * 2.0, false);
        let arc_axcy_bxby = Element::arc(axcy, bxby, arc_radius, false);
        let arc_cxdy_axcy = Element::arc(cxdy, axcy, arc_radius, false);
        let arc_cxby_excy = Element::arc(cxby, excy, arc_radius, false);
        let arc_dxdy_axcy = Element::arc(dxdy, axcy, arc_radius * 2.0, false);
        let arc_excy_cxdy = Element::arc(excy, cxdy, arc_radius, false);
        let arc_excy_bxdy = Element::arc(excy, bxdy, arc_radius * 2.0, false);
        let arc_dxby_excy = Element::arc(dxby, excy, arc_radius, false);
        let arc_bxdy_axcy = Element::arc(bxdy, axcy, arc_radius, false);
        let arc_excy_dxdy = Element::arc(excy, dxdy, arc_radius, false);
        let arc_dxcy_bxdy = Element::arc(dxcy, bxdy, arc_radius * 2.0, false);
        let arc_bxcy_dxby = Element::arc(bxcy, dxby, arc_radius * 2.0, false);
        let arc_dxdy_bxcy = Element::arc(dxdy, bxcy, arc_radius * 2.0, false);
        let arc_bxby_dxcy = Element::arc(bxby, dxcy, arc_radius * 2.0, false);
        let arc_bxdy_bxby = Element::arc(bxdy, bxby, arc_radius * 2.0, false);
        let arc_dxby_dxdy = Element::arc(dxby, dxdy, arc_radius * 2.0, false);
        let arc_dxby_cxdy = Element::arc(dxby, cxdy, arc_radius * 4.0, false);
        let arc_bxdy_cxby = Element::arc(bxdy, cxby, arc_radius * 4.0, false);
        let arc_cxby_dxdy = Element::arc(cxby, dxdy, arc_radius * 4.0, false);
        let arc_cxdy_bxby = Element::arc(cxdy, bxby, arc_radius * 4.0, false);
        let arc_dxay_dxey = Element::arc(dxay, dxey, arc_radius * 4.0, false);
        let arc_bxey_bxay = Element::arc(bxey, bxay, arc_radius * 4.0, false);
        let arc_excy_axcy = Element::arc(excy, axcy, arc_radius * 4.0, false);
        let arc_axcy_excy = Element::arc(axcy, excy, arc_radius * 4.0, false);

        //extended arc
        let arc_excy_axbhey = Element::arc(excy, axbhey, arc_radius * 4.0, false);
        let arc_exbhey_axcy = Element::arc(exbhey, axcy, arc_radius * 4.0, false);
        let arc_axbhay_excy = Element::arc(axbhay, excy, arc_radius * 4.0, false);
        let arc_axcy_exbhay = Element::arc(axcy, exbhay, arc_radius * 4.0, false);

        // common path lines
        let vertical = Element::solid_line(center_top, center_bottom);
        let horizontal = Element::solid_line(mid_left, mid_right);
        let slant_left = Element::solid_line(high_left, low_right);
        let slant_right = Element::solid_line(low_left, high_right);
        let low_horizontal = Element::solid_line(low_left, low_right);

        // extended lines
        let low_horizontal_extend_left_half = Element::solid_line(low_right,
                                                                  &Point::new(ax - ch, ey));
        let low_horizontal_extend_right_half = Element::solid_line(low_left,
                                                                   &Point::new(ex + ch, ey));
        let low_horizontal_extend_left_full = Element::solid_line(low_right,
                                                                  &Point::new(ax - eh, ey));
        let low_horizontal_extend_right_full = Element::solid_line(low_left,
                                                                   &Point::new(ex + eh, ey));

        // dashed lines
        let vertical_dashed = Element::line(center_top, center_bottom, Stroke::Dashed, None);
        let horizontal_dashed = Element::line(mid_left, mid_right, Stroke::Dashed, None);
        let low_horizontal_dashed = Element::line(low_left, low_right, Stroke::Dashed, None);

        let arrow_down = Element::line(center_top,
                                       center_bottom,
                                       Stroke::Solid,
                                       Some(Feature::Arrow));
        let arrow_down_dashed = Element::line(center_top,
                                              center_bottom,
                                              Stroke::Dashed,
                                              Some(Feature::Arrow));
        let arrow_up = Element::line(center_bottom,
                                     center_top,
                                     Stroke::Solid,
                                     Some(Feature::Arrow));
        let arrow_up_dashed = Element::line(center_bottom,
                                            center_top,
                                            Stroke::Dashed,
                                            Some(Feature::Arrow));
        let arrow_left = Element::line(mid_right, cxcy, Stroke::Solid, Some(Feature::Arrow));
        let arrow_left_dashed =
            Element::line(mid_right, cxcy, Stroke::Dashed, Some(Feature::Arrow));
        let arrow_right = Element::line(mid_left, cxcy, Stroke::Solid, Some(Feature::Arrow));
        let arrow_right_dashed =
            Element::line(mid_left, cxcy, Stroke::Dashed, Some(Feature::Arrow));
        let arrow_bottom_left =
            Element::line(high_right, cxcy, Stroke::Solid, Some(Feature::Arrow));
        let arrow_bottom_right =
            Element::line(high_left, cxcy, Stroke::Solid, Some(Feature::Arrow));
        let arrow_top_left = Element::line(low_right, cxcy, Stroke::Solid, Some(Feature::Arrow));
        let arrow_top_right = Element::line(low_left, cxcy, Stroke::Solid, Some(Feature::Arrow));

        // relative location of characters
        let this = &Loc::new(x, y);
        let top = &this.top();
        let left = &this.left();
        let bottom = &this.bottom();
        let right = &this.right();
        let top_left = &this.top_left();
        let top_right = &this.top_right();
        let bottom_left = &this.bottom_left();
        let bottom_right = &this.bottom_right();

        // left of left
        let left_left = &this.left().left();
        let right_right = &this.right().right();


        let match_list: Vec<(bool, Vec<Element>)> = 
            vec![
                /*
                    |
                */
                (self.is_char(this, is_vertical) && !self.used_as_text(this),
                 vec![vertical.clone()]
                ),
                /*
                    -
                */
                (self.is_char(this, is_horizontal) && !self.used_as_text(this),
                 vec![horizontal.clone()]
                ),

                /*
                    _
                */
                (self.is_char(this, is_low_horizontal) && !self.used_as_text(this),
                 vec![low_horizontal.clone()]
                ),
                /*
                   :
                   :
                   must have at least 1 align to it to be treated as vertical 
                */
                (self.is_char(this, is_vertical_dashed)
                  && (self.is_char(top, is_vertical_dashed)
                     || self.is_char(bottom, is_vertical_dashed)),
                  vec![vertical_dashed.clone()]
                ),

                /*
                   ==  at least 2 next to it
                */
                (self.is_char(this, is_horizontal_dashed)
                  && ((self.is_char(left, is_horizontal_dashed)
                     && self.is_char(right, is_horizontal_dashed)
                     )
                     ||
                     (self.is_char(left, is_horizontal_dashed)
                     && self.is_char(left_left, is_horizontal_dashed)
                     )
                     ||
                     (self.is_char(right, is_horizontal_dashed)
                      && self.is_char(right_right, is_horizontal_dashed)
                     )
                     ),
                  vec![horizontal_dashed.clone()]
                ),
                /*
                   ...  at least 2 next to it
                */
                (self.is_char(this, is_low_horizontal_dashed)
                  && ((self.is_char(left, is_low_horizontal_dashed) //left & right
                      && self.is_char(right, is_low_horizontal_dashed)
                      )
                     || 
                      (self.is_char(left, is_low_horizontal_dashed)
                       && self.is_char(left_left, is_low_horizontal_dashed)
                      )
                     || 
                      (self.is_char(right, is_low_horizontal_dashed)
                       && self.is_char(right_right, is_low_horizontal_dashed)
                      )
                     ),
                  vec![low_horizontal_dashed.clone()]
                ),
                /*
                    /
                */
                (self.is_char(this, is_slant_right) && !self.used_as_text(this),
                 vec![slant_right.clone()]
                ),
                /*
                    \
                */
                (self.is_char(this, is_slant_left) && !self.used_as_text(this),
                 vec![slant_left.clone()]
                ),

                /*
                    ^
                    |
                */
                (self.is_char(this, is_arrow_up)
                 && self.is_char(bottom, is_vertical),
                 vec![arrow_up.clone()]
                ),
                /*
                    ^
                    :
                */
                (self.is_char(this, is_arrow_up)
                 && self.is_char(bottom, is_vertical_dashed),
                 vec![arrow_up_dashed.clone()]
                ),
                /*
                    |
                    V
                */
                (self.is_char(this, is_arrow_down)
                 && self.is_char(top, is_vertical),
                 vec![arrow_down.clone()]
                ),

                /*
                    :
                    V
                */
                (self.is_char(this, is_arrow_down)
                 && self.is_char(top, is_vertical_dashed),
                 vec![arrow_down_dashed.clone()]
                ),
                /*
                    <-
                     
                */
                (self.is_char(this, is_arrow_left)
                 && self.is_char(right, is_horizontal),
                 vec![arrow_left.clone()]
                ),
                /*
                    <=
                     
                */
                (self.is_char(this, is_arrow_left)
                 && self.is_char(right, is_horizontal_dashed),
                 vec![arrow_left_dashed.clone()]
                ),
                /*
                    ->
                     
                */
                (self.is_char(this, is_arrow_right)
                 && self.is_char(left, is_horizontal),
                 vec![arrow_right.clone()]
                ),
                /*
                    =>
                     
                */
                (self.is_char(this, is_arrow_right)
                 && self.is_char(left, is_horizontal_dashed),
                 vec![arrow_right_dashed.clone()]
                ),
                /*
                    ^
                     \
                */
                (self.is_char(this, is_arrow_up)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![arrow_top_left.clone()]
                ),
                /*
                      ^
                     /
                */
                (self.is_char(this, is_arrow_up)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![arrow_top_right.clone()]
                ),
                /*
                      /
                     V 
                */
                (self.is_char(this, is_arrow_down)
                 && self.is_char(top_right, is_slant_right),
                 vec![arrow_bottom_left.clone()]
                ),
                /*
                      \
                       V 
                */
                (self.is_char(this, is_arrow_down)
                 && self.is_char(top_left, is_slant_left),
                 vec![arrow_bottom_right.clone()]
                ),
                /*
                       _  or |_
                      |
                */
                (self.is_char(this, is_low_horizontal)
                 && (self.is_char(bottom_left, is_vertical)
                    || self.is_char(left, is_vertical)
                    ),
                 vec![low_horizontal_extend_left_half.clone()]
                ),
                /*
                       _  or _|
                        |
                */
                (self.is_char(this, is_low_horizontal)
                 && (self.is_char(bottom_right, is_vertical)
                    || self.is_char(right, is_vertical)
                    ),
                 vec![low_horizontal_extend_right_half.clone()]
                ),
                /*
                       /_
                     
                */
                (self.is_char(this, is_low_horizontal)
                 && self.is_char(left, is_slant_right),
                 vec![low_horizontal_extend_left_full.clone()]
                ),
                /*
                       _\
                     
                */
                (self.is_char(this, is_low_horizontal)
                 && self.is_char(right, is_slant_left),
                 vec![low_horizontal_extend_right_full.clone()]
                ),
                /*
                      +-
                      | 
                */
                (self.is_char(this, is_intersection)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom, is_vertical),
                 vec![cxcy_cxey.clone(), cxcy_excy.clone()]
                ),
                /*
                     -+
                      | 
                */
                (self.is_char(this, is_intersection)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(bottom, is_vertical),
                 vec![cxcy_cxey.clone(), cxcy_axcy.clone()]
                ),
                /*
                     |
                     +-
                */
                (self.is_char(this, is_intersection)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top, is_vertical),
                 vec![cxcy_cxay.clone(), cxcy_excy.clone()]
                ),
                /*
                     |
                    -+
                */
                (self.is_char(this, is_intersection)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top, is_vertical),
                 vec![cxcy_cxay.clone(), cxcy_axcy.clone()]
                ),
                /*
                      .-
                      | 
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom, is_vertical),
                 vec![cxdy_cxey.clone(), arc_excy_cxdy.clone()]
                ),
                /*
                      -.
                       | 
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(bottom, is_vertical),
                 vec![cxdy_cxey.clone(), arc_cxdy_axcy.clone()]
                ),
                /*
                     | 
                     '- 
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top, is_vertical),
                 vec![cxay_cxby.clone(), arc_cxby_excy.clone()]
                ),
                /*
                     | 
                    -' 
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top, is_vertical),
                 vec![cxay_cxby.clone(), arc_axcy_cxby.clone()]
                ),
                /*
                    .-  
                   / 
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![axey_bxdy.clone(), arc_excy_bxdy.clone()]
                ),
                /*
                   -.  
                     \ 
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![exey_dxdy.clone(), arc_dxdy_axcy.clone()]
                ),
                /*
                   -.  
                   / 
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![axey_bxdy.clone(), arc_bxdy_axcy.clone()]
                ),
                /*
                   .-
                    \
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![exey_dxdy.clone(), arc_excy_dxdy.clone()]
                ),
                /*
                   \  
                    '-  
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top_left, is_slant_left),
                 vec![axay_bxby.clone(), arc_bxby_excy.clone()]
                ),
                /*
                     / 
                    '-  
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top_right, is_slant_right),
                 vec![dxby_exay.clone(), arc_dxby_excy.clone()]
                ),
                /*
                    \
                    -'
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top_left, is_slant_left),
                 vec![axay_bxby.clone(), arc_axcy_bxby.clone()]
                ),
                /*
                      /
                    -'
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top_right, is_slant_right),
                 vec![dxby_exay.clone(), arc_axcy_dxby.clone()]
                ),
                /*
                    \       \
                     .  or   )
                    /       /
                */
                ((self.is_char(this, is_round) || self.is_char(this, is_close_curve))
                 && self.is_char(top_left, is_slant_left)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![axay_bxby.clone(),axey_bxdy.clone(), arc_bxdy_bxby.clone()]
                ),
                /*
                      /       /
                     .  or   (
                      \       \
                */
                ((self.is_char(this, is_round) || self.is_char(this, is_open_curve))
                 && self.is_char(top_right, is_slant_right)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![exay_dxby.clone(),exey_dxdy.clone(), arc_dxby_dxdy.clone()]
                ),
                /*
                      .
                     (
                      '
                */
                (self.is_char(this, is_open_curve) 
                 && self.is_char(top_right, is_round)
                 && self.is_char(bottom_right, is_round),
                 vec![arc_dxay_dxey.clone()]
                ),
                /*
                      .
                       ) 
                      '
                */
                (self.is_char(this, is_close_curve) 
                 && self.is_char(top_left, is_round)
                 && self.is_char(bottom_left, is_round),
                 vec![arc_bxey_bxay.clone()]
                ),
                /*
                      .-
                     (
                */
                (self.is_char(this, is_round) 
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom_left, is_open_curve),
                 vec![arc_excy_axbhey.clone()]
                ),
                /*
                       -.
                         ) 
                */
                (self.is_char(this, is_round) 
                 && self.is_char(left, is_horizontal)
                 && self.is_char(bottom_right, is_close_curve),
                 vec![arc_exbhey_axcy.clone()]
                ),
                /*
                    (  
                     '- 
                */
                (self.is_char(this, is_round) 
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top_left, is_open_curve),
                 vec![arc_axbhay_excy.clone()]
                ),
                /*
                        ) 
                      -'
                */
                (self.is_char(this, is_round) 
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top_right, is_close_curve),
                 vec![arc_axcy_exbhay.clone()]
                ),
                /*
                     .- 
                     ' 
                */
                (self.is_char(this, is_low_round) 
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom, is_high_round),
                 vec![arc_excy_cxdy.clone(), cxdy_cxey.clone()]
                ),
                /*
                     -.
                      ' 
                */
                (self.is_char(this, is_low_round) 
                 && self.is_char(left, is_horizontal)
                 && self.is_char(bottom, is_high_round),
                 vec![arc_cxdy_axcy.clone(),cxdy_cxey.clone()]
                ),
                /*
                     . 
                     '-
                */
                (self.is_char(this, is_high_round) 
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top, is_low_round),
                 vec![arc_cxby_excy.clone(), cxay_cxby.clone()]
                ),
                /*
                       . 
                      -'
                */
                (self.is_char(this, is_high_round) 
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top, is_low_round),
                 vec![arc_axcy_cxby.clone(), cxay_cxby.clone()]
                ),
                /*
                      .-.  
                     (    
                */
                (self.is_char(this, is_horizontal) 
                 && self.is_char(left, is_low_round)
                 && self.is_char(right, is_low_round)
                 && self.is_char(&this.bottom_left().left(), is_open_curve),
                 vec![arc_excy_axcy.clone()]
                ),
                /*         
                     (
                      '-' 
                */
                (self.is_char(this, is_horizontal) 
                 && self.is_char(left, is_high_round)
                 && self.is_char(right, is_high_round)
                 && self.is_char(&this.top_left().left(), is_open_curve),
                 vec![arc_axcy_excy.clone()]
                ),
                /*
                      / 
                     .  
                     |
                */
                (self.is_char(this, is_round)
                 && self.is_char(bottom, is_vertical)
                 && self.is_char(top_right, is_slant_right),
                 vec![exay_dxby.clone(),cxey_cxdy.clone(), arc_dxby_cxdy.clone()]
                ),
                /*
                     | 
                     .  
                    /
                */
                (self.is_char(this, is_round)
                 && self.is_char(top, is_vertical)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![cxay_cxby.clone(), axey_bxdy.clone(), arc_bxdy_cxby.clone()]
                ),
                /*
                    \
                     .  
                     | 
                */
                (self.is_char(this, is_round)
                 && self.is_char(bottom, is_vertical)
                 && self.is_char(top_left, is_slant_left),
                 vec![axay_bxby.clone(), cxdy_cxey.clone(), arc_cxdy_bxby.clone()]
                ),
                /*
                     |
                     .  
                      \ 
                */
                (self.is_char(this, is_round)
                 && self.is_char(top, is_vertical)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![cxay_cxby.clone(), dxdy_exey.clone(), arc_cxby_dxdy.clone()]
                ),
                /*
                     /
                    .-
                   /
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top_right, is_slant_right)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![slant_right.clone(), dxcy_excy.clone(), arc_dxcy_bxdy.clone()]
                ),
                /*
                     /
                   -.
                   /
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top_right, is_slant_right)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![slant_right.clone(), bxcy_axcy.clone(), arc_bxcy_dxby.clone()]
                ),
                /*
                    \
                    -.
                      \
                   
                */
                (self.is_char(this, is_round)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top_left, is_slant_left)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![slant_left.clone(),bxcy_axcy.clone(), arc_dxdy_bxcy.clone()]
                ),
                /*
                    \
                     .-
                      \
                   
                */
                (self.is_char(this, is_round)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top_left, is_slant_left)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![slant_left.clone(),dxcy_excy.clone(), arc_bxby_dxcy.clone()]
                ),
                /*
                     |
                    -+-
                */
                (self.is_char(this, is_intersection)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top, is_vertical),
                 vec![cxcy_cxay.clone(), horizontal.clone()]
                ),
                /*
                    -+-
                     |
                */
                (self.is_char(this, is_intersection)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(bottom, is_vertical),
                 vec![cxcy_cxey.clone(), horizontal.clone()]
                ),
                /*
                     |
                    -+
                     |
                */
                (self.is_char(this, is_intersection)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(top, is_vertical)
                 && self.is_char(bottom, is_vertical),
                 vec![vertical.clone(), cxcy_axcy.clone()]
                ),
                /*
                     |
                     +-
                     |
                */
                (self.is_char(this, is_intersection)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top, is_vertical)
                 && self.is_char(bottom, is_vertical),
                 vec![vertical.clone(), cxcy_excy.clone()]
                ),
                /*
                     | 
                     .  
                    /|
                */
                (self.is_char(this, is_round)
                 && self.is_char(bottom, is_vertical)
                 && self.is_char(top, is_vertical)
                 && self.is_char(bottom_left, is_slant_right),
                 vec![vertical.clone(), axey_bxdy.clone(), arc_bxdy_cxby.clone()]
                ),
                /*
                     | 
                     .  
                     |\
                */
                (self.is_char(this, is_round)
                 && self.is_char(bottom, is_vertical)
                 && self.is_char(top, is_vertical)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![vertical.clone(), exey_dxdy.clone(), arc_cxby_dxdy.clone()]
                ),
                /*
                    |  
                   -+-
                    | 
                */
                ((self.is_char(this, is_intersection) || self.is_char(this, is_round) || self.is_char(this, is_marker))
                 && self.is_char(top, is_vertical)
                 && self.is_char(bottom, is_vertical)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(right, is_horizontal),
                 vec![vertical.clone(), horizontal.clone()]
                ),
                /*
                    :  
                   =+=
                    : 
                */
                ((self.is_char(this, is_intersection) || self.is_char(this, is_round) || self.is_char(this, is_marker))
                 && self.is_char(top, is_vertical_dashed)
                 && self.is_char(bottom, is_vertical_dashed)
                 && self.is_char(left, is_horizontal_dashed)
                 && self.is_char(right, is_horizontal_dashed),
                 vec![vertical_dashed.clone(), horizontal_dashed.clone()]
                ),
                /*
                   \|/ 
                    + 
                   /|\
                */
                ((self.is_char(this, is_intersection) || self.is_char(this, is_round) || self.is_char(this, is_marker))
                 && self.is_char(top, is_vertical)
                 && self.is_char(bottom, is_vertical)
                 && self.is_char(top_left, is_slant_left)
                 && self.is_char(top_right, is_slant_right)
                 && self.is_char(bottom_left, is_slant_right)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![vertical.clone(), slant_left.clone(), slant_right.clone()]
                ),
                /*
                   \|/ 
                   -+-
                   /|\
                */
                ((self.is_char(this, is_intersection) || self.is_char(this, is_round) || self.is_char(this, is_marker))
                 && self.is_char(top, is_vertical)
                 && self.is_char(bottom, is_vertical)
                 && self.is_char(left, is_horizontal)
                 && self.is_char(right, is_horizontal)
                 && self.is_char(top_left, is_slant_left)
                 && self.is_char(top_right, is_slant_right)
                 && self.is_char(bottom_left, is_slant_right)
                 && self.is_char(bottom_right, is_slant_left),
                 vec![vertical.clone(), horizontal.clone(), slant_left.clone(), slant_right.clone()]
                ),
            ];
        let match_path: Option<(bool, Vec<Element>)> = match_list.into_iter()
            .rev()
            .find(|x| {
                let &(cond, _) = x;
                cond
            });

        let paths: Option<Vec<Element>> = match match_path {
            Some((_, paths)) => Some(paths),
            None => {
                let ch = self.get(this);
                match ch {
                    Some(ch) => {
                        if !ch.is_whitespace() ||
                           (*ch == ' ' && self.is_char(left, |c| c.is_alphanumeric()) &&
                            self.is_char(right, |c| c.is_alphanumeric())) {
                            let s = escape_char(ch);
                            let text = Element::Text(Loc::new(x, y), s);
                            Some(vec![text])
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            }
        };

        paths

    }

    fn get_all_elements(&self, settings: &Settings) -> Vec<(Loc, Vec<Element>)> {
        let mut all_paths = vec![];
        for row in 0..self.lines.len() {
            let line = &self.lines[row];
            for column in 0..line.len() {
                let x = column as isize;
                let y = row as isize;
                match self.get_elements(x, y, settings) {
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
        let elements = self.get_all_elements(settings);
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

    pub fn get_svg(&self, settings: &Settings) -> SVG {
        let nodes = self.get_svg_nodes(settings);
        let width = settings.text_width * self.columns as f32;
        let height = settings.text_height * self.rows as f32;
        let mut svg = SVG::new()
            .set("font-size", 14)
            .set("font-family", "Electrolize,Titillium Web, Trebuchet MS, Arial")
            .set("width", width)
            .set("height", height);

        svg.append(get_defs());
        svg.append(get_styles());

        for node in nodes {
            match node {
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
    /* <![CDATA[ */
    line, path {
      stroke: black;
      stroke-width: 1;
    }
 /* ]]> */
    "#;
    Style::new(style)
}

fn arrow_marker() -> Marker {
    let mut marker = Marker::new()
        .set("id", "triangle")
        .set("viewBox", "0 0 14 14")
        .set("refX", 0)
        .set("refY", 5)
        .set("markerUnits", "strokeWidth")
        .set("markerWidth", 10)
        .set("markerHeight", 10)
        .set("orient", "auto");

    let path = SvgPath::new().set("d", "M 0 0 L 10 5 L 0 10 z");
    marker.append(path);
    marker

}

fn is_vertical(ch: &char) -> bool {
    *ch == '|'
}

fn is_horizontal(ch: &char) -> bool {
    *ch == '-'
}

fn is_horizontal_dashed(ch: &char) -> bool {
    *ch == '='
}

fn is_vertical_dashed(ch: &char) -> bool {
    *ch == ':'
}

fn is_low_horizontal(ch: &char) -> bool {
    *ch == '_'
}

fn is_low_horizontal_dashed(ch: &char) -> bool {
    *ch == '.'
}

fn is_slant_left(ch: &char) -> bool {
    *ch == '\\'
}
fn is_slant_right(ch: &char) -> bool {
    *ch == '/'
}

fn is_low_round(ch: &char) -> bool {
    *ch == '.'
}

fn is_high_round(ch: &char) -> bool {
    *ch == '\''
}

fn is_round(ch: &char) -> bool {
    is_low_round(ch) || is_high_round(ch)
}

fn is_intersection(ch: &char) -> bool {
    *ch == '+'
}

fn is_marker(ch: &char) -> bool {
    *ch == '*'
}

fn is_arrow_up(ch: &char) -> bool {
    *ch == '^'
}

fn is_arrow_down(ch: &char) -> bool {
    *ch == 'v' || *ch == 'V'
}

fn is_arrow_left(ch: &char) -> bool {
    *ch == '<'
}

fn is_arrow_right(ch: &char) -> bool {
    *ch == '>'
}

fn is_open_curve(ch: &char) -> bool {
    *ch == '('
}

fn is_close_curve(ch: &char) -> bool {
    *ch == ')'
}


fn is_drawing_element(ch: &char) -> bool{
    [is_vertical,
     is_horizontal,
     is_vertical_dashed,
     is_horizontal_dashed,
     is_low_horizontal,
     is_low_horizontal_dashed,
     is_slant_left,
     is_slant_right,
     is_low_round,
     is_high_round,
     is_round,
     is_intersection,
     is_marker,
     is_arrow_up,
     is_arrow_down,
     is_arrow_left,
     is_arrow_right,
     is_open_curve,
     is_close_curve,
    ].iter().find(|&x| x(ch))
          .map_or(false, |_| true)

}

#[test]
fn test_drawing_element(){
    assert!(is_drawing_element(&'|'));
    assert!(is_drawing_element(&'-'));
    assert!(is_drawing_element(&'='));
    assert!(is_drawing_element(&'_'));
    assert!(is_drawing_element(&'/'));
}

fn escape_char(ch: &char) -> String {
    let escs = [('"', "&quot;"), ('\'', "&apos;"), ('<', "&lt;"), ('>', "&gt;"), ('&', "&amp;")];
    let quote_match: Option<&(char, &str)> = escs.iter()
        .find(|pair| {
            let &(e, _) = *pair;
            e == *ch
        });
    let quoted: String = match quote_match {
        Some(&(_, quoted)) => String::from(quoted),
        None => {
            let mut s = String::new();
            s.push(*ch);
            s
        }
    };
    quoted

}
