use loc::Loc;
use focus_char::FocusChar;
use optimizer::Optimizer;
use svg_element::SvgElement;
use svg::node::element::SVG;
use svg::Node;
use svg::node::element::{
    Definitions,
    Marker,
    Rectangle as SvgRect,
    Style,
    Circle as SvgCircle,
    Polygon as SvgPolygon,
    Group,
};
use element::Element;
use pom::TextInput;
use pom::parser::{sym,none_of};
use settings::Settings;
use unicode_width::UnicodeWidthChar;
use pom;

#[derive(Debug)]
pub struct Grid {
    pub settings: Settings,
    /// cell value is in string instead of char to accomodate multiple width characters
    /// each line is Vec<String>
    index: Vec<Vec<String>>,
    /// This are text elements that are escaped and are not processed for diagram
    /// matching
    text_elm: Vec<(usize, usize, String)>,
}
impl Grid {
    /// instantiate a grid from input ascii text
    /// Issues:
    /// 1. 2-width, 5 bytes, single character  i.e. 统
    /// 2. 1-width, 2 bytes, single character  i.e. ö
    /// 3. 1-width, 3 bytes, single character  i.e. o͡͡͡
    pub fn from_str(s: &str, settings: &Settings) -> Grid {
        let lines: Vec<&str> = s.lines().collect();
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(lines.len());
        let mut text_elm: Vec<(usize, usize, String)> = vec![];
        for (y, line) in lines.iter().enumerate() {
            let (line, escaped_texts): (String, Vec<(usize, String)>) = exclude_escaped_text(line);
            let mut row: Vec<String> = Vec::with_capacity(line.chars().count());
            for (x, escaped) in escaped_texts {
                text_elm.push((x, y, svg_escape(&escaped)));
            }
            for ch in line.chars() {
                if let Some(1) = ch.width() {
                    row.push(format!("{}", ch));
                } else if let Some(2) = ch.width() {
                    row.push(format!("{}", ch));
                    // HACK: push a blank to the next cell,
                    //in order to make this character twice as
                    // big and aligns the next succeeding characters on
                    // this row
                    row.push(format!("\0"));
                }
                // if zero width char, append it to the previous string
                else if let Some(0) = ch.width() {
                    let prev: Option<String> = row.pop();
                    match prev {
                        Some(mut prev) => {
                            prev.push(ch);
                            row.push(prev);
                        }
                        None => (),
                    }
                }
            }
            rows.push(row);
        }
        let g = Grid {
            settings: settings.clone(),
            index: rows,
            text_elm: text_elm,
        };
        // do the pre processing here
        g.pre_process()
    }


    pub fn rows(&self) -> usize {
        self.index.len()
    }

    /// get the maximum row len
    pub fn columns(&self) -> usize {
        self.index.iter().map(|r| r.len()).max().unwrap_or(0)
    }

    /// get a character at this location
    /// widths are computed since there are
    /// characters that spans 2 columns
    /// and characters that has 0 width
    ///
    pub fn get(&self, loc: &Loc) -> Option<&String> {
        match self.index.get(loc.y as usize) {
            Some(row) => row.get(loc.x as usize),
            None => None,
        }
    }


    fn text(&self, loc: &Loc) -> &str {
        match self.get(loc) {
            Some(ref s) => s,
            None => ""
        }
    }

    /// get the focus char at this location
    pub fn get_focuschar(&self, loc: &Loc) -> FocusChar {
        FocusChar::new(&loc, self)
    }

    /// process the enhancing of circle elements
    /// this should be called before other elements are extracted from the grid
    fn get_enhance_circle_elements(&self) -> (Vec<Vec<Vec<Element>>>, Vec<Loc>){
        let mut rows: Vec<Vec<Vec<Element>>> = Vec::with_capacity(self.index.len());
        let mut all_consumed_loc: Vec<Loc> = vec![];
        for (y,line) in self.index.iter().enumerate() {
            let mut row: Vec<Vec<Element>> = Vec::with_capacity(line.len());
            for (x,_cell) in line.iter().enumerate() {
                let loc = Loc::new(x as i32, y as i32);
                let focus_char = self.get_focuschar(&loc);
                let (cell_elements, consumed_loc) = focus_char.get_enhance_circle_elements();
                all_consumed_loc.extend(consumed_loc);
                row.push(cell_elements);
            }
            rows.push(row);
        }
        (rows, all_consumed_loc)
    }

