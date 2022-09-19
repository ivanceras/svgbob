use crate::Cell;
use crate::Fragment;
use crate::Settings;

pub struct FragmentSpan {
    cells: Vec<Cell>,
    fragment: Fragment,
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
}
