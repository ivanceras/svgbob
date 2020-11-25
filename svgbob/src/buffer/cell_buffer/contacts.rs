use super::endorse;
use crate::buffer::{fragment::Fragment, Cell};
use std::{
    fmt,
};

/// Contains a group of fragments that are touching each other
/// The purpose of Contacts is to group fragments together
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
pub struct Contacts(pub Vec<Fragment>);

impl AsRef<Vec<Fragment>> for Contacts {
    fn as_ref(&self) -> &Vec<Fragment> {
        &self.0
    }
}

impl AsMut<Vec<Fragment>> for Contacts {
    fn as_mut(&mut self) -> &mut Vec<Fragment> {
        &mut self.0
    }
}

impl Contacts {
    pub(crate) fn new(fragment: Fragment) -> Self {
        Contacts(vec![fragment])
    }

    /// Check if any fragment can be group with any of the other fragment
    /// We use `.rev()` on this list of fragment since it has a high change of matching at the last
    /// added fragment of the next fragments to be checked.
    pub(crate) fn is_contacting_frag(&self, other_frag: &Fragment) -> bool {
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
        if let Some(rect) = endorse::endorse_rect(self.as_ref()) {
            Some(rect.into())
        } else if let Some(rounded_rect) = endorse::endorse_rounded_rect(self.as_ref()) {
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
}

impl fmt::Display for Contacts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for frag in self.as_ref().iter() {
            writeln!(f, "\t{}", frag)?;
        }
        Ok(())
    }
}
