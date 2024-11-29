use crate::{
    buffer::{
        fragment_buffer::{FragmentSpan, FragmentTree},
        Fragment, StringBuffer,
    },
    fragment::CellText,
    util::parser,
    Merge, Settings,
};
pub use cell::{Cell, CellGrid};
pub use contacts::Contacts;
pub use endorse::Endorse;
use itertools::Itertools;
use sauron::{
    html,
    html::attributes::{class, id},
    html::*,
    svg::{attributes::*, *},
    Node,
};
pub use span::Span;
use std::{
    collections::BTreeMap,
    fmt,
    ops::{Deref, DerefMut},
};
use unicode_width::UnicodeWidthStr;

mod cell;
mod contacts;
mod endorse;
mod span;

/// The simplest buffer.
/// This is maps which char belong to which cell skipping the whitespaces
#[derive(Debug, Default, Clone)]
pub struct CellBuffer {
    map: BTreeMap<Cell, char>,
    /// class, <style>
    /// assemble into
    ///
    /// ```css
    /// .class { styles }
    /// ```
    css_styles: Vec<(String, String)>,
    escaped_text: Vec<(Cell, String)>,
}

impl Deref for CellBuffer {
    type Target = BTreeMap<Cell, char>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for CellBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl CellBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_css_styles(&mut self, css_styles: Vec<(String, String)>) {
        self.css_styles.extend(css_styles);
    }

    pub fn bounds(&self) -> Option<(Cell, Cell)> {
        let xlimits =
            self.iter().map(|(cell, _)| cell.x).minmax().into_option();
        let ylimits =
            self.iter().map(|(cell, _)| cell.y).minmax().into_option();
        match (xlimits, ylimits) {
            (Some((min_x, max_x)), Some((min_y, max_y))) => {
                Some((Cell::new(min_x, min_y), Cell::new(max_x, max_y)))
            }
            _ => None,
        }
    }

    /// get the svg node of this cell buffer, using the default settings for the sizes
    pub fn get_node<MSG>(&self) -> Node<MSG> {
        let (node, _w, _h) = self.get_node_with_size(&Settings::default());
        node
    }

    /// calculate the appropriate size (w,h) in pixels for the whole cell buffer to fit
    /// appropriately
    pub(crate) fn get_size(&self, settings: &Settings) -> (f32, f32) {
        let (_top_left, bottom_right) =
            self.bounds().unwrap_or((Cell::new(0, 0), Cell::new(0, 0)));
        let w = settings.scale * (bottom_right.x + 2) as f32 * Cell::width();
        let h = settings.scale * (bottom_right.y + 2) as f32 * Cell::height();
        (w, h)
    }

    /// get all nodes of this cell buffer
    pub fn get_node_with_size<MSG>(
        &self,
        settings: &Settings,
    ) -> (Node<MSG>, f32, f32) {
        let (w, h) = self.get_size(settings);

        let legend_css = self.legend_css();
        let (group_nodes, fragments) = self.group_nodes_and_fragments(settings);

        let svg_node =
            Self::fragments_to_node(fragments, legend_css, settings, w, h)
                .with_children(group_nodes);
        (svg_node, w, h)
    }

    /// get all nodes and use the size supplied
    pub fn get_node_override_size<MSG>(
        &self,
        settings: &Settings,
        w: f32,
        h: f32,
    ) -> Node<MSG> {
        let legend_css = self.legend_css();
        let (group_nodes, fragments) = self.group_nodes_and_fragments(settings);

        let svg_node =
            Self::fragments_to_node(fragments, legend_css, settings, w, h)
                .with_children(group_nodes);

        svg_node
    }