    /// process the enhanced circle elements first
    /// then process the generic enhancements
    fn get_enhance_elements(&self) -> (Vec<Vec<Vec<Element>>>, Vec<Loc>){
        let (enhanced_circle_elm, circle_consumed_loc) = self.get_enhance_circle_elements();
        let mut rows: Vec<Vec<Vec<Element>>> = Vec::with_capacity(self.index.len());
        rows.extend(enhanced_circle_elm);
        let mut all_consumed_loc: Vec<Loc> = vec![];
        for (y,line) in self.index.iter().enumerate() {
            let mut row: Vec<Vec<Element>> = Vec::with_capacity(line.len());
            for (x,_cell) in line.iter().enumerate() {
                let loc = Loc::new(x as i32, y as i32);
                if !circle_consumed_loc.contains(&loc){
                    let focus_char = self.get_focuschar(&loc);
                    let (cell_elements, consumed_loc) = focus_char.get_enhance_elements();
                    all_consumed_loc.extend(consumed_loc);
                    row.push(cell_elements);
                }
            }
            rows.push(row);
        }
        all_consumed_loc.extend(circle_consumed_loc);
        (rows, all_consumed_loc)
    }

    /// vector of each elements arranged in rows x columns
    /// returns all the elements and the consumed location
    fn get_all_elements(&self) -> Vec<Vec<Vec<Element>>>{
        let (enhanced_elms, enhance_consumed_locs) = self.get_enhance_elements();
        let mut rows: Vec<Vec<Vec<Element>>> = Vec::with_capacity(self.index.len());
        rows.extend(enhanced_elms);
        for (y, line) in self.index.iter().enumerate() {
            let mut row: Vec<Vec<Element>> = Vec::with_capacity(line.len());
            for (x, _cell) in line.iter().enumerate() {
                let loc = Loc::new(x as i32, y as i32);
                if !enhance_consumed_locs.contains(&loc) {
                    let focus_char = self.get_focuschar(&loc);
                    let cell_elements = focus_char.get_elements();
                    row.push(cell_elements);
                }
            }
            rows.push(row);
        }
        rows
    }


    fn get_escaped_text_elements(&self) -> Vec<Element> {
        self.text_elm
            .iter()
            .map(|&(x, y, ref text)| Element::Text(Loc::new(x as i32, y as i32), text.to_owned()))
            .collect()
    }

    /// each component has its relative location retain
    /// use this info for optimizing svg by checking closest neigbor
    fn get_svg_nodes(&self) -> Vec<Vec<SvgElement>> {
        let mut grouped_nodes = vec![];
        let mut elements= self.get_all_elements();
        let text_elm = self.get_escaped_text_elements();
        elements.push(vec![text_elm]);
        let optimizer = Optimizer::new(elements);
        let optimized_elements:Vec<Vec<Element>> = optimizer.optimize(&self.settings);
        for group in optimized_elements {
            let mut svg_group = vec![];
            for elem in group{
                let element: SvgElement = elem.to_svg(&self.settings);
                svg_group.push(element);
            }
            grouped_nodes.push(svg_group);
        }
        grouped_nodes
    }


    pub fn get_size(&self) -> (f32, f32) {
        let width = self.settings.text_width * self.columns() as f32;
        let height = self.settings.text_height * self.rows() as f32;
        (width, height)
    }

