use crate::{fragment::Line, Point};
use ncollide2d::{
    bounding_volume::AABB, math::Isometry, query::point_internal::point_query::PointQuery,
};
use std::cmp::Ordering;

pub fn opt_ord(f1: Option<f32>, f2: Option<f32>) -> Ordering {
    match (f1, f2) {
        (None, None) => Ordering::Equal,
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (Some(f1), Some(f2)) => ord(f1, f2),
    }
}

pub fn ord(f1: f32, f2: f32) -> Ordering {
    if f1 == f2 {
        Ordering::Equal
    } else if f1 > f2 {
        Ordering::Greater
    } else if f1 < f2 {
        Ordering::Less
    } else {
        println!("f1: {}, f2: {}", f1, f2);
        unreachable!("comparison should only be 3 possibilities")
    }
}

/// clips a line to the bounding box of this whole grid
/// and approximate each point to the closes intersection
fn clip_line_internal(aabb: &AABB<f32>, start: Point, end: Point) -> Option<(Point, Point)> {
    let start_v = start.to_vector();
    let end_v = end.to_vector() - start_v;
    let clipped = aabb.clip_line(&start, &end_v);
    if let Some(clipped) = clipped {
        let a = *clipped.a();
        let b = *clipped.b();
        Some((a.into(), b.into()))
    } else {
        None
    }
}

/// clip a line but do not extend the points
pub fn clip_line(aabb: &AABB<f32>, start: Point, end: Point) -> Option<(Point, Point)> {
    let clipped = clip_line_internal(aabb, start, end);
    let identity = &Isometry::identity();
    if let Some(clipped) = clipped {
        let mut clip_start = clipped.0;
        let mut clip_end = clipped.1;

        if aabb.contains_point(identity, &start) {
            clip_start = start;
        }
        if aabb.contains_point(identity, &end) {
            clip_end = end;
        }
        if clip_start == clip_end {
            None
        } else {
            Some((clip_start, clip_end))
        }
    } else {
        None
    }
}

/// the threshold are of 0.01 is used since
/// lines may not be very aligned.
pub fn is_collinear(a: &Point, b: &Point, c: &Point) -> bool {
    ncollide2d::utils::triangle_area(a, b, c) < 0.01
}

pub fn pad(v: f32) -> f32 {
    if v > 0.0 { v.ceil() } else { v.floor() }
}

/// this is parser module which provides parsing for identifier for
/// extracting the css tag of inside of a shape fragment
pub mod parser {
    use pom::parser::{call, end, is_a, list, none_of, one_of, sym, tag, Parser};
    use std::iter::FromIterator;