    /// return the fragments that are (close objects, touching grouped fragments)
    pub fn get_fragment_spans(&self) -> (Vec<FragmentSpan>, Vec<Span>) {
        let escaped_text = self.escaped_text_nodes();

        let group_adjacents = Vec::<Span>::from(self);
        let (endorsed_fragments, vec_spans): (
            Vec<Vec<FragmentSpan>>,
            Vec<Vec<Span>>,
        ) = group_adjacents
            .into_iter()
            .map(|span| span.endorse())
            .map(|endorse| (endorse.accepted, endorse.rejects))
            .unzip();

        let mut accepted: Vec<FragmentSpan> =
            endorsed_fragments.into_iter().flatten().collect();

        let rejects: Vec<Span> = vec_spans.into_iter().flatten().collect();

        accepted.extend(escaped_text);

        (accepted, rejects)
    }

    /// return fragments that are Rect, Circle,
    pub(crate) fn into_shapes_fragment(&self) -> Vec<FragmentSpan> {
        let endorse = self.endorse_to_fragment_spans();
        endorse
            .accepted
            .into_iter()
            .filter(|frag| frag.fragment.is_rect() || frag.fragment.is_circle())
            .collect()
    }

    /// returns (single_member, grouped,  rest of the fragments
    fn endorse_to_fragment_spans(
        &self,
    ) -> Endorse<FragmentSpan, Vec<FragmentSpan>> {
        // endorsed_fragments are the fragment result of successful endorsement
        //
        // vec_groups are not endorsed, but are still touching, these will be grouped together in
        // the svg node
        let group_adjacents: Vec<Span> = self.into();
        let (endorsed_fragments, vec_contacts): (
            Vec<Vec<FragmentSpan>>,
            Vec<Vec<Contacts>>,
        ) = group_adjacents
            .into_iter()
            .map(|span| span.endorse())
            .map(|endorse| {
                (
                    endorse.accepted,
                    endorse
                        .rejects
                        .into_iter()
                        .flat_map(|span| <Vec<Contacts>>::from(span))
                        .collect::<Vec<Contacts>>(),
                )
            })
            .unzip();

        // partition the vec_groups into groups that is alone and the group
        // that is contacting their parts
        let (single_member, vec_groups): (Vec<Contacts>, Vec<Contacts>) =
            vec_contacts
                .into_iter()
                .flatten()
                .partition(move |contacts| contacts.0.len() == 1);

        let single_member_fragments: Vec<FragmentSpan> = single_member
            .into_iter()
            .flat_map(|contact| contact.as_ref().to_vec())
            .collect();

        let rejects: Vec<Vec<FragmentSpan>> = vec_groups
            .into_iter()
            .map(|contact| contact.as_ref().to_vec())
            .collect();

        let accepted: Vec<FragmentSpan> = endorsed_fragments
            .into_iter()
            .flatten()
            .chain(single_member_fragments.into_iter())
            .collect();
        Endorse { accepted, rejects }
    }

    /// group nodes that can be group and the rest will be fragments
    /// Note: The grouped fragments is scaled here
    fn group_nodes_and_fragments<MSG>(
        &self,
        settings: &Settings,
    ) -> (Vec<Node<MSG>>, Vec<FragmentSpan>) {
        let escaped_text_nodes = self.escaped_text_nodes();
        let Endorse { accepted, rejects } = self.endorse_to_fragment_spans();

        // grouped fragments will be rendered as svg groups
        let group_nodes: Vec<Node<MSG>> = rejects
            .into_iter()
            .map(move |fragments| {
                let group_members = fragments
                    .iter()
                    .map(move |gfrag| {
                        let scaled = gfrag.scale(settings.scale);
                        let node: Node<MSG> = scaled.fragment.into();
                        node
                    })
                    .collect::<Vec<Node<MSG>>>();
                g([], group_members)
            })
            .collect();

        let mut fragments: Vec<FragmentSpan> = accepted;

        fragments.extend(escaped_text_nodes);

        (group_nodes, fragments)
    }

    fn escaped_text_nodes(&self) -> Vec<FragmentSpan> {
        self.escaped_text
            .iter()
            .map(|(cell, text)| CellText::new(*cell, text.clone()).into())
            .collect()
    }

