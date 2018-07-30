use std::cmp::Ordering;
use svg_element::SvgElement;
use svg::Node;
use svg::node::element::{
    Circle as SvgCircle,
    Line as SvgLine,
    Path as SvgPath,
    Text as SvgText,
};
use grid::svg_escape;
use point::collinear;
use settings::Settings;

use point::Point;
use loc::Loc;
use element::{
    Stroke::{Solid,Dashed},
    ArcFlag::{Minor,Major},
    Feature::{Arrow,ArrowStart,Circle,OpenCircle},
};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, PartialEq, PartialOrd )]
pub enum Element {
    Circle(Point, f32, String),
    Line(Point, Point, Stroke, Vec<Feature>),
    Arc(Point, Point, f32, ArcFlag, bool, Stroke, Vec<Feature>),
    Text(Loc, String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Stroke {
    Solid,
    Dashed,
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Feature {
    ArrowStart, // start arrow
    Arrow,  //end
    Circle, //start
    OpenCircle, //start
}



#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ArcFlag {
    Major,
    Minor,
}

impl Ord for Element{
    fn cmp(&self, other: &Self) -> Ordering{
        if let Some(order) = self.partial_cmp(&other){
            return order
        }
        Ordering::Less
    }
}
impl Eq for Element{
}

pub fn line(a: &Point, b: &Point) -> Element {
    Element::Line(a.clone(), b.clone(), Solid, vec![])
}

pub fn dashed_line(a: &Point, b: &Point) -> Element {
    Element::Line(a.clone(), b.clone(), Dashed, vec![])
}

pub fn circle_start_line(a: &Point, b: &Point) -> Element {
    Element::Line(a.clone(), b.clone(), Solid, vec![Circle])
}

pub fn circle_open_line(a: &Point, b: &Point) -> Element {
    Element::Line(a.clone(), b.clone(), Solid, vec![OpenCircle])
}

pub fn solid_circle(c: &Point, r: f32) -> Element {
    Element::Circle(c.clone(), r, "solid".to_string())
}

pub fn _arrow_arc(a: &Point, b: &Point, r: f32) -> Element {
    Element::Arc(a.clone(), b.clone(), r, Minor, false, Solid, vec![Arrow])
}

pub fn _arrow_sweep_arc(a: &Point, b: &Point, r: f32) -> Element {
    Element::Arc(a.clone(), b.clone(), r.clone(), Minor, true, Solid, vec![Arrow])
}

pub fn arc(a: &Point, b: &Point, r: f32) -> Element {
    Element::Arc(a.clone(), b.clone(), r, Minor, false, Solid, vec![])
}

pub fn _arc_major(a: &Point, b: &Point, r: f32) -> Element {
    Element::Arc(a.clone(), b.clone(), r, Major, false, Solid, vec![])
}

pub fn open_circle(c: &Point, r: f32) -> Element {
    Element::Circle(c.clone(), r.clone(), "open".to_string())
}

pub fn arrow_line(s: &Point, e: &Point) -> Element {
    Element::Line(s.clone(), e.clone(), Solid, vec![Arrow])
}

pub fn start_arrow_line(s: &Point, e: &Point) -> Element {
    Element::Line(s.clone(), e.clone(), Solid, vec![ArrowStart, Arrow,Circle])
}

pub fn text(loc: &Loc, txt: &str) -> Element {
    Element::Text(loc.clone(), svg_escape(txt))
}


impl Element {
    // if this element can reduce the other, return the new reduced element
    // for line it has to be collinear and in can connect start->end->start
    // for text, the other text should apear on the right side of this text
    pub fn reduce(&self, other: &Element) -> Option<Element> {
        // if same then return one
        if self == other{
            return Some(other.clone())
        }
        match *self {
            Element::Line(ref s, ref e, ref stroke, ref feature) => {
                match *other {
                    Element::Line(ref s2, ref e2, ref stroke2, ref feature2) => {
                        // note: dual 3 point check for trully collinear lines
                        if collinear(s, e, s2) 
                            && collinear(s, e, e2) 
                            && stroke == stroke2{
                            // same length line
                            if s == s2 && e == e2 && feature == feature2{
                                return Some(other.clone())
                            }

                            //    line1      line2
                            //   s-----e   s2-----e2
                            //   s----------------e2
                            else if e == s2 {
                                // -----
                                // o----
                                let cond1 = feature.is_empty() || (feature.contains(&Circle) && feature.len() == 1);
                                // ------
                                // ------>
                                let cond2 = feature2.is_empty() || (feature2.contains(&Arrow) && feature2.len() ==1);
                                if cond1 && cond2{
                                    return Some(Element::Line(
                                                s.clone(),
                                                e2.clone(),
                                                stroke.clone(),
                                                feature2.clone()
                                                ));
                                }
                            }
                            //    line1     line2
                            //  s------e   e2-------s2
                            //  s-------------------s2
                            else if e == e2{
                                //  -------  --------
                                //  o------  ---------
                                if (feature.is_empty() || (feature.contains(&Circle) && feature.len() == 1))
                                    && feature2.is_empty(){
                                    return Some(Element::Line(
                                            s.clone(),
                                            s2.clone(),
                                            stroke.clone(),
                                            feature2.clone()
                                            ));
                                }
                            }
                            //    line1    line2
                            //  e------s   s2------e2
                            //  s------------------e2
                            else if s == s2{
                                // -------   -------
                                // -------  ------->
                                if feature.is_empty()
                                    && (feature2.is_empty() || (feature2.contains(&Arrow) && feature2.len() ==1 )){
                                        return Some(Element::Line(
                                                e.clone(),
                                                e2.clone(),
                                                stroke.clone(),
                                                feature2.clone()
                                                ));
                                    }
                            }
                            //      line1    line2
                            //  e------s    e2------s2
                            //  e---------------------s2
                            //
                            else if s == e2{
                                //   -----   -----
                                //   -----   ----o
                                //   <----   -----
                                //   <----   ----o
                                let cond1 = feature.is_empty() || (feature.contains(&Arrow) && feature.len() == 1);
                                let cond2 = feature2.is_empty() || (feature2.contains(&Circle) && feature2.len() == 1); 
                                if cond1 && cond2{
                                    return Some(Element::Line(
                                            s2.clone(),
                                            e.clone(),
                                            stroke.clone(),
                                            feature.clone(),
                                            ));
                                    }
                            }
                        }
                        return None;
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
    pub fn to_svg(&self, settings: &Settings) -> SvgElement {
        match *self {
            Element::Circle(ref c, r, ref class) => {
                let svg_circle = SvgCircle::new()
                    .set("class", class.clone())
                    .set("cx", c.x)
                    .set("cy", c.y)
                    .set("r", r);

                SvgElement::Circle(svg_circle)
            }
            Element::Line(ref s, ref e, ref stroke, ref features) => {
                let mut svg_line = SvgLine::new()
                    .set("x1", s.x)
                    .set("y1", s.y)
                    .set("x2", e.x)
                    .set("y2", e.y);
                for feature in features{
                    match *feature {
                        Arrow => {
                            svg_line.assign("marker-end", "url(#triangle)");
                        }
                        ArrowStart => {
                            svg_line.assign("marker-start", "url(#triangle)");
                        }
                        Circle => {
                            svg_line.assign("marker-start", "url(#circle)");
                        }
                        OpenCircle => {
                            svg_line.assign("marker-start", "url(#open_circle)");
                        }
                    };
                }
                match *stroke {
                    Solid => (),
                    Dashed => {
                        svg_line.assign("fill", "none");
                        svg_line.assign("class","dashed");
                    }
                };

                SvgElement::Line(svg_line)
            }
            Element::Arc(ref s, ref e, radius, ref arc_flag, sweep, _, ref features) => {
                let sweept = if sweep { "1" } else { "0" };
                let arc_flag = match *arc_flag {
                    Major => "1",
                    Minor => "0",
                };
                let d = format!(
                    "M {} {} A {} {} 0 {} {} {} {}",
                    s.x, s.y, radius, radius, arc_flag, sweept, e.x, e.y
                );
                let mut svg_arc = SvgPath::new().set("d", d).set("fill", "none");
                for feature in features{
                    match *feature {
                        Arrow => {
                            svg_arc.assign("marker-end", "url(#triangle)");
                        }
                        ArrowStart => {
                            svg_arc.assign("marker-start", "url(#triangle)");
                        }
                        Circle => {
                            svg_arc.assign("marker-start", "url(#circle)");
                        }
                        OpenCircle => {
                            svg_arc.assign("marker-start", "url(#open_circle)");
                        }
                    };
                }
                SvgElement::Path(svg_arc)
            }
            Element::Text(ref loc, ref string) => {
                let sx = loc.x as f32 * settings.text_width + settings.text_width / 8.0;
                let sy = loc.y as f32 * settings.text_height + settings.text_height * 3.0 / 4.0;
                let mut svg_text = SvgText::new().set("x", sx).set("y", sy);
                let text_node = svg::node::Text::new(string.to_string());
                svg_text.append(text_node);
                SvgElement::Text(svg_text)
            }
        }
    }
}
