use crate::Cell;
use crate::Fragment;
use crate::Settings;
use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
pub struct FragmentSpan {
    pub cells: Vec<Cell>,
    pub fragment: Fragment,
}

impl fmt::Display for FragmentSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cell in &self.cells {
            write!(f, "{}: ", cell);
        }
        writeln!(f, "{}", self.fragment)
    }
}

impl FragmentSpan {
    pub fn new(cell: Cell, fragment: Fragment) -> Self {
        Self {
            cells: vec![cell],
            fragment,
        }
    }

    pub fn merge(&self, other: &Self, settings: &Settings) -> Option<Self> {
        if let Some(new_merge) = self.fragment.merge(&other.fragment, settings)
        {
            Some(Self {
                cells: [self.cells.clone(), other.cells.clone()].concat(),
                fragment: new_merge,
            })
        } else {
            None
        }
    }

    pub(crate) fn merge_recursive(
        fragment_spans: Vec<Self>,
        settings: &Settings,
    ) -> Vec<Self> {
        let original_len = fragment_spans.len();
        let merged = Self::second_pass_merge(fragment_spans, settings);
        if merged.len() < original_len {
            Self::merge_recursive(merged, settings)
        } else {
            merged
        }
    }

    fn second_pass_merge(
        fragment_spans: Vec<Self>,
        settings: &Settings,
    ) -> Vec<Self> {
        let mut new_groups: Vec<Self> = vec![];
        for fragment_span in fragment_spans.into_iter() {
            let is_merged = new_groups.iter_mut().rev().any(|new_group| {
                if let Some(new_merged) =
                    new_group.merge(&fragment_span, settings)
                {
                    *new_group = new_merged;
                    true
                } else {
                    false
                }
            });
            if !is_merged {
                new_groups.push(fragment_span);
            }
        }
        new_groups
    }

    pub(crate) fn is_contacting(&self, other: &Self) -> bool {
        self.fragment.is_contacting(&other.fragment)
    }

    pub fn absolute_position(&self, cell: Cell) -> Self {
        Self {
            cells: self.cells.clone(),
            fragment: self.fragment.absolute_position(cell),
        }
    }
}