    /// construct the css from the # Legend: of the diagram
    fn legend_css(&self) -> String {
        let classes: Vec<String> = self
            .css_styles
            .iter()
            .map(|(class, styles)| {
                format!(".svgbob .{}{{ {} }}", class, styles)
            })
            .collect();
        classes.join("\n")
    }

    fn style<MSG>(settings: &Settings, legend_css: String) -> Node<MSG> {
        use sauron::html::units::px;

        let stroke_color = settings.stroke_color.to_owned();
        let stroke_width = settings.stroke_width.to_owned();
        let background = settings.background.to_owned();
        let fill_color = settings.fill_color.to_owned();
        let font_family = settings.font_family.to_owned();
        let font_size = settings.font_size.to_owned();

        // we put a .svgbob class in order to avoid clashing with other svg in the document
        // since the style element in this svg also affects the other svg element in the whole
        // document
        let element_styles = sauron::jss! {
                ".svgbob line, .svgbob path, .svgbob circle, .svgbob rect, .svgbob polygon": {
                      stroke: stroke_color.clone(),
                      stroke_width: stroke_width,
                      stroke_opacity: 1,
                      fill_opacity: 1,
                      stroke_linecap: "round",
                      stroke_linejoin: "miter",
                },

                ".svgbob text": {
                    /* This fix the spacing bug in svg text*/
                    white_space: "pre",
                    fill: stroke_color,
                    font_family: font_family,
                    font_size: px(font_size),
                },

                ".svgbob rect.backdrop":{
                    stroke: "none",
                    fill: background.clone(),
                },

                ".svgbob .broken":{
                    stroke_dasharray: 8,
                },

                ".svgbob .filled":{
                    fill: fill_color,
                },

                ".svgbob .bg_filled":{
                    fill: background.clone(),
                    stroke_width: 1,
                },

                ".svgbob .nofill":{
                    fill: background,
                },

                ".svgbob .end_marked_arrow":{
                    marker_end: "url(#arrow)",
                },

                ".svgbob .start_marked_arrow":{
                    marker_start: "url(#arrow)",
                },

                ".svgbob .end_marked_diamond":{
                    marker_end: "url(#diamond)",
                },
                ".svgbob .start_marked_diamond":{
                    marker_start: "url(#diamond)",
                },

                ".svgbob .end_marked_circle":{
                    marker_end: "url(#circle)",
                },

                ".svgbob .start_marked_circle":{
                    marker_start: "url(#circle)",
                },

                ".svgbob .end_marked_open_circle":{
                    marker_end: "url(#open_circle)",
                },

                ".svgbob .start_marked_open_circle":{
                    marker_start: "url(#open_circle)",
                },

                ".svgbob .end_marked_big_open_circle":{
                    marker_end: "url(#big_open_circle)",
                },

                ".svgbob .start_marked_big_open_circle": {
                    marker_start: "url(#big_open_circle)",
                }
        };

        // Combine the css, so as not to have a <!-- separator --> comment which
        // was intended only for text node added after a previous text node.
        let css = [element_styles, legend_css].join("\n");
        html::tags::style([], [text(css)])
    }

    /// convert the fragments into svg nodes using the supplied settings, with size for the
    /// dimension
    pub fn fragments_to_node<MSG>(
        fragments: Vec<FragmentSpan>,
        legend_css: String,
        settings: &Settings,
        w: f32,
        h: f32,
    ) -> Node<MSG> {
        let fragments_scaled: Vec<FragmentSpan> = fragments
            .into_iter()
            .map(|frag| frag.scale(settings.scale))
            .collect();
        let fragment_nodes: Vec<Node<MSG>> =
            FragmentTree::fragments_to_node(fragments_scaled);

        let mut children = vec![];
        if settings.include_styles {
            children.push(Self::style(settings, legend_css));
        }
        if settings.include_defs {
            children.push(Self::get_defs());
        }

        // backdrop needs to appear first before the fragment nodes
        // otherwise it will cover the other elements
        // in accordance to how z-index works
        if settings.include_backdrop {
            children.push(rect(
                [class("backdrop"), x(0), y(0), width(w), height(h)],
                [],
            ));
        }

        children.extend(fragment_nodes);

        svg(
            [
                xmlns("http://www.w3.org/2000/svg"),
                width(w),
                height(h),
                class("svgbob"),
            ],
            children,
        )
    }

