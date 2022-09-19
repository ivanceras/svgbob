use super::endorse;
use crate::buffer::fragment_buffer::FragmentSpan;
use crate::buffer::{fragment::Fragment, Cell};
use std::fmt;

/// Contains a group of fragments that are touching each other
/// The purpose of Contacts is to group fragments together
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
pub struct Contacts(pub Vec<FragmentSpan>);

impl AsRef<Vec<FragmentSpan>> for Contacts {
    fn as_ref(&self) -> &Vec<FragmentSpan> {
        &self.0
    }
}

impl AsMut<Vec<FragmentSpan>> for Contacts {
    fn as_mut(&mut self) -> &mut Vec<FragmentSpan> {
        &mut self.0
    }
}

impl Contacts {
    pub(crate) fn new(fragment: FragmentSpan) -> Self {
        Contacts(vec![fragment])
    }

    pub fn fragments(&self) -> Vec<&Fragment> {
        self.0.iter().map(|fs| &fs.fragment).collect()
    }

    pub fn cells(&self) -> Vec<Cell> {
        self.0.iter().flat_map(|fs| fs.cells.clone()).collect()
    }

    /// Check if any fragment can be group with any of the other fragment
    /// We use `.rev()` on this list of fragment since it has a high change of matching at the last
    /// added fragment of the next fragments to be checked.
    pub(crate) fn is_contacting_frag(&self, other_frag: &FragmentSpan) -> bool {
        self.as_ref()
            .iter()
            .rev()
            .any(|frag| frag.is_contacting(other_frag))
    }

    pub(crate) fn is_contacting(&self, other: &Self) -> bool {
        other
            .as_ref()
            .iter()
            .any(|other_frag| self.is_contacting_frag(other_frag))
    }

    /// Endorse if the fragments in this group
    /// can be:
    ///  - rect
    ///  - rounded_rect
    pub(crate) fn endorse_rects(&self) -> Option<Fragment> {
        let fragments = self.fragments();
        if let Some(rect) = endorse::endorse_rect(&fragments) {
            Some(rect.into())
        } else if let Some(rounded_rect) =
            endorse::endorse_rounded_rect(&fragments)
        {
            Some(rounded_rect.into())
        } else {
            None
        }
    }

    pub(crate) fn absolute_position(&self, cell: Cell) -> Self {
        Contacts(
            self.as_ref()
                .iter()
                .map(|frag| frag.absolute_position(cell))
                .collect(),
        )
    }

    pub fn is_bounded(&self, bound1: Cell, bound2: Cell) -> bool {
        self.cells()
            .iter()
            .all(|cell| cell.is_bounded(bound1, bound2))
    }

    pub fn hit_cell(&self, needle: Cell) -> bool {
        self.cells().iter().any(|cell| *cell == needle)
    }
}

impl fmt::Display for Contacts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for frag in self.as_ref().iter() {
            writeln!(f, "\t{}", frag)?;
        }
        Ok(())
    }
}
