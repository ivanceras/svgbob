use super::Element;
use super::Feature;
use super::Loc;
use super::Point;
use super::Settings;
use super::Stroke;
use ArcFlag;

pub struct Optimizer {
    elements: Vec<Vec<Vec<Element>>>,
    /// TODO: consumed location should also include the consumed element index of that location.
    consumed_loc: Vec<Loc>,
}

impl Optimizer {
    pub fn new(elements: Vec<Vec<Vec<Element>>>, consumed_loc: Vec<Loc>) -> Optimizer {
        Optimizer {
            elements: elements,
            consumed_loc: consumed_loc,
        }
    }


    fn get(&self, loc: &Loc) -> Option<&Vec<Element>> {
        match self.elements.get(loc.y as usize) {
            Some(row) => row.get(loc.x as usize),
            None => None,
        }
    }



    // return the reduced element and the index of the matching element on this location
    fn reduce(&self, elm1: &Element, loc2: &Loc) -> Option<(Vec<Element>, usize)> {
        // try all the elments of this location
        if let Some(elements2) = self.get(loc2){
            if elements2.len() > 0 {
                for (i,elm2) in elements2.iter().enumerate(){
                    // use the element that can be reduced with
                    if let Some(reduced) = elm1.reduce(&elm2){
                        let mut new_reduced = vec![];
                        new_reduced.push(reduced);
                        return Some((new_reduced, i));
                    }
                }
            }
        }
        None
        
    }
    fn trace_elements(&self, element: &Element, loc: &Loc) -> (Vec<Element>, Vec<(Loc, usize)>) {
        //trace to the right first
        let right = loc.right();
        let bottom = loc.bottom();
        let bottom_right = loc.bottom_right();
        let bottom_left = loc.bottom_left();
        if let Some((all_reduced, elm_index)) = self.reduce(element, &right){
            let mut all_consumed:Vec<(Loc, usize)> = vec![];
            let mut only_reduced = vec![];
            for reduced_elm in all_reduced{
                let (reduced, consumed) = self.trace_elements(&reduced_elm, &right);
                all_consumed.push((right.clone(), elm_index));
                all_consumed.extend(consumed);
                only_reduced = reduced;
            }
            (only_reduced, all_consumed)
        }
        else if let Some((all_reduced, elm_index)) = self.reduce(element, &bottom){
            let mut all_consumed = vec![];
            let mut only_reduced = vec![];
            for reduced_elm in all_reduced{
                let (reduced, consumed) = self.trace_elements(&reduced_elm, &bottom);
                all_consumed.push((bottom.clone(), elm_index));
                all_consumed.extend(consumed);
                only_reduced = reduced;
            }
            (only_reduced, all_consumed)
        }
        else if let Some((all_reduced, elm_index)) = self.reduce(element, &bottom_right){
            let mut all_consumed = vec![];
            let mut only_reduced = vec![];
            for reduced_elm in all_reduced{
                let (reduced, consumed) = self.trace_elements(&reduced_elm, &bottom_right);
                all_consumed.push((bottom_right.clone(), elm_index));
                all_consumed.extend(consumed);
                only_reduced = reduced;
            }
            (only_reduced, all_consumed)
        }
        else if let Some((all_reduced, elm_index)) = self.reduce(element, &bottom_left){
            let mut all_consumed = vec![];
            let mut only_reduced = vec![];
            for reduced_elm in all_reduced{
                let (reduced, consumed) = self.trace_elements(&reduced_elm, &bottom_left);
                all_consumed.push((bottom_left.clone(),elm_index));
                all_consumed.extend(consumed);
                only_reduced = reduced;
            }
            (only_reduced, all_consumed)
        }
        else{
            (vec![element.to_owned()], vec![])
        }
    }

    // TODO: order the elements in such a way that
    // the start -> end -> start chains nicely
    pub fn optimize(&self, settings: &Settings) -> Vec<Element> {
        let completely_consumed_locs:Vec<Loc> = self.consumed_loc.clone();
        let mut tracing_consumed_locs: Vec<(Loc,usize)> = vec![];
        let mut optimized = vec![];
        for (y, line) in self.elements.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                let loc = &Loc::new(x as i32, y as i32);
                for (elm_index, elm) in cell.iter().enumerate() {
                    if !completely_consumed_locs.contains(loc) 
                        && !tracing_consumed_locs.contains(&(loc.clone(),elm_index)){
                            let (traced, consumed) = self.trace_elements(elm, loc);
                            optimized.extend(traced);
                            tracing_consumed_locs.extend(consumed);
                    }
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
        let mut solid_lines = vec![];
        let mut dashed_lines = vec![];
        let mut solid_arcs = vec![];
        let mut dashed_arcs = vec![];
        let mut arrows = vec![];
        let mut text = vec![];
        let mut circles = vec![];
        for elm in elements {
            match elm {
                Element::Circle(_, _, _) => {
                    circles.push(elm.clone());
                }
                Element::Line(_, _, ref stroke, ref features) => {
                    for feature in features{
                        match *feature {
                            Feature::Arrow => {
                                arrows.push(elm.clone());
                            }
                            // circle at the end rather than arrow
                            Feature::Circle => {
                                arrows.push(elm.clone());
                            }
                        }
                    }
                    match *stroke {
                        Stroke::Solid => {
                            solid_lines.push(elm.clone());
                        }
                        Stroke::Dashed => {
                            dashed_lines.push(elm.clone());
                        }
                    }
                },
                Element::Arc(_, _, _, _, _, ref stroke, ref features) => {
                    for feature in features{
                        match *feature {
                            Feature::Arrow => {
                                arrows.push(elm.clone());
                            }
                            Feature::Circle => {
                                arrows.push(elm.clone());
                            }
                        }
                    }

                    match *stroke {
                        Stroke::Solid => {
                            solid_arcs.push(elm.clone());
                        }
                        Stroke::Dashed => {
                            dashed_arcs.push(elm.clone());
                        }
                    }
                },
                Element::Text(_, _) => text.push(elm.clone()),
                Element::Path(_, _, _, _) => {
                    merged.push(elm.clone());
                }
            }
        }
        //merged.push(unify(solid_paths, Stroke::Solid));
        //merged.push(unify(dashed_paths, Stroke::Dashed));
        merged.extend(solid_lines);
        merged.extend(dashed_lines);
        merged.extend(solid_arcs);
        merged.extend(dashed_arcs);
        merged.extend(arrows);
        merged.extend(text);
        merged.extend(circles);
        merged
    }
}

/// cramp all paths that can be connected here
fn _unify(elements: Vec<Element>, stroke: Stroke) -> Element {
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
            Element::Arc(s, e, r, large, sw, _, _) => {
                if start.is_none() {
                    start = Some(s.clone());
                }
                let match_last_loc = match last_loc {
                    Some(last_loc) => s == last_loc,
                    None => false,
                };
                let sweep = if sw { 1 } else { 0 };
                let large = match large {
                    ArcFlag::Major => 1,
                    ArcFlag::Minor => 0,
                };
                if match_last_loc {
                    paths.push_str(&format!(" A {} {} 0 0 {} {} {}", r, r, sweep, e.x, e.y));
                } else {
                    paths.push_str(&format!(
                        " M {} {} A {} {} 0 {} {} {} {}",
                        s.x, s.y, r, r, large, sweep, e.x, e.y
                    ));
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