    fn get_defs<MSG>() -> Node<MSG> {
        defs(
            [],
            [
                Self::arrow_marker(),
                Self::diamond_marker(),
                Self::circle_marker(),
                Self::open_circle_marker(),
                Self::big_open_circle_marker(),
            ],
        )
    }

    fn arrow_marker<MSG>() -> Node<MSG> {
        marker(
            [
                id("arrow"),
                view_box("-2 -2 8 8"),
                ref_x(4),
                ref_y(2),
                marker_width(7),
                marker_height(7),
                orient("auto-start-reverse"),
            ],
            [polygon([points("0,0 0,4 4,2 0,0")], [])],
        )
    }

    fn diamond_marker<MSG>() -> Node<MSG> {
        marker(
            [
                id("diamond"),
                view_box("-2 -2 8 8"),
                ref_x(4),
                ref_y(2),
                marker_width(7),
                marker_height(7),
                orient("auto-start-reverse"),
            ],
            [polygon([points("0,2 2,0 4,2 2,4 0,2")], [])],
        )
    }

    fn open_circle_marker<MSG>() -> Node<MSG> {
        marker(
            [
                id("open_circle"),
                view_box("0 0 8 8"),
                ref_x(4),
                ref_y(4),
                marker_width(7),
                marker_height(7),
                orient("auto-start-reverse"),
            ],
            [circle(
                [cx(4), cy(4), r(2), html::attributes::class("bg_filled")],
                [],
            )],
        )
    }

    fn circle_marker<MSG>() -> Node<MSG> {
        marker(
            [
                id("circle"),
                view_box("0 0 8 8"),
                ref_x(4),
                ref_y(4),
                marker_width(7),
                marker_height(7),
                orient("auto-start-reverse"),
            ],
            [circle(
                [cx(4), cy(4), r(2), html::attributes::class("filled")],
                [],
            )],
        )
    }

    fn big_open_circle_marker<MSG>() -> Node<MSG> {
        marker(
            [
                id("big_open_circle"),
                view_box("0 0 8 8"),
                ref_x(4),
                ref_y(4),
                marker_width(7),
                marker_height(7),
                orient("auto-start-reverse"),
            ],
            [circle(
                [cx(4), cy(4), r(3), html::attributes::class("bg_filled")],
                [],
            )],
        )
    }

    /// returns a (Cell, escaped string), and the strings that are not part of the escape string
    fn escape_line(line: usize, raw: &str) -> (Vec<(Cell, String)>, String) {
        let mut no_escaped_text = String::new();

        let mut index = 0;
        let mut escaped_text = vec![];
        let input_chars: Vec<char> = raw.chars().collect();
        let char_locs = parser::line_parse()
            .parse(&input_chars)
            .expect("should parse");
        if char_locs.is_empty() {
            no_escaped_text = raw.to_string();
        } else {
            for (start, end) in char_locs.iter() {
                let escaped = input_chars[*start + 1..*end].iter().fold(
                    String::new(),
                    |mut acc, c| {
                        acc.push(*c);
                        acc
                    },
                );
                let escaped_unicode_width = escaped.width();
                let cell = Cell::new(*start as i32, line as i32);
                escaped_text.push((cell, escaped));
                no_escaped_text += &input_chars[index..*start].iter().fold(
                    String::new(),
                    |mut acc, c| {
                        acc.push(*c);
                        acc
                    },
                );

                // we add 2 to account for the double quotes on end of the escaped string
                no_escaped_text += &" ".repeat(escaped_unicode_width + 2);
                index = end + 1;
            }
            // include the rest of the text
            no_escaped_text += &input_chars[index..].iter().fold(
                String::new(),
                |mut acc, c| {
                    acc.push(*c);
                    acc
                },
            );
        }
        (escaped_text, no_escaped_text)
    }
}

