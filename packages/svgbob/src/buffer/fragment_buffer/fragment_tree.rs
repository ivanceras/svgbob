use crate::buffer::fragment_buffer::FragmentSpan;
use crate::Fragment;
use sauron::{html::attributes::*, Node};

/// A tree of fragments where a fragment can contain other fragments
/// when those fragments are inside in this fragment
/// The main purpose of this struct is for tagging fragments
/// such as rect and circles to have a CellText fragment inside that are special
/// text commands such as css classes, which the user can style the containing fragment
#[derive(Debug, Clone)]
pub struct FragmentTree {
    fragment: FragmentSpan,
    css_tag: Vec<String>,
    enclosing: Vec<FragmentTree>,
}

impl FragmentTree {
    pub(crate) fn new(fragment: FragmentSpan) -> Self {
        FragmentTree {
            fragment,
            css_tag: vec![],
            enclosing: vec![],
        }
    }

    fn can_fit(&self, other: &Self) -> bool {
        self.fragment.fragment.can_fit(&other.fragment.fragment)
    }

    /// check if this fragment can fit to this fragment tree.
    /// this also check if any of the children of this tree can fit
    /// the fragment
    fn enclose(&mut self, other: &Self) -> bool {
        if self.can_fit(other) {
            self.enclosing.push(other.clone());
            true
        } else {
            for child in &mut self.enclosing {
                if child.enclose(other) {
                    return true;
                }
            }
            false
        }
    }

    /// Try to put the other fragment somwhere in the tree, but traversing the depth first.
    /// This is needed for accurately tagging which shapes by putting the right cell_text into
    /// it's direct parent instead of just checking whether the text is bounded by some shapes.
    fn enclose_deep_first(&mut self, other: &Self) -> bool {
        for child in &mut self.enclosing {
            if child.enclose_deep_first(other) {
                return true;
            }
        }
        if self.can_fit(other) {
            let css_tags = other.fragment.fragment.as_css_tag();
            if !css_tags.is_empty() {
                self.css_tag.extend(css_tags);
            } else {
                self.enclosing.push(other.clone());
            }
            true
        } else {
            false
        }
    }

    pub(crate) fn enclose_fragments(fragments: Vec<FragmentSpan>) -> Vec<Self> {
        let fragment_trees: Vec<Self> = fragments
            .into_iter()
            .map(FragmentTree::new)
            .collect();
        Self::enclose_recursive(fragment_trees)
    }

    pub(crate) fn enclose_recursive(fragment_trees: Vec<Self>) -> Vec<Self> {
        let original_len = fragment_trees.len();
        let merged = Self::second_pass_enclose(fragment_trees);
        if merged.len() < original_len {
            Self::enclose_recursive(merged)
        } else {
            merged
        }
    }

    /// make all the fragments a fragment tree and try to fit each other
    fn second_pass_enclose(fragment_trees: Vec<Self>) -> Vec<Self> {
        let mut new_trees: Vec<Self> = vec![];
        for frag_tree in fragment_trees {
            let is_enclosed = new_trees
                .iter_mut()
                .rev()
                .any(|new_tree| new_tree.enclose_deep_first(&frag_tree));
            if !is_enclosed {
                new_trees.push(frag_tree);
            }
        }
        new_trees
    }

    /// convert back into fragments
    fn into_nodes<MSG>(self) -> Vec<Node<MSG>> {
        let mut nodes = vec![];
        let mut fragment_node: Node<MSG> = self.fragment.fragment.into();
        let _css_tag_len = self.css_tag.len();
        fragment_node =
            fragment_node.merge_attributes(vec![classes(self.css_tag)]);

        nodes.push(fragment_node);
        for child in self.enclosing {
            nodes.extend(child.into_nodes())
        }
        nodes
    }

    /// convert fragments to node, where cell_text and text may become
    /// css class of the contain fragment
    pub(crate) fn fragments_to_node<MSG>(
        fragments: Vec<FragmentSpan>,
    ) -> Vec<Node<MSG>> {
        let fragment_trees: Vec<FragmentTree> =
            Self::enclose_fragments(fragments);
        fragment_trees
            .into_iter()
            .flat_map(|frag_tree| frag_tree.into_nodes())
            .collect()
    }
}

