use crate::{buffer::Span, Cell, Merge, Settings};
pub use direction::Direction;
pub use fragment::Fragment;
pub use fragment_span::FragmentSpan;
pub use fragment_tree::FragmentTree;
use itertools::Itertools;
use std::{
    collections::BTreeMap,
    fmt::Write,
    ops::{Deref, DerefMut},
};

pub mod direction;
pub mod fragment;
mod fragment_span;
mod fragment_tree;

/// Fragment buffer contains the drawing fragments for each cell
/// Svg can be converted to fragment buffer
/// then from the fragment we can match which characters is best suited for
/// a particular set of fragment contained in a cell and then create a stringbuffer.
/// The stringbuffer becomes the ascii diagrams
///  SVG -> FragmentBuffer -> StringBuffer -> Ascii Diagrams
///
/// We can also create a reverse
///  Ascii Diagrams -> String Buffer -> Fragment Buffer -> SVG
///
/// ```ignore
///      0 1 2 3 4           B C D
///     0┌─┬─┬─┬─┐        A┌─┬─┬─┬─┐E
///     1├─┼─┼─┼─┤         │ │ │ │ │
///     2├─┼─┼─┼─┤        F├─G─H─I─┤J
///     3├─┼─┼─┼─┤         │ │ │ │ │
///     4├─┼─┼─┼─┤        K├─L─M─N─┤O
///     5├─┼─┼─┼─┤         │ │ │ │ │
///     6├─┼─┼─┼─┤        P├─Q─R─S─┤T
///     7├─┼─┼─┼─┤         │ │ │ │ │
///     8└─┴─┴─┴─┘        U└─┴─┴─┴─┘Y
/// ```                      V W X
/// TODO: rename this to FragmentSpan Buffer
#[derive(Debug, Default)]
pub struct FragmentBuffer(BTreeMap<Cell, Vec<FragmentSpan>>);

impl Deref for FragmentBuffer {
    type Target = BTreeMap<Cell, Vec<FragmentSpan>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FragmentBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FragmentBuffer {
    pub fn new() -> Self {
        FragmentBuffer::default()
    }

    /// dump for debugging purpose only
    /// printling the fragments on this fragment buffer
    pub fn dump(&self) -> String {
        let mut buff = String::new();
        for (cell, shapes) in self.iter() {
            write!(buff, "\ncell: {} ", cell);
            for shape in shapes {
                write!(buff, "\n    {}", shape.fragment);
            }
        }
        buff
    }

    /// sort the fragments content in this cell
    fn sort_fragments_in_cell(&mut self, cell: Cell) {
        if let Some(fragments) = &mut self.get_mut(&cell) {
            (*fragments).sort();
        }
    }

    fn bounds(&self) -> Option<(Cell, Cell)> {
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

    pub fn get_size(&self, settings: &Settings) -> (f32, f32) {
        let (_top_left, bottom_right) =
            self.bounds().unwrap_or((Cell::new(0, 0), Cell::new(0, 0)));
        let w = settings.scale * (bottom_right.x + 2) as f32 * Cell::width();
        let h = settings.scale * (bottom_right.y + 2) as f32 * Cell::height();
        (w, h)
    }

    /// Note: Same fragment span can be stored in the same cell
    /// as it simplifies the algorithm for mergin marker line (lines with dots, and arrows)
    /// Since they will be attached to each other at the cell level
    fn add_fragment_span_to_cell(
        &mut self,
        cell: Cell,
        fragment_span: FragmentSpan,
    ) {
        if let Some(existing) = self.get_mut(&cell) {
            if !existing.contains(&fragment_span) {
                existing.push(fragment_span);
            } else {
                println!("already contain fragment span..");
            }
        } else {
            self.insert(cell, vec![fragment_span]);
        }
        self.sort_fragments_in_cell(cell);
    }

    /// Add a single fragment to this cell
    pub fn add_fragment_to_cell(
        &mut self,
        cell: Cell,
        ch: char,
        fragment: Fragment,
    ) {
        let fragment_span = FragmentSpan::new(Span::new(cell, ch), fragment);
        self.add_fragment_span_to_cell(cell, fragment_span);
    }

    /// add multiple fragments to cell
    pub fn add_fragments_to_cell(
        &mut self,
        cell: Cell,
        ch: char,
        fragments: Vec<Fragment>,
    ) {
        let fragment_spans = fragments
            .into_iter()
            .map(|fragment| FragmentSpan {
                span: Span::new(cell, ch),
                fragment,
            })
            .collect();
        if let Some(existing) = self.get_mut(&cell) {
            existing.extend(fragment_spans);
        } else {
            self.insert(cell, fragment_spans);
        }
        self.sort_fragments_in_cell(cell);
    }

    pub fn merge_fragment_spans(&self) -> Vec<FragmentSpan> {
        let fragment_spans = self.abs_fragment_spans();
        FragmentSpan::merge_recursive(fragment_spans)
    }

    /// Collect all the fragment span where all fragment spans of each cell
    /// are converted to their absolute position
    fn abs_fragment_spans(&self) -> Vec<FragmentSpan> {
        self.iter()
            .flat_map(|(cell, fragment_spans)| {
                fragment_spans
                    .iter()
                    .map(|frag_span| frag_span.absolute_position(*cell))
            })
            .collect()
    }
}