    /// get the generated svg according to the settings specified
    pub fn get_svg(&self) -> SVG {
        let group_nodes = self.get_svg_nodes();
        let (width, height) = self.get_size();
        let mut svg = SVG::new();

        if let Some(ref id) = self.settings.id {
            svg.assign("id", id.to_owned());
        }
        if let Some(ref class) = self.settings.class {
            svg.assign("class", class.to_owned());
        }
        svg.assign("font-size", self.settings.font_size);
        svg.assign("font-family", self.settings.font_family.to_owned());
        svg.assign("width", width);
        svg.assign("height", height);


        svg.append(get_defs());
        svg.append(get_styles(&self.settings));


        let rect = SvgRect::new()
            .set("x", 0)
            .set("y", 0)
            .set("fill", self.settings.background_color.to_string())
            .set("width", width)
            .set("height", height);
        svg.append(rect);

        for group in group_nodes {
            let mut svg_group = Group::new();
            for node in group{
                match node {
                    SvgElement::Circle(circle) => {
                        svg_group.append(circle);
                    }
                    SvgElement::Line(line) => {
                        svg_group.append(line);
                    }
                    SvgElement::Path(path) => {
                        svg_group.append(path);
                    }
                    SvgElement::Text(text) => {
                        svg_group.append(text);
                    }
                }
            }
            svg.append(svg_group);
        }
        svg
    }
    /// traverse each element of the grid and swap characters as needed
    fn pre_process(&self) -> Self {
        let mut new_index: Vec<Vec<String>> = vec![];
        for (y,line) in self.index.iter().enumerate(){
            let mut row: Vec<String> = vec![];
            for (x,_) in line.iter().enumerate(){
                let loc = &Loc::new(x as i32, y as i32);
                let swap = self.swap_char(loc);
                row.push(swap.to_string());
            }
            new_index.push(row);
        }
        Grid{
            settings: self.settings.clone(),
            index: new_index,
            text_elm: self.text_elm.clone()
        }
    }

    /// swap characters  - - - with ~~~~~
    fn swap_char(&self, loc: &Loc) -> &str {
        let cell = self.text(loc);

        let left = self.text(&loc.left());
        let left2 = self.text(&loc.in_left(2));
        let left3 = self.text(&loc.in_left(3));
        let left4 = self.text(&loc.in_left(4));
        let right = self.text(&loc.right());
        let right2 = self.text(&loc.in_right(2));
        let right3 = self.text(&loc.in_right(3));
        let right4 = self.text(&loc.in_right(4));
        // [- - -]
        //  ^
        // if `-` and right is ` ` and right(2) is `-` right(3) is ` ` and right(4) is `-`
        if (cell == "-" && right == " " && right2 == "-" && right3 == " " && right4 == "-")
        // [- - -]
        //   ^
        // if ` `  and left  is `-` and right is `-` and right(2) is ` ` and right(3) is `-`
        || (cell == " " && left == "-" && right == "-" && right2 == " " && right3 == "-")
        // [- - -]
        //    ^
        // if `-` , and left  is ` ` and right is ` ` and left(2) is `-`  and right(2) is `-`
        || (cell == "-" && left == " " && right == " " && left2 == "-" && right2 == "-")
        // [- - -]
        //     ^
        //  if ` `, and left is `-` and right is `-` and left(2) is ` ` and left(3) is `-`
        || (cell == " " && left == "-" && right == "-" && left2 == " " && left3 == "-")
        // [- - -]
        //      ^
        //  if `-`, and left  is ` ` and left(2) is `-` and left(3) is ` ` and left(4) is `-`
        || (cell == "-" && left == " " && left2 == "-" && left3 == " " && left4 == "-")
        {
            "~"
        }
        else{
           cell
        }
    }

}

fn get_defs() -> Definitions {
    let mut defs = Definitions::new();
    defs.append(arrow_marker());
    defs.append(clear_arrow_marker());
    defs.append(circle_marker());
    defs.append(square_marker());
    defs.append(open_circle_marker());
    defs.append(big_open_circle_marker());
    defs
}

fn get_styles(settings: &Settings) -> Style {
    let style = format!(
        r#"
    line,path {{
      stroke: {stroke_color};
      stroke-width: {stroke_width};
      stroke-opacity: 1;
      fill-opacity: 1;
      stroke-linecap: round;
      stroke-linejoin: miter;
    }}
    line.dashed {{
        stroke-dasharray: 5;
    }}
    circle.solid {{
      fill:{stroke_color};
      stroke: black;
      stroke-width: {stroke_width};
      stroke-opacity: 1;
      fill-opacity: 1;
      stroke-linecap: round;
      stroke-linejoin: miter;
    }}
    circle.open {{
      fill:none;
      stroke: black;
      stroke-width: {stroke_width};
      stroke-opacity: 1;
      fill-opacity: 1;
      stroke-linecap: round;
      stroke-linejoin: miter;
    }}
    tspan.head{{
        fill: none;
        stroke: none;
    }}
    "#,
        stroke_width = settings.stroke_width,
        stroke_color = &settings.stroke_color,
    );
    Style::new(style)
        .set("type", "text/css")
}