//TODO: fix all of this tests
/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buffer::Cell,
        fragment::{rect, CellText},
        Point,
    };

    #[test]
    fn test_enclose() {
        let mut rect1 = FragmentTree::new(rect(
            Point::new(0.0, 0.0),
            Point::new(10.0, 10.0),
            false,
            false,
        ));
        let rect2 = FragmentTree::new(rect(
            Point::new(1.0, 1.0),
            Point::new(9.0, 9.0),
            false,
            false,
        ));
        let text1 = FragmentTree::new(Fragment::CellText(CellText::new(
            Cell::new(2, 2),
            "{doc}".to_string(),
        )));
        let text2 = FragmentTree::new(Fragment::CellText(CellText::new(
            Cell::new(2, 2),
            "This is a hello world!".to_string(),
        )));

        assert!(rect1.enclose(&rect2));
        assert!(rect1.enclose(&text1));
        assert!(!rect1.enclose(&text2));
        dbg!(rect1);
    }

    #[test]
    fn test_enclose_recursive() {
        let rect1 =
            rect(Point::new(0.0, 0.0), Point::new(10.0, 10.0), false, false);
        let rect2 =
            rect(Point::new(1.0, 1.0), Point::new(9.0, 9.0), false, false);
        let text1 = Fragment::CellText(CellText::new(
            Cell::new(2, 2),
            "{doc}".to_string(),
        ));
        let text2 = Fragment::CellText(CellText::new(
            Cell::new(2, 2),
            "This is a hello world!".to_string(),
        ));

        let fragments = vec![rect1, rect2, text1, text2];
        let fragment_trees = FragmentTree::enclose_fragments(fragments);
        dbg!(&fragment_trees);

        assert_eq!(
            fragment_trees,
            vec![
                FragmentTree {
                    fragment: rect(
                        Point::new(0.0, 0.0),
                        Point::new(10.0, 10.0),
                        false,
                        false
                    ),
                    css_tag: vec![],
                    enclosing: vec![FragmentTree {
                        fragment: rect(
                            Point::new(1.0, 1.0),
                            Point::new(9.0, 9.0),
                            false,
                            false
                        ),
                        css_tag: vec!["doc".to_string()],
                        enclosing: vec![],
                    },],
                },
                FragmentTree {
                    fragment: Fragment::CellText(CellText::new(
                        Cell::new(2, 2),
                        "This is a hello world!".to_string(),
                    )),
                    css_tag: vec![],
                    enclosing: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_enclose_recursive_different_order() {
        let rect1 =
            rect(Point::new(0.0, 0.0), Point::new(10.0, 10.0), false, false);
        let rect2 =
            rect(Point::new(1.0, 1.0), Point::new(9.0, 9.0), false, false);
        let text1 = Fragment::CellText(CellText::new(
            Cell::new(2, 2),
            "{doc}".to_string(),
        ));
        let text2 = Fragment::CellText(CellText::new(
            Cell::new(2, 2),
            "This is a hello world!".to_string(),
        ));

        let fragments = vec![rect1, rect2, text1, text2];
        let fragment_trees = FragmentTree::enclose_fragments(fragments);
        dbg!(&fragment_trees);

        assert_eq!(
            fragment_trees,
            vec![
                FragmentTree {
                    fragment: rect(
                        Point::new(0.0, 0.0),
                        Point::new(10.0, 10.0),
                        false,
                        false
                    ),
                    css_tag: vec![],
                    enclosing: vec![FragmentTree {
                        fragment: rect(
                            Point::new(1.0, 1.0),
                            Point::new(9.0, 9.0),
                            false,
                            false
                        ),
                        css_tag: vec!["doc".to_string()],
                        enclosing: vec![],
                    },],
                },
                FragmentTree {
                    fragment: Fragment::CellText(CellText::new(
                        Cell::new(2, 2),
                        "This is a hello world!".to_string(),
                    )),
                    css_tag: vec![],
                    enclosing: vec![],
                },
            ]
        );
    }
}
*/
