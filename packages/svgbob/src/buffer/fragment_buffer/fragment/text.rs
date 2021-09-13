use crate::{
    buffer::{Cell, CellGrid},
    fragment::Bounds,
    Point,
};
use sauron::{html::*, svg, svg::attributes::*, Node};
use std::{borrow::Cow, cmp::Ordering, fmt};

/// A horizontal cell text
/// Operated based on cell
/// Text are threated differently
/// since scaling them losses the sense
/// of which text it adjacent to without keeping
/// track of the scale.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CellText {
    pub start: Cell,
    pub content: String,
}

impl CellText {
    pub fn new(start: Cell, content: String) -> Self {
        CellText { start, content }
    }

    fn end_cell(&self) -> Cell {
        Cell::new(self.start.x + self.content.len() as i32, self.start.y)
    }

    /// get the cells of this text
    /// TODO: use iterator
    fn cells(&'_ self) -> impl IntoIterator<Item = Cell> + '_ {
        let range = self.start.x..(self.start.x + self.content.len() as i32);
        range.map(move |x| Cell::new(x, self.start.y))
    }

    fn is_adjacent_cell(&self, other_cell: Cell) -> bool {
        self.cells()
            .into_iter()
            .any(|cell| cell.y == other_cell.y && cell.is_adjacent(&other_cell))
    }

    /// cell text is groupable when they are adjacent
    pub(crate) fn is_contacting(&self, other: &Self) -> bool {
        self.cells()
            .into_iter()
            .any(|cell| other.is_adjacent_cell(cell))
    }

    /// text can merge if they are next to each other and at the same line
    pub(in crate) fn can_merge(&self, other: &Self) -> bool {
        self.start.y == other.start.y
            && (self.start.x + self.content.len() as i32 == other.start.x
                || other.start.x + other.content.len() as i32 == self.start.x)
    }

    pub(in crate) fn merge(&self, other: &Self) -> Option<Self> {
        if self.can_merge(other) {
            if self.start.x < other.start.x {
                Some(CellText::new(
                    self.start,
                    format!("{}{}", self.content, other.content),
                ))
            } else {
                Some(CellText::new(
                    other.start,
                    format!("{}{}", other.content, self.content),
                ))
            }
        } else {
            None
        }
    }

    pub(in crate) fn absolute_position(&self, cell: Cell) -> Self {
        CellText {
            start: Cell::new(self.start.x + cell.x, self.start.y + cell.y),
            ..self.clone()
        }
    }
}

impl Bounds for CellText {
    fn bounds(&self) -> (Point, Point) {
        (
            self.start.top_left_most(),
            self.end_cell().bottom_right_most(),
        )
    }
}

impl Into<Text> for CellText {
    fn into(self) -> Text {
        Text::new(self.start.q(), self.content)
    }
}

impl<MSG> Into<Node<MSG>> for CellText {
    fn into(self) -> Node<MSG> {
        let text: Text = self.into();
        text.into()
    }
}

impl fmt::Display for CellText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CT {} {}", self.start, self.content)
    }
}

/// This is ready to be scaled and drawn into the svg file
#[derive(Debug, Clone)]
pub struct Text {
    pub start: Point,
    pub text: String,
}

impl Text {
    pub fn new(start: Point, text: String) -> Self {
        Text { start, text }
    }

    /// get the textwidth in terms of cell grid points
    fn text_width(&self) -> f32 {
        self.text.len() as f32 * CellGrid::width()
    }

    pub(in crate) fn absolute_position(&self, cell: Cell) -> Self {
        Text {
            start: cell.absolute_position(self.start),
            ..self.clone()
        }
    }

    pub(in crate) fn scale(&self, scale: f32) -> Self {
        Text {
            start: self.start.scale(scale),
            ..self.clone()
        }
    }
}

fn replace_html_char<'a>(ch: char) -> Cow<'a, str> {
    match ch {
        '>' => Cow::from("&gt;"),
        '<' => Cow::from("&lt;"),
        '&' => Cow::from("&amp;"),
        '\'' => Cow::from("&#39;"),
        '"' => Cow::from("&quot;"),
        '\0' => Cow::from(""),
        _ => Cow::from(ch.to_string()),
    }
}