fn arrow_marker() -> Marker {
    let mut marker = Marker::new()
        .set("id", "triangle")
        .set("viewBox", "0 0 8 4")
        .set("refX", 4)
        .set("refY", 2)
        .set("orient", "auto")
        .set("markerWidth", 8)
        .set("markerHeight", 8);

    let path = SvgPolygon::new()
        .set("points", "0,0 0,4 8,2 0,0")
        .set("fill", "black");

    marker.append(path);
    marker
}

fn clear_arrow_marker() -> Marker {
    let mut marker = Marker::new()
        .set("id", "clear_triangle")
        .set("viewBox", "0 0 20 14")
        .set("refX", 1)
        .set("refY", 7)
        .set("orient", "auto")
        .set("markerWidth", 10)
        .set("markerHeight", 10);

    let path = SvgPolygon::new()
        .set("points", "2,2 2,12 18,7 2,2")
        .set("fill", "none")
        .set("stroke-width", 2)
        .set("stroke", "black");

    marker.append(path);
    marker
}


///   <marker id="dot" viewBox="0 0 10 10" refX="5" refY="5"
///        markerWidth="5" markerHeight="5">
///      <circle cx="5" cy="5" r="5" fill="red" />
///    </marker>
fn circle_marker() -> Marker {
    let mut marker = Marker::new()
        .set("id", "circle")
        .set("viewBox", "0 0 20 20")
        .set("refX", 10)
        .set("refY", 10)
        .set("orient", "auto")
        .set("markerWidth", 5)
        .set("markerHeight", 5);

    let circle = SvgCircle::new()
        .set("cx",10)
        .set("cy",10)
        .set("r",8)
        .set("fill","black");
    marker.append(circle);
    marker
}

fn square_marker() -> Marker {
    let mut marker = Marker::new()
        .set("id", "square")
        .set("viewBox", "0 0 20 20")
        .set("refX", 10)
        .set("refY", 10)
        .set("orient", "auto")
        .set("markerWidth", 5)
        .set("markerHeight", 5);

    let square = SvgRect::new()
        .set("x",0)
        .set("y",0)
        .set("width",20)
        .set("height",20)
        .set("fill","black");
    marker.append(square);
    marker
}

fn open_circle_marker() -> Marker {
    let mut marker = Marker::new()
        .set("id", "open_circle")
        .set("viewBox", "0 0 20 20")
        .set("refX", 10)
        .set("refY", 10)
        .set("orient", "auto")
        .set("markerWidth", 10)
        .set("markerHeight", 10);

    let circle = SvgCircle::new()
        .set("cx",10)
        .set("cy",10)
        .set("r",4)
        .set("stroke","black")
        .set("stroke-width",2)
        .set("fill","white");
    marker.append(circle);
    marker
}
fn big_open_circle_marker() -> Marker {
    let mut marker = Marker::new()
        .set("id", "big_open_circle")
        .set("viewBox", "0 0 40 40")
        .set("refX", 20)
        .set("refY", 20)
        .set("orient", "auto")
        .set("markerWidth", 20)
        .set("markerHeight", 20);

    let circle = SvgCircle::new()
        .set("cx",20)
        .set("cy",20)
        .set("r",6)
        .set("stroke","black")
        .set("stroke-width",2)
        .set("fill","white");
    marker.append(circle);
    marker
}
//copied from https://github.com/rust-lang/rust/blob/master/src/librustdoc/html/escape.rs
//just adding for \0
pub fn svg_escape(arg: &str) -> String {
    use std::fmt;

    /// Wrapper struct which will emit the HTML-escaped version of the contained
    /// string when passed to a format string.
    pub struct Escape<'a>(pub &'a str);

    impl<'a> fmt::Display for Escape<'a> {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            // Because the internet is always right, turns out there's not that many
            // characters to escape: http://stackoverflow.com/questions/7381974
            let Escape(s) = *self;
            let pile_o_bits = s;
            let mut last = 0;
            for (i, ch) in s.bytes().enumerate() {
                match ch as char {
                    '<' | '>' | '&' | '\'' | '"' | '\0' => {
                        fmt.write_str(&pile_o_bits[last..i])?;
                        let s = match ch as char {
                            '>' => "&gt;",
                            '<' => "&lt;",
                            '&' => "&amp;",
                            '\'' => "&#39;",
                            '"' => "&quot;",
                            '\0' => "",
                            _ => unreachable!(),
                        };
                        fmt.write_str(s)?;
                        last = i + 1;
                    }
                    _ => {}
                }
            }

            if last < s.len() {
                fmt.write_str(&pile_o_bits[last..])?;
            }
            Ok(())
        }
    };
    let escaped = Escape(arg);
    format!("{}", escaped)
}

