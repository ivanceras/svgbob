use crate::{buffer::Settings, Cell};
pub use fragment::Fragment;
pub use fragment_tree::FragmentTree;
use itertools::Itertools;
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

pub mod fragment;
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
#[derive(Debug)]
pub struct FragmentBuffer(BTreeMap<Cell, Vec<Fragment>>);

impl Deref for FragmentBuffer {
    type Target = BTreeMap<Cell, Vec<Fragment>>;

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
        FragmentBuffer(BTreeMap::new())
    }

    /// dump for debugging purpose only
    /// printling the fragments on this fragment buffer
    pub fn dump(&self) -> String {
        let mut buff = String::new();
        for (cell, shapes) in self.iter() {
            buff.push_str(&format!("\ncell: {} ", cell));
            for shape in shapes {
                buff.push_str(&format!("\n    {}", shape));
            }
        }
        buff
    }

    /// Add a single fragment to this cell
    pub fn add_fragment_to_cell(&mut self, cell: Cell, fragment: Fragment) {
        if let Some(existing) = self.get_mut(&cell) {
            existing.push(fragment);
        } else {
            self.insert(cell, vec![fragment]);
        }
        self.sort_fragments_in_cell(cell);
    }

    /// sort the fragments content in this cell
    fn sort_fragments_in_cell(&mut self, cell: Cell) {
        if let Some(fragments) = &mut self.get_mut(&cell) {
            (*fragments).sort();
        }
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

    pub(crate) fn get_size(&self, settings: &Settings) -> (f32, f32) {
        let (top_left, bottom_right) = self.bounds().unwrap_or((Cell::new(0, 0), Cell::new(0, 0)));
        let w = settings.scale * (bottom_right.x + 2) as f32 * Cell::width();
        let h = settings.scale * (bottom_right.y + 2) as f32 * Cell::height();
        (w, h)
    }

    /// add multiple fragments to cell
    pub fn add_fragments_to_cell(&mut self, cell: Cell, fragments: Vec<Fragment>) {
        if let Some(existing) = self.get_mut(&cell) {
            existing.extend(fragments);
        } else {
            self.insert(cell, fragments);
        }
        self.sort_fragments_in_cell(cell);
    }

    pub(crate) fn merge_fragments(&self) -> Vec<Fragment> {
        let fragments = self.first_pass_merge();
        Self::merge_recursive(fragments)
    }

    /// merge fragments that can be merged.
    /// This is only merging the fragments that are in the same
    /// cell
    fn first_pass_merge(&self) -> Vec<Fragment> {
        let mut merged: Vec<Fragment> = vec![];
        for (cell, fragments) in self.iter() {
            for frag in fragments.iter() {
                //Note: The fragments are calculated with their absolute
                // parameters and is derived from the cell position
                let abs_frag = frag.absolute_position(*cell);
                let had_merged = merged.iter_mut().rev().any(|mfrag| {
                    if mfrag.can_merge(&abs_frag) {
                        if let Some(new_merge) = mfrag.merge(&abs_frag) {
                            *mfrag = new_merge;
                        } else {
                            panic!("Should merged");
                        }
                        true
                    } else {
                        false
                    }
                });
                if !had_merged {
                    merged.push(abs_frag);
                }
            }
        }
        merged
    }

    fn merge_recursive(fragments: Vec<Fragment>) -> Vec<Fragment> {
        let original_len = fragments.len();
        let merged = Self::second_pass_merge(fragments);
        if merged.len() < original_len {
            Self::merge_recursive(merged)
        } else {
            merged
        }
    }

    fn second_pass_merge(fragments: Vec<Fragment>) -> Vec<Fragment> {
        let mut new_fragments: Vec<Fragment> = vec![];
        for fragment in fragments.into_iter() {
            let is_merged = new_fragments.iter_mut().rev().any(|new_frag| {
                if new_frag.can_merge(&fragment) {
                    *new_frag = new_frag.merge(&fragment).expect("should merge");
                    true
                } else {
                    false
                }
            });
            if !is_merged {
                new_fragments.push(fragment);
            }
        }
        new_fragments
    }
}
