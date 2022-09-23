use super::endorse;
use super::endorse::Endorse;
use crate::buffer::fragment_buffer::FragmentSpan;
use crate::buffer::Span;
use crate::buffer::{fragment::Fragment, Cell};
use crate::Merge;
use std::fmt;

/// Contains a group of fragments that are touching each other
/// The purpose of Contacts is to group fragments together
#[derive(Debug, Clone, PartialEq, Eq)]
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
        self.0.iter().flat_map(|fs| fs.cells()).collect()
    }

    pub fn span(&self) -> Span {
        let cell_chars: Vec<(Cell, char)> =
            self.0.iter().flat_map(|fs| fs.span.0.clone()).collect();
        cell_chars.into()
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

    ///TODO: return the FragmentSpan
    /// Endorse if the fragments in this group
    /// can be:
    ///  - rect
    ///  - rounded_rect
    pub(crate) fn endorse_rect(&self) -> Option<Fragment> {
        let fragments = self.fragments();
        if let Some(rect) = endorse::endorse_rect(&fragments) {
            Some(rect.into())
        } else {
            endorse::endorse_rounded_rect(&fragments)
                .map(|rounded_rect| rounded_rect.into())
        }
    }

    /// First phase of endorsing to shapes, in this case, rects and rounded_rects
    ///
    /// This function is calling on endorse methods that is applicable
    /// to fragments that are touching, to be promoted to a shape.
    /// These includes: rect, roundedrect,
    pub(crate) fn endorse_rects(
        groups: Vec<Contacts>,
    ) -> Endorse<FragmentSpan, Contacts> {
        let mut accepted = vec![];
        let mut rejects: Vec<Contacts> = vec![];
        for group in groups {
            if let Some(fragment) = group.endorse_rect() {
                //TODO: use the fragment span at group.0
                let span = group.span();
                let fragment_span = FragmentSpan::new(span, fragment);
                accepted.push(fragment_span);
            } else {
                rejects.push(group);
            }
        }
        Endorse { accepted, rejects }
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

impl Merge for Contacts {
    fn merge(&self, other: &Self) -> Option<Self> {
        if self.is_contacting(&other) {
            let mut fragment_spans: Vec<FragmentSpan> = self.0.clone();
            fragment_spans.extend_from_slice(&other.0);
            Some(Contacts(fragment_spans))
        } else {
            None
        }
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
