pub trait Merge {
    /// An implementation for each implementing objects
    /// which creates a new instance merging `self` and the `other` item.
    fn merge(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    /// Merge all items until the size don't change
    fn merge_recursive(items: impl IntoIterator<Item = Self>) -> Vec<Self>
    where
        Self: Sized,
    {
        let items: Vec<Self> = items.into_iter().collect();
        let original_len = items.len();
        let merged = Self::second_pass_merge(items);
        if merged.len() < original_len {
            Self::merge_recursive(merged)
        } else {
            merged
        }
    }

    /// Iterate through each items in the group and merge that items
    /// that can be merged
    fn second_pass_merge(items: impl IntoIterator<Item = Self>) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut new_groups: Vec<Self> = vec![];
        for item in items.into_iter() {
            if !new_groups.iter_mut().rev().any(|new_group| {
                if let Some(new_merged) = new_group.merge(&item) {
                    *new_group = new_merged;
                    true
                } else {
                    false
                }
            }) {
                new_groups.push(item)
            }
        }
        new_groups
    }
}
