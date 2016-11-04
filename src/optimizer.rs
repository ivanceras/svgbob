use super::Loc;
use super::Element;
use super::Stroke;
use super::Feature;
use super::Point;
use super::Settings;

pub struct Optimizer {
    elements: Vec<(Loc, Vec<Element>)>,
}

impl Optimizer {
    pub fn new(elements: Vec<(Loc, Vec<Element>)>) -> Optimizer {
        Optimizer { elements: elements }
    }

    fn get(&self, loc: &Loc) -> Option<&Vec<Element>> {
        let found = self.elements
            .iter()
            .find(|x| {
                let &(ref l, _) = *x;
                loc == l
            });
        match found {
            Some(&(_, ref elm)) => Some(elm),    
            None => None,
        }
    }

    // return the first element only
    // there is only one element in the component
    fn first_element_only(&self, loc: &Loc) -> Option<&Element> {
        match self.get(loc) {
            Some(elements) => {
                if elements.len() == 1 {
                    elements.get(0)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    // if the path on this location can be eated
    // from the left, from the top
    fn is_edible(&self, loc: &Loc) -> bool {
        self.can_loc_reduce(&loc.top(), loc) || self.can_loc_reduce(&loc.left(), loc) ||
        self.can_loc_reduce(&loc.top_left(), loc) ||
        self.can_loc_reduce(&loc.bottom_left(), loc) ||
        self.can_loc_reduce(&loc.left().left(), loc) //full width character CJK can reduce 2 cells apart
    }
    // determine if element in location1
    // can reduce the element in location2
    fn can_loc_reduce(&self, loc1: &Loc, loc2: &Loc) -> bool {
        match self.first_element_only(loc1) {
            Some(elm1) => self.reduce(elm1, loc2).is_some(),
            None => false,
        }
    }

    fn reduce(&self, elm1: &Element, loc2: &Loc) -> Option<Element> {
        let elm2 = self.first_element_only(loc2);
        match elm2 {
            Some(elm2) => elm1.reduce(elm2),
            None => None,
        }
    }
    fn trace_elements(&self, element: &Element, loc: &Loc) -> Element {
        match self.reduce(element, &loc.right()) {
            Some(reduced) => self.trace_elements(&reduced, &loc.right()),
            None => {
                match self.reduce(element, &loc.bottom()) {
                    Some(reduced) => self.trace_elements(&reduced, &loc.bottom()),
                    None => {
                        match self.reduce(element, &loc.bottom_right()) {
                            Some(reduced) => self.trace_elements(&reduced, &loc.bottom_right()),
                            None => {
                                match self.reduce(element, &loc.top_right()) {
                                    Some(reduced) => {
                                        self.trace_elements(&reduced, &loc.top_right())
                                    }
                                    None => {
                                        //full width character CJK can reduce 2 cells apart
                                        match self.reduce(element, &loc.right().right()){
                                            Some(reduced) => self.trace_elements(&reduced, &loc.right().right()),
                                            None => element.clone(),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // TODO: order the elements in such a way that
    // the start -> end -> start chains nicely
    pub fn optimize(&self, settings: &Settings) -> Vec<Element> {
        let mut optimized = vec![];
        for &(ref loc, ref elem) in &self.elements {
            if self.is_edible(&loc) {
                ;
            } else {
                for e in elem {
                    let traced = self.trace_elements(e, loc);
                    optimized.push(traced);
                }
            }
        }
        if settings.compact_path {
            self.merge_paths(optimized)
        } else {
            optimized
        }
    }
    // take all paths and non-arrow line in 1 path
    // the text in separated
    fn merge_paths(&self, elements: Vec<Element>) -> Vec<Element> {
        let mut merged = vec![];
        let mut solid_paths = vec![];
        let mut dashed_paths = vec![];
        let mut arrows = vec![];
        let mut text = vec![];
        for elm in elements {
            match elm {
                Element::Line(_, _, ref stroke, ref feature) => {
                    match *feature {
                        Feature::Arrow => {
                            arrows.push(elm.clone());
                        },
                        Feature::Circle =>{
                            arrows.push(elm.clone());
                        },
                        Feature::Nothing => {
                            match *stroke {
                                Stroke::Solid => {
                                    solid_paths.push(elm.clone());
                                }
                                Stroke::Dashed => {
                                    dashed_paths.push(elm.clone());
                                }
                            }
                        }
                    }
                }
                Element::Arc(_, _, _, _) => solid_paths.push(elm.clone()),
                Element::Text(_, _) => text.push(elm.clone()),
                Element::Path(_, _, _, _) => {
                    merged.push(elm.clone());
                }
            }
        }
        merged.push(unify(solid_paths, Stroke::Solid));
        merged.push(unify(dashed_paths, Stroke::Dashed));
        merged.extend(arrows);
        merged.extend(text);
        merged
    }
}

fn unify(elements: Vec<Element>, stroke: Stroke) -> Element {
    let mut paths = String::new();
    let mut last_loc = None;
    let mut start = None;
    for elm in elements {
        match elm {
            Element::Line(s, e, _, _) => {
                if start.is_none() {
                    start = Some(s.clone());
                }
                let match_last_loc = match last_loc {
                    Some(last_loc) => s == last_loc,
                    None => false,
                };
                if match_last_loc {
                    paths.push_str(&format!(" L {} {}", e.x, e.y));
                } else {
                    paths.push_str(&format!(" M {} {} L {} {}", s.x, s.y, e.x, e.y));
                }
                last_loc = Some(e.clone());
            }
            Element::Arc(s, e, r, sw) => {
                if start.is_none() {
                    start = Some(s.clone());
                }
                let match_last_loc = match last_loc {
                    Some(last_loc) => s == last_loc,
                    None => false,
                };
                let sweep = if sw { 1 } else { 0 };
                if match_last_loc {
                    paths.push_str(&format!(" A {} {} 0 0 {} {} {}", r, r, sweep, e.x, e.y));
                } else {
                    paths.push_str(&format!(" M {} {} A {} {} 0 0 {} {} {}",
                                            s.x,
                                            s.y,
                                            r,
                                            r,
                                            sweep,
                                            e.x,
                                            e.y));
                }
                last_loc = Some(e.clone());
            }
            _ => panic!("only lines are arc can be unified"),
        }
    }
    let el_start = match start {
        Some(start) => start.clone(),
        None => Point::new(0.0, 0.0),
    };
    let el_end = match last_loc {
        Some(last_loc) => last_loc.clone(),
        None => Point::new(0.0, 0.0),
    };
    Element::Path(el_start, el_end, paths, stroke)
}