fn escape_html_text(s: &str) -> String {
    s.chars().map(|ch| replace_html_char(ch)).collect()
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "T {} {}", self.start, self.text)
    }
}

impl<MSG> Into<Node<MSG>> for Text {
    fn into(self) -> Node<MSG> {
        svg::tags::text(
            [x(self.start.x), y(self.start.y)],
            #[cfg(not(feature = "with-dom"))]
            [text(escape_html_text(&self.text))],
            #[cfg(feature = "with-dom")]
            [text(&self.text)],
        )
    }
}

impl Bounds for Text {
    fn bounds(&self) -> (Point, Point) {
        (
            self.start,
            Point::new(self.start.x + self.text_width(), self.start.y),
        )
    }
}

impl Eq for Text {}

/// This is needed since this struct contains f32 which rust doesn't provide Eq implementation
impl Ord for Text {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then(self.text.cmp(&other.text))
    }
}

impl PartialOrd for Text {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        buffer::{Cell, CellBuffer, Contacts, Span},
        fragment::CellText,
        Settings,
    };

    #[test]
    fn test_cell_text_absolute_position() {
        let t1 = CellText::new(Cell::new(0, 0), "c".to_string());
        let t1_abs = t1.absolute_position(Cell::new(1, 1));
        assert_eq!(t1_abs.start, Cell::new(1, 1));

        let t2 = CellText::new(Cell::new(4, 3), "c".to_string());
        let t2_abs = t2.absolute_position(Cell::new(5, 7));
        assert_eq!(t2_abs.start, Cell::new(9, 10));
    }

    #[test]
    fn text_merge_point_base() {
        let c1 = Cell::new(1, 1);
        let c2 = Cell::new(3, 1);
        let t1 = CellText::new(c1, "He".to_string());
        let t2 = CellText::new(c2, "llo".to_string());
        assert!(t1.can_merge(&t2));
        assert_eq!(t2.merge(&t1), t1.merge(&t2));
        assert_eq!(t1.merge(&t2), Some(CellText::new(c1, "Hello".to_string())));
    }

    #[test]
    fn test_paragraph() {
        let art = r#"
        This is some sort of a paragraph
        with long words such as supercalifragilisticexpialiducios
        and pneumonoultramicrospocisilicovulcanoconiosis
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.localize().get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        assert_eq!(15, groups.len());
        assert_eq!(
            groups[0],
            Contacts::new(
                CellText::new(Cell::new(0, 0), "This".to_string()).into()
            )
        );
        assert_eq!(
            groups[5],
            Contacts::new(
                CellText::new(Cell::new(21, 0), "a".to_string()).into()
            )
        );
        assert_eq!(
            groups[14],
            Contacts::new(
                CellText::new(
                    Cell::new(4, 2),
                    "pneumonoultramicrospocisilicovulcanoconiosis".to_string()
                )
                .into()
            )
        );
    }

    #[test]
    fn test_2paragraph() {
        let art = r#"
        This is some sort of a paragraph
        with long words such as supercalifragilisticexpialiducios
        and pneumonoultramicrospocisilicovulcanoconiosis

        Loren ipsum is a second paragrapah.
        This is in one span since all the letters
        are adjacent since they are right next to
        each other taking account the top and bottom
        neighbor cell
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 2);
        let settings = &Settings::default();
        let groups2 = spans.remove(1).localize().get_contacts(settings);
        let groups1 = spans.remove(0).localize().get_contacts(settings);
        println!("span1 groups:");
        for (i, group1) in groups1.iter().enumerate() {
            println!("\tgroup {} {}", i, group1);
        }
        println!("span2 groups:");
        for (i, group2) in groups2.iter().enumerate() {
            println!("\tgroup {} {}", i, group2);
        }
        assert_eq!(groups1.len(), 15);
        assert_eq!(groups2.len(), 33);
        assert_eq!(
            groups1[0].as_ref()[0].as_cell_text().unwrap().start,
            Cell::new(0, 0)
        );
        assert_eq!(
            groups2[0].as_ref()[0].as_cell_text().unwrap().start,
            Cell::new(0, 0)
        );
    }
}