impl fmt::Display for CellBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "CellBuffer dump..")?;
        for (cell, ch) in self.iter() {
            writeln!(f, "{} {}", cell, ch)?;
        }
        Ok(())
    }
}

impl From<&str> for CellBuffer {
    fn from(input: &str) -> Self {
        let css_styles = if let Some(loc) = input.find("# Legend:") {
            if let Ok(css_styles) = parser::parse_css_legend(&input[loc..]) {
                Some((loc, css_styles))
            } else {
                None
            }
        } else {
            None
        };
        if let Some((loc, css_styles)) = css_styles {
            let mut cell_buffer =
                CellBuffer::from(StringBuffer::from(&input[..loc]));
            cell_buffer.add_css_styles(css_styles);
            cell_buffer
        } else {
            CellBuffer::from(StringBuffer::from(input))
        }
    }
}

impl From<StringBuffer> for CellBuffer {
    fn from(sb: StringBuffer) -> Self {
        use std::iter::FromIterator;

        let mut buffer = CellBuffer::new();
        for (y, line) in sb.iter().enumerate() {
            let line_str = String::from_iter(line);
            let (escaped_text, unescaped) = Self::escape_line(y, &line_str);
            buffer.escaped_text.extend(escaped_text);

            for (x, ch) in unescaped.chars().enumerate() {
                if ch != '\0' && !ch.is_whitespace() {
                    let cell = Cell::new(x as i32, y as i32);
                    buffer.insert(cell, ch);
                }
            }
        }
        buffer
    }
}