    /// Parses a list with the defined separator, but will fail early when one of the
    /// item can not be parsed
    pub fn list_fail<'a, I, O, U>(
        parser: Parser<'a, I, O>,
        separator: Parser<'a, I, U>,
    ) -> Parser<'a, I, Vec<O>>
    where
        O: 'a,
        U: 'a,
    {
        Parser::new(move |input: &'a [I], start: usize| {
            let mut items = vec![];
            let mut pos = start;
            match (parser.method)(input, pos) {
                Ok((first_item, first_pos)) => {
                    items.push(first_item);
                    pos = first_pos;
                    loop {
                        match (separator.method)(input, pos) {
                            Ok((_, sep_pos)) => {
                                match (parser.method)(input, sep_pos) {
                                    Ok((more_item, more_pos)) => {
                                        items.push(more_item);
                                        pos = more_pos;
                                    }
                                    Err(e) => {
                                        // return early when there is an
                                        // error matching the succeeding
                                        // items
                                        return Err(e);
                                    }
                                }
                            }
                            Err(e) => {
                                // the separator does not match, just break
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    // return early when there is an error matching the first item
                    return Err(e);
                }
            }
            Ok((items, pos))
        })
    }

    pub(super) fn alpha_or_underscore(ch: char) -> bool {
        pom::char_class::alpha(ch as u8) || underscore(ch)
    }

    pub(super) fn alphanum_or_underscore(ch: char) -> bool {
        pom::char_class::alphanum(ch as u8) || underscore(ch)
    }

    /// is this car is an underscore
    pub(super) fn underscore(ch: char) -> bool {
        ch == '_'
    }

    pub fn new_line<'a>() -> Parser<'a, char, ()> {
        one_of("\n").discard()
    }

    /// any whitespace character
    pub fn space<'a>() -> Parser<'a, char, ()> {
        one_of(" \t").repeat(0..).discard()
    }

    pub fn white_space<'a>() -> Parser<'a, char, ()> {
        one_of(" \t\r\n").repeat(0..).discard()
    }

    /// a valid identifier
    pub(crate) fn ident<'a>() -> Parser<'a, char, String> {
        (is_a(alpha_or_underscore) + is_a(alphanum_or_underscore).repeat(0..))
            .map(|(ch1, rest_ch)| format!("{}{}", ch1, String::from_iter(rest_ch)))
    }

    fn classes<'a>() -> Parser<'a, char, Vec<String>> {
        list(ident(), sym(','))
    }

    fn tag_classes<'a>() -> Parser<'a, char, Vec<String>> {
        sym('{') * classes() - sym('}')
    }

    /// parse css tag in the format of '{', <identifier>, '}'
    pub(crate) fn parse_css_tag(input: &str) -> Result<Vec<String>, pom::Error> {
        let cell_text_chars: Vec<char> = input.chars().collect();
        parse_css_tag_chars(&cell_text_chars)
    }

    fn parse_css_tag_chars(input: &[char]) -> Result<Vec<String>, pom::Error> {
        tag_classes().parse(input)
    }

    /// string inside a css content, taken as is as long as it is not `{` or `}`
    fn css_strings<'a>() -> Parser<'a, char, String> {
        let char_string = none_of("{}").repeat(1..).map(String::from_iter);
        let string = char_string.repeat(0..);
        string.map(|strings| strings.concat())
    }

    fn css_styles<'a>() -> Parser<'a, char, String> {
        sym('{') * css_strings() - sym('}')
    }

    /// a = {fill: red}
    /// b = {stroke: blue}
    fn css_style_list<'a>() -> Parser<'a, char, Vec<(String, String)>> {
        list(class_and_style(), new_line())
    }

    /// a = {fill: red}
    fn class_and_style<'a>() -> Parser<'a, char, (String, String)> {
        (-space() * ident() - space() - sym('=') - space()) + css_styles()
    }

    /// Parses:
    /// #Legend:\n
    ///  a = {fill: red}
    ///  b = {stroke: blue}
    ///
    fn css_legend<'a>() -> Parser<'a, char, Vec<(String, String)>> {
        (space() - sym('#') - space() - tag("Legend:") - space() - new_line()) * css_style_list()
    }

    fn css_legend_with_padding<'a>() -> Parser<'a, char, Vec<(String, String)>> {
        css_legend() - white_space()
    }

    pub(crate) fn parse_css_legend(input: &str) -> Result<Vec<(String, String)>, pom::Error> {
        let input_chars: Vec<char> = input.chars().collect();
        parse_css_legend_chars(&input_chars)
    }

    fn parse_css_legend_chars(input: &[char]) -> Result<Vec<(String, String)>, pom::Error> {
        css_legend_with_padding().parse(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_css_styles() {
            let input = "{fill:blue; stroke:red;}";
            let input_chars: Vec<char> = input.chars().collect();
            let css = css_styles().parse(&input_chars).expect("should parse");
            println!("css: {}", css);
            assert_eq!(css, "fill:blue; stroke:red;");
        }

        #[test]
        fn test_class_and_style() {
            let input = "a = {fill:blue; stroke:red;}";
            let input_chars: Vec<char> = input.chars().collect();
            let css = class_and_style().parse(&input_chars).expect("should parse");
            println!("css: {:?}", css);
            assert_eq!(css, ("a".to_string(), "fill:blue; stroke:red;".to_string()));
        }

        #[test]
        fn test_css_style_list() {
            let input = "a = {fill:blue; stroke:red;}\n\
                    b = {fill:red; stroke:black;}";
            let input_chars: Vec<char> = input.chars().collect();
            let css = css_style_list().parse(&input_chars).expect("should parse");
            println!("css: {:?}", css);
            assert_eq!(
                css,
                vec![
                    ("a".to_string(), "fill:blue; stroke:red;".to_string()),
                    ("b".to_string(), "fill:red; stroke:black;".to_string())
                ]
            );
        }

        #[test]
        fn test_css_legend() {
            let input = "# Legend: \n\
                    a = {fill:blue; stroke:red;}\n\
                    b = {fill:red; stroke:black;}";
            let input_chars: Vec<char> = input.chars().collect();
            let css = css_legend_with_padding()
                .parse(&input_chars)
                .expect("should parse");
            println!("css: {:?}", css);
            assert_eq!(
                css,
                vec![
                    ("a".to_string(), "fill:blue; stroke:red;".to_string()),
                    ("b".to_string(), "fill:red; stroke:black;".to_string())
                ]
            );
        }

        #[test]
        fn test_css_legend2() {
            let input = "# Legend: \n\
                    big_circle = {fill:blue;}\n\
                    red = {stroke:red;}\n\n\n";
            let input_chars: Vec<char> = input.chars().collect();
            let css = css_legend_with_padding()
                .parse(&input_chars)
                .expect("should parse");
            println!("css: {:?}", css);
            assert_eq!(
                css,
                vec![
                    ("big_circle".to_string(), "fill:blue;".to_string()),
                    ("red".to_string(), "stroke:red;".to_string())
                ]
            );
        }
        #[test]
        fn test_css_legend3() {
            let input = "# Legend:\n\
                    big_circle = {fill:blue;}\n\
                    red = {stroke:red;}\n";
            let input_chars: Vec<char> = input.chars().collect();
            let css = css_legend_with_padding()
                .parse(&input_chars)
                .expect("should parse");
            println!("css: {:?}", css);
            assert_eq!(
                css,
                vec![
                    ("big_circle".to_string(), "fill:blue;".to_string()),
                    ("red".to_string(), "stroke:red;".to_string())
                ]
            );
        }

        #[test]
        #[should_panic]
        fn test_css_legend_not_matching() {
            let input = "# Legend1: \n\
                    a = {fill:blue; stroke:red;}\n\
                    b = {fill:red; stroke:black;}";
            let input_chars: Vec<char> = input.chars().collect();
            let css = css_legend().parse(&input_chars).expect("should parse");
            println!("css: {:?}", css);
            assert_eq!(
                css,
                vec![
                    ("a".to_string(), "fill:blue; stroke:red;".to_string()),
                    ("b".to_string(), "fill:red; stroke:black;".to_string())
                ]
            );
        }

        #[test]
        fn test_css_styles_with_new_line() {
            let input = "{fill:blue;\n\
                stroke:red;}";
            let input_chars: Vec<char> = input.chars().collect();
            let css = css_styles().parse(&input_chars).expect("should parse");
            println!("css: {}", css);
            assert_eq!(
                css,
                "fill:blue;\n\
                stroke:red;"
            );
        }

        #[test]
        fn test_tag_class() {
            let input = "{blue_circle}";
            let input_chars: Vec<char> = input.chars().collect();
            let tag = tag_classes().parse(&input_chars).expect("should parse");
            assert_eq!(tag, vec!["blue_circle".to_string()]);
        }

        #[test]
        #[should_panic]
        fn test_invalid_tag_class_not_closed() {
            let input = "{blue_circle";
            let input_chars: Vec<char> = input.chars().collect();
            let tag = tag_classes().parse(&input_chars).expect("should parse");
            assert_eq!(tag, vec!["blue_circle".to_string()]);
        }

        #[test]
        #[should_panic]
        fn test_invalid_tag_class_asterisk() {
            let input = "{blue_*circle}";
            let input_chars: Vec<char> = input.chars().collect();
            let tag = tag_classes().parse(&input_chars).expect("should parse");
            assert_eq!(tag, vec!["blue_circle".to_string()]);
        }

        #[test]
        fn test_tag_classes() {
            let input = "{blue_circle,red_dot}";
            let input_chars: Vec<char> = input.chars().collect();
            let tag = tag_classes().parse(&input_chars).expect("should parse");
            assert_eq!(tag, vec!["blue_circle".to_string(), "red_dot".to_string()]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::CellGrid;

    #[test]
    fn test_collinear() {
        let a = CellGrid::a();
        let b = CellGrid::b();
        let c = CellGrid::c();
        let e = CellGrid::e();
        let f = CellGrid::f();
        assert!(is_collinear(&a, &b, &c));
        assert!(!is_collinear(&f, &c, &e));
        assert!(!is_collinear(&f, &a, &b));
    }

    #[test]
    fn test_padding() {
        assert_eq!(10.0, pad(9.1));
        assert_eq!(-2.0, pad(-1.21));
        assert_eq!(-3.0, pad(-2.99));
    }

    #[test]
    fn test_clip_line() {
        let a = CellGrid::a();
        let b = CellGrid::b();
        let bounds = AABB::new(*a, *b);
        let clip = clip_line(&bounds, a, b);
        println!("clip: {:#?}", clip);
        assert_eq!(clip, Some((a, b)));
    }

    #[test]
    fn test_clip_line3() {
        let c = CellGrid::c();
        let m = CellGrid::m();
        let w = CellGrid::w();
        let bounds = AABB::new(*m, *w);
        let clip = clip_line(&bounds, c, w);
        println!("clip: {:#?}", clip);
        assert_eq!(clip, Some((m, w)));
    }
}