fn exclude_escaped_text(line: &str) -> (String, Vec<(usize, String)>) {
    let mut input = TextInput::new(line);
    let parsed = line_parse().parse(&mut input);
    let mut buffer = String::new();
    let mut text_elm: Vec<(usize, String)> = vec![];
    if let Ok(parsed) = parsed {
        let mut index = 0;
        if !parsed.is_empty() {
            for (start, end) in parsed {
                let escaped = &line[start + 1..end];
                let recons = &line[index..start];
                text_elm.push((start, escaped.to_string()));
                buffer.push_str(recons);
                buffer.push_str(&" ".repeat(end + 1 - start));
                index = end + 1;
            }
            buffer.push_str(&line[index..line.len()]);
        } else {
            buffer.push_str(line);
        }
    }
    (buffer, text_elm)
}

#[test]
fn test_escaped_string() {
    let input3 = r#"The "qu/i/ck" brown "fox\"s" jumps over the lazy "do|g""#;
    let mut raw3 = TextInput::new(input3);
    let output3 = line_parse().parse(&mut raw3);
    println!("output3: {:?}", output3);
    //assert_eq!(Ok(vec![(4, 12), (20, 27), (49, 54)]), output3);
    let mut matches = vec![];
    let mut recons = String::new();
    let mut text_elm: Vec<(usize, String)> = vec![];
    let mut index = 0;
    if let Ok(output) = output3 {
        for (start, end) in output {
            println!("matches: {}", &input3[start..end + 1]);
            matches.push(input3[start..end + 1].to_string());
            let slice = &input3[index..start];
            recons.push_str(slice);
            recons.push_str(&" ".repeat(end + 1 - start));
            text_elm.push((start, input3[start + 1..end].to_string()));
            index = end + 1;
        }
    }
    println!("input3: {}", input3);
    println!("recons: {}", recons);
    println!("escaped: {:?}", text_elm);
    assert_eq!(vec![r#""qu/i/ck""#, r#""fox\"s""#, r#""do|g""#], matches);
    assert_eq!(input3.len(), recons.len());
}

#[test]
fn test_escaped_multiline_string() {
    let input3 = r#"The "qu/i/ck brown fox \njumps over the lazy do|g""#;
    let mut raw3 = TextInput::new(input3);
    let output3 = line_parse().parse(&mut raw3);
    println!("output3: {:?}", output3);
    assert_eq!(Ok(vec![(4, 49)]), output3);
    let mut matches = vec![];
    let mut recons = String::new();
    let mut text_elm: Vec<(usize, String)> = vec![];
    let mut index = 0;
    if let Ok(output) = output3 {
        for (start, end) in output {
            println!("matches: {}", &input3[start..end + 1]);
            matches.push(input3[start..end + 1].to_string());
            let slice = &input3[index..start];
            recons.push_str(slice);
            recons.push_str(&" ".repeat(end + 1 - start));
            text_elm.push((start, input3[start + 1..end].to_string()));
            index = end + 1;
        }
    }
    println!("input3: {}", input3);
    println!("recons: {}", recons);
    println!("escaped: {:?}", text_elm);
    assert_eq!(
        vec![r#""qu/i/ck brown fox \njumps over the lazy do|g""#],
        matches
    );
    assert_eq!(input3.len(), recons.len());
}

fn escape_string() -> pom::parser::Parser<'static, char, (usize, usize)> {
    let escape_sequence = sym('\\') * sym('"'); //escape sequence \"
    let char_string = escape_sequence | none_of("\"");
    let escaped_string_end = sym('"') * char_string.repeat(0..).pos() - sym('"');
    none_of("\"").repeat(0..).pos() + escaped_string_end - none_of("\"").repeat(0..).discard()
}

fn line_parse() -> pom::parser::Parser<'static, char, Vec<(usize, usize)>> {
    escape_string().repeat(0..)
}

