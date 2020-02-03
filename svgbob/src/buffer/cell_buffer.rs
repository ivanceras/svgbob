use crate::{
    buffer::{fragment_buffer::FragmentTree, Fragment, StringBuffer},
    util::parser,
};
pub use cell::{Cell, CellGrid};
pub use contacts::Contacts;
use itertools::Itertools;
use sauron::{
    html,
    html::{attributes::*, *},
    svg,
    svg::{attributes::*, *},
    Node,
};
pub use settings::Settings;
pub use span::Span;
use std::{
    collections::BTreeMap,
    fmt,
    ops::{Deref, DerefMut},
};

mod cell;
mod contacts;
mod endorse;
mod settings;
mod span;

/// The simplest buffer.
/// This is maps which char belong to which cell skipping the whitespaces
#[derive(Debug)]
pub struct CellBuffer {
    map: BTreeMap<Cell, char>,
    /// class, <style>
    /// assemble into
    ///
    /// ```css
    /// .class { styles }
    /// ```
    css_styles: Vec<(String, String)>,
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
        CellBuffer { map: BTreeMap::new(), css_styles: vec![] }
    }

    pub fn add_css_styles(&mut self, css_styles: Vec<(String, String)>) {
        self.css_styles.extend(css_styles);
    }

    /// Groups cell that are adjacents
    /// Note: using .rev() since this has a high change that the last cell is adjacent with the
    /// current cell tested
    pub fn group_adjacents(&self) -> Vec<Span> {
        let mut adjacents: Vec<Span> = vec![];
        for (cell, ch) in self.iter() {
            let belongs_to_adjacents = adjacents.iter_mut().rev().any(|contacts| {
                if contacts.is_adjacent(cell) {
                    contacts.push((*cell, *ch));
                    true
                } else {
                    false
                }
            });
            if !belongs_to_adjacents {
                adjacents.push(Span::new(*cell, *ch));
            }
        }
        Self::merge_recursive(adjacents)
    }

    /// merge span recursively until it hasn't changed the number of spans
    fn merge_recursive(adjacents: Vec<Span>) -> Vec<Span> {
        let original_len = adjacents.len();
        let merged = Self::second_pass_merge(adjacents);
        // if has merged continue merging until nothing can be merged
        if merged.len() < original_len { Self::merge_recursive(merged) } else { merged }
    }

    /// second pass merge is operating on span comparing to other spans
    fn second_pass_merge(adjacents: Vec<Span>) -> Vec<Span> {
        let mut new_groups: Vec<Span> = vec![];
        for span in adjacents.into_iter() {
            let is_merged = new_groups.iter_mut().rev().any(|new_group| {
                if new_group.can_merge(&span) {
                    new_group.merge(&span);
                    true
                } else {
                    false
                }
            });
            if !is_merged {
                new_groups.push(span);
            }
        }
        new_groups
    }

    fn bounds(&self) -> Option<(Cell, Cell)> {
        let xlimits = self.iter().map(|(cell, _)| cell.x).minmax().into_option();
        let ylimits = self.iter().map(|(cell, _)| cell.y).minmax().into_option();
        match (xlimits, ylimits) {
            (Some((min_x, max_x)), Some((min_y, max_y))) => {
                Some((Cell::new(min_x, min_y), Cell::new(max_x, max_y)))
            }
            _ => None,
        }
    }

    /// get the svg node of this cell buffer, using the default settings for the sizes
    pub fn get_node(&self) -> Node<()> {
        let (node, _w, _h) = self.get_node_with_size(&Settings::default());
        node
    }

    /// calculate the appropriate size (w,h) in pixels for the whole cell buffer to fit
    /// appropriately
    pub(crate) fn get_size(&self, settings: &Settings) -> (f32, f32) {
        let (top_left, bottom_right) = self.bounds().unwrap_or((Cell::new(0, 0), Cell::new(0, 0)));
        let w = settings.scale * (bottom_right.x + 2) as f32 * Cell::width();
        let h = settings.scale * (bottom_right.y + 2) as f32 * Cell::height();
        (w, h)
    }

    /// get all nodes of this cell buffer
    pub fn get_node_with_size(&self, settings: &Settings) -> (Node<()>, f32, f32) {
        let (w, h) = self.get_size(&settings);
        // vec_fragments are the fragment result of successful endorsement
        //
        // vec_groups are not endorsed, but are still touching, these will be grouped together in
        // the svg node
        let (vec_fragments, vec_contacts): (Vec<Vec<Fragment>>, Vec<Vec<Contacts>>) =
            self.group_adjacents().into_iter().map(|span| span.endorse()).unzip();


        // partition the vec_groups into groups that is alone and the group
        // that is contacting their parts
        let (single_member, vec_groups): (Vec<Contacts>, Vec<Contacts>) =
            vec_contacts.into_iter().flatten().partition(move |contacts| contacts.0.len() == 1);

        let single_member_fragments: Vec<Fragment> =
            single_member.into_iter().flat_map(|contact| contact.0).collect();

        let group_nodes: Vec<Node<()>> = vec_groups
            .into_iter()
            .map(|contact| contact.0)
            .map(move |contacts| {
                let mut group_members = contacts
                    .iter()
                    .map(move |gfrag| gfrag.scale(settings.scale).into())
                    .collect::<Vec<_>>();
                g(vec![], group_members)
            })
            .collect();

        let mut fragments: Vec<Fragment> = vec_fragments.into_iter().flatten().collect();
        fragments.extend(single_member_fragments);
        let mut svg_node = Self::fragments_to_node(fragments, self.legend_css(), settings, w, h)
            .add_children(group_nodes);
        (svg_node, w, h)
    }

    /// construct the css from the # Legend: of the diagram
    fn legend_css(&self) -> String {
        let classes: Vec<String> = self
            .css_styles
            .iter()
            .map(|(class, styles)| format!(".{}{{ {} }}", class, styles))
            .collect();
        classes.join("\n")
    }

    fn get_style(settings: &Settings, legend_css: String) -> Node<()> {
        html::tags::style(vec![], vec![text(format!(
            "line, path, circle,rect,polygon {{
                          stroke: {stroke_color};
                          stroke-width: {stroke_width};
                          stroke-opacity: 1;
                          fill-opacity: 1;
                          stroke-linecap: round;
                          stroke-linejoin: miter;
                        }}

                    text {{
                        fill: {stroke_color};
                        }}
                        rect.backdrop{{
                            stroke: none;
                            fill: {background};
                        }}
                        .broken{{
                            stroke-dasharray: 8;
                        }}
                        .filled{{
                            fill: {fill_color};
                        }}
                        .bg_filled{{
                            fill: {background};
                        }}
                        .nofill{{
                            fill: none;
                        }}

                        text {{
                         font-family: {font_family};
                         font-size: {font_size}px;
                        }}

                        .end_marked_arrow{{
                            marker-end: url(#arrow);
                         }}
                        .start_marked_arrow{{
                            marker-start: url(#arrow);
                         }}

                        .end_marked_diamond{{
                            marker-end: url(#diamond);
                         }}
                        .start_marked_diamond{{
                            marker-start: url(#diamond);
                         }}

                        .end_marked_circle{{
                            marker-end: url(#circle);
                         }}
                        .start_marked_circle{{
                            marker-start: url(#circle);
                         }}

                        .end_marked_open_circle{{
                            marker-end: url(#open_circle);
                         }}
                        .start_marked_open_circle{{
                            marker-start: url(#open_circle);
                         }}

                        .end_marked_big_open_circle{{
                            marker-end: url(#big_open_circle);
                         }}
                        .start_marked_big_open_circle{{
                            marker-start: url(#big_open_circle);
                         }}

                         {legend_css}
                        ",
            background = settings.background,
            fill_color = settings.fill_color,
            stroke_color = settings.stroke_color,
            stroke_width = settings.stroke_width,
            font_size = settings.font_size,
            font_family = settings.font_family,
            legend_css = legend_css,
        ))])
    }

    /// convert the fragments into svg nodes using the supplied settings, with size for the
    /// dimension
    pub(crate) fn fragments_to_node(
        fragments: Vec<Fragment>,
        legend_css: String,
        settings: &Settings,
        w: f32,
        h: f32,
    ) -> Node<()> {
        let fragments_scaled: Vec<Fragment> =
            fragments.into_iter().map(|frag| frag.scale(settings.scale)).collect();
        let fragment_nodes: Vec<Node<()>> = FragmentTree::fragments_to_node(fragments_scaled);


        let svg_node = svg(vec![xmlns("http://www.w3.org/2000/svg"), width(w), height(h)], vec![
            Self::get_style(settings, legend_css),
            Self::get_defs(),
            rect(vec![class("backdrop"), x(0), y(0), width(w), height(h)], vec![]),
        ])
        .add_children(fragment_nodes);
        svg_node
    }

    fn get_defs() -> Node<()> {
        defs(vec![], vec![
            Self::arrow_marker(),
            Self::diamond_marker(),
            Self::circle_marker(),
            Self::open_circle_marker(),
            Self::big_open_circle_marker(),
        ])
    }

    fn arrow_marker() -> Node<()> {
        marker(
            vec![
                id("arrow"),
                viewBox("-2 -2 8 8"),
                refX(4),
                refY(2),
                markerWidth(7),
                markerHeight(7),
                orient("auto-start-reverse"),
            ],
            vec![polygon(vec![points("0,0 0,4 4,2 0,0")], vec![])],
        )
    }

    fn diamond_marker() -> Node<()> {
        marker(
            vec![
                id("diamond"),
                viewBox("-2 -2 8 8"),
                refX(4),
                refY(2),
                markerWidth(7),
                markerHeight(7),
                orient("auto-start-reverse"),
            ],
            vec![polygon(vec![points("0,2 2,0 4,2 2,4 0,2")], vec![])],
        )
    }

    fn open_circle_marker() -> Node<()> {
        marker(
            vec![
                id("open_circle"),
                viewBox("0 0 8 8"),
                refX(4),
                refY(4),
                markerWidth(7),
                markerHeight(7),
                orient("auto-start-reverse"),
            ],
            vec![circle(vec![cx(4), cy(4), r(2), html::attributes::class("bg_filled")], vec![])],
        )
    }

    fn circle_marker() -> Node<()> {
        marker(
            vec![
                id("circle"),
                viewBox("0 0 8 8"),
                refX(4),
                refY(4),
                markerWidth(7),
                markerHeight(7),
                orient("auto-start-reverse"),
            ],
            vec![circle(vec![cx(4), cy(4), r(2), html::attributes::class("filled")], vec![])],
        )
    }

    fn big_open_circle_marker() -> Node<()> {
        marker(
            vec![
                id("big_open_circle"),
                viewBox("0 0 8 8"),
                refX(4),
                refY(4),
                markerWidth(7),
                markerHeight(7),
                orient("auto-start-reverse"),
            ],
            vec![circle(vec![cx(4), cy(4), r(3), html::attributes::class("bg_filled")], vec![])],
        )
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
            let mut cell_buffer = CellBuffer::from(StringBuffer::from(&input[..loc]));
            cell_buffer.add_css_styles(css_styles);
            cell_buffer
        } else {
            CellBuffer::from(StringBuffer::from(input))
        }
    }
}

impl From<StringBuffer> for CellBuffer {
    fn from(sb: StringBuffer) -> Self {
        let mut buffer = CellBuffer::new();
        for (y, line) in sb.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if *ch != '\0' && !ch.is_whitespace() {
                    let cell = Cell::new(x as i32, y as i32);
                    buffer.insert(cell, *ch);
                }
            }
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;


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
        let adjacents = buffer.group_adjacents();
        for (i, span) in adjacents.iter().enumerate() {
            println!("span: {}", i);
            println!("{}\n\n", span);
        }
        assert_eq!(adjacents.len(), 3);
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
        let adjacents = buffer.group_adjacents();
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
        let adjacents = buffer.group_adjacents();
        for (i, span) in adjacents.iter().enumerate() {
            println!("span: {}", i);
            println!("{}\n\n", span);
        }
        assert_eq!(adjacents.len(), 1);
    }
}
