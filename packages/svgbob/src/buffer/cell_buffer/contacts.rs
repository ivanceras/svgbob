use super::endorse;
use crate::buffer::fragment_buffer::FragmentSpan;
use crate::buffer::{fragment::Fragment, Cell};
use std::fmt;

/// Contains a group of fragments that are touching each other
/// The purpose of Contacts is to group fragments together
#[derive(Debug, Clone)]
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
        } else if let Some(rounded_rect) =
            endorse::endorse_rounded_rect(&fragments)
        {
            Some(rounded_rect.into())
        } else {
            None
        }
    }

    pub(crate) fn group_recursive(groups: Vec<Contacts>) -> Vec<Contacts> {
        let original_len = groups.len();
        let grouped = Self::second_pass_groupable(groups);
        // continue calling group recursive until the original len
        // has not decreased
        if grouped.len() < original_len {
            Self::group_recursive(grouped)
        } else {
            grouped
        }
    }

    fn second_pass_groupable(groups: Vec<Contacts>) -> Vec<Contacts> {
        let mut new_groups: Vec<Contacts> = vec![];
        for group in groups.into_iter() {
            let is_grouped = new_groups.iter_mut().any(|new_group| {
                if new_group.is_contacting(&group) {
                    new_group.as_mut().extend_from_slice(group.as_ref());
                    true
                } else {
                    false
                }
            });
            if !is_grouped {
                new_groups.push(group);
            }
        }
        new_groups
    }

    /// First phase of endorsing to shapes, in this case, rects and rounded_rects
    ///
    /// This function is calling on endorse methods that is applicable
    /// to fragments that are touching, to be promoted to a shape.
    /// These includes: rect, roundedrect,
    pub(crate) fn endorse_rects(
        groups: Vec<Contacts>,
    ) -> (Vec<Fragment>, Vec<Contacts>) {
        let mut fragments = vec![];
        let mut un_endorsed_rect: Vec<Contacts> = vec![];
        for group in groups {
            if let Some(fragment) = group.endorse_rect() {
                fragments.push(fragment);
            } else {
                un_endorsed_rect.push(group);
            }
        }
        (fragments, un_endorsed_rect)
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