/// Groups cell that are adjacents (cells that are next to each other, horizontally or
/// vertically)
/// Note: using .rev() since this has a high change that the last cell is adjacent with the
/// current cell tested
impl From<&CellBuffer> for Vec<Span> {
    fn from(cb: &CellBuffer) -> Vec<Span> {
        let spans: Vec<Span> =
            cb.iter().map(|(cell, ch)| Span::new(*cell, *ch)).collect();
        Span::merge_recursive(spans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_line() {
        let raw = r#"The "qu/i/ck" brown "fox\"s" jumps over the lazy "do|g""#;
        let ex2 = r#"The           brown          jumps over the lazy       "#;
        let (escaped, unescaped) = CellBuffer::escape_line(0, raw);
        println!("escaped: {:#?}", escaped);
        println!("unescaped: {}", unescaped);
        assert_eq!(
            vec![
                (Cell::new(4, 0), "qu/i/ck".to_string()),
                (Cell::new(20, 0), r#"fox\"s"#.to_string()),
                (Cell::new(49, 0), "do|g".to_string())
            ],
            escaped
        );
        assert_eq!(ex2, unescaped);
    }

    #[test]
    fn test_escape_line2() {
        let raw = r#"The quick brown fox jumps over the "lazy" dog"#;
        let ex2 = r#"The quick brown fox jumps over the        dog"#;
        let (escaped, unescaped) = CellBuffer::escape_line(0, raw);
        println!("escaped: {:#?}", escaped);
        println!("unescaped: {}", unescaped);
        assert_eq!(vec![(Cell::new(35, 0), "lazy".to_string())], escaped);
        assert_eq!(ex2, unescaped);
    }

    #[test]
    fn test_escape_line3() {
        let raw = r#" in between "|      |" these "#;
        let ex2 = r#" in between            these "#;
        let (escaped, unescaped) = CellBuffer::escape_line(0, raw);
        println!("escaped: {:#?}", escaped);
        println!("unescaped: {}", unescaped);
        assert_eq!(vec![(Cell::new(12, 0), "|      |".to_string())], escaped);
        assert_eq!(ex2, unescaped);
    }

    #[test]
    fn test_issue38_escaped_german_umlauts() {
        let raw = r#"This is some german character "ÖÄÜ" and was escaped"#;
        let ex2 = r#"This is some german character       and was escaped"#;
        let (escaped, unescaped) = CellBuffer::escape_line(0, raw);
        println!("escaped: {:#?}", escaped);
        println!("unescaped: {}", unescaped);
        assert_eq!(vec![(Cell::new(30, 0), "ÖÄÜ".to_string())], escaped);
        assert_eq!(ex2, unescaped);
    }

    #[test]
    fn test_issue38_escape_cjk() {
        let raw = r#"This is some CJK "一" and was escaped"#;
        let ex2 = r#"This is some CJK      and was escaped"#;
        let (escaped, unescaped) = CellBuffer::escape_line(0, raw);
        println!("escaped: {:#?}", escaped);
        println!("unescaped: {}", unescaped);
        assert_eq!(vec![(Cell::new(17, 0), r#"一"#.to_string())], escaped);
        assert_eq!(ex2, unescaped);
    }

    #[test]
    fn test_simple_adjacents() {
        let art = r#"
..     ._.
''    (   )
       `-'

.--------.
|________|
    "#;
        let buffer = CellBuffer::from(art);
        let adjacents: Vec<Span> = buffer.into();
        for (i, span) in adjacents.iter().enumerate() {
            println!("span: {}", i);
            println!("{}\n\n", span);
        }
        assert_eq!(adjacents.len(), 3);
    }

    #[test]
    fn test_shapes_fragment() {
        let art = r#"

            +-------+
 *----->    |       |
            +-------+

This is a text

            ,-.
           (   )
            `-'

    "#;
        let buffer = CellBuffer::from(art);
        let shapes = buffer.into_shapes_fragment();
        println!("shapes: {:#?}", shapes);
        assert_eq!(2, shapes.len());
    }

    #[test]
    fn test_shapes_fragment_intersecting() {
        let art = r#"
            +-------+
            |       |
            +-------+

    "#;
        let buffer = CellBuffer::from(art);
        let shapes = buffer.into_shapes_fragment();
        println!("shapes: {:#?}", shapes);
        assert_eq!(1, shapes.len());
        assert!(shapes[0]
            .fragment
            .hit(Cell::new(15, 1).a(), Cell::new(15, 1).y()));
    }

    /// The . in .-/
    /// will create a new contacts even since it is not adjacent to /
    /// so it needs a second_pass to merge it
    #[test]
    fn test_one_big() {
        let art = r#"

       .---.
      /-o-/--
   .-/ / /->
  ( *  \/
   '-.  \
      \ /
       '
    "#;
        let buffer = CellBuffer::from(art);
        let adjacents: Vec<Span> = buffer.into();
        for (i, span) in adjacents.iter().enumerate() {
            println!("span: {}", i);
            println!("{}\n\n", span);
        }
        assert_eq!(adjacents.len(), 1);
    }

    #[test]
    fn test_one_bigger() {
        let art = r#"

                                       .------------>---------------.
           ┌-------------┐  .-.   .-.  |  ┌------┐  .-.   ┌-----┐   |    .-.   ┌------┐
      O____| struct_name |_( : )_( | )_◞__| name |_( : )__| tpe |___◟___( | )__| body |______O
        ◝  └-------------┘  `-'   `-'   ◜ └------┘  `-'   └-----┘  ◝     `-'   └------┘  ◜
        |                               |                    .-.   |                     |
        |                               `------------<------( , )--'                     |
        |                                                    `-'                         |
        `--------------------------------------------------------------------------------'

    "#;
        let buffer = CellBuffer::from(art);
        let adjacents: Vec<Span> = buffer.into();
        for (i, span) in adjacents.iter().enumerate() {
            println!("span: {}", i);
            println!("{}\n\n", span);
        }
        assert_eq!(adjacents.len(), 1);
    }
}
