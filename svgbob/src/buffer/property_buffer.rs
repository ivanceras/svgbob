use crate::{
    fragment,
    map::{ASCII_PROPERTIES, UNICODE_FRAGMENTS},
    Cell, Fragment, FragmentBuffer,
};
pub use property::{Property, Signal};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

mod property;

/// PropertyBuffer is a buffer
/// which contains the property of each cell
/// This will be used in the first phase of converting ascii diagrams into fragment buffer
/// The properties are generated once and will be repeatedly used for the second phase
/// where testing the neighboring charaters to determine the fragment to be drawn for that cell.
pub struct PropertyBuffer<'p>(HashMap<Cell, &'p Property>);

impl<'p> PropertyBuffer<'p> {
    pub fn new() -> Self {
        PropertyBuffer(HashMap::new())
    }

    /// get the appropriate character for this cell
    /// that matches the its behavior affected by the  8 surrounding properties
    /// of it's neighbor
    ///
    /// Note: this is primarily used for the drawing api, converting fragments
    /// into the most fitting ascii / unicode character
    pub fn match_char_from_cell(
        &self,
        cell: Cell,
        fragments: &Vec<Fragment>,
        try_unicode: bool,
    ) -> Option<char> {
        // try the unicode first
        let matched_unicode = if try_unicode { Fragment::match_unicode(fragments) } else { None };

        if let Some(matched_unicode) = matched_unicode {
            Some(matched_unicode)
        } else {
            let empty = &&Property::empty();
            let top_left = self.get(&cell.top_left()).unwrap_or(empty);
            let top = self.get(&cell.top()).unwrap_or(empty);
            let top_right = self.get(&cell.top_right()).unwrap_or(empty);
            let left = self.get(&cell.left()).unwrap_or(empty);
            let right = self.get(&cell.right()).unwrap_or(empty);
            let bottom_left = self.get(&cell.bottom_left()).unwrap_or(empty);
            let bottom = self.get(&cell.bottom()).unwrap_or(empty);
            let bottom_right = self.get(&cell.bottom_right()).unwrap_or(empty);

            Self::match_char_with_surrounding_properties(
                &fragments,
                top_left,
                top,
                top_right,
                left,
                right,
                bottom_left,
                bottom,
                bottom_right,
            )
        }
    }

    /// if the fragments match to the return fragments
    /// of the property behavior, then it is a match
    pub fn match_char_with_surrounding_properties(
        fragments: &Vec<Fragment>,
        top_left: &Property,
        top: &Property,
        top_right: &Property,
        left: &Property,
        right: &Property,
        bottom_left: &Property,
        bottom: &Property,
        bottom_right: &Property,
    ) -> Option<char> {
        let signature_match = ASCII_PROPERTIES.iter().find_map(|(ch, property)| {
            if property.match_signature(fragments) {
                Some(*ch)
            } else {
                None
            }
        });
        // if no match in signature, find it in behavior match
        if signature_match.is_some() {
            signature_match
        } else {
            ASCII_PROPERTIES.iter().find_map(|(ch, property)| {
                let mut behavioral_fragments = property.fragments(
                    top_left,
                    top,
                    top_right,
                    left,
                    right,
                    bottom_left,
                    bottom,
                    bottom_right,
                );
                let mut merged_behavioral_fragments =
                    Fragment::merge_recursive(behavioral_fragments);
                merged_behavioral_fragments.sort();
                merged_behavioral_fragments.dedup();
                assert!(merged_behavioral_fragments.is_sorted());

                assert!(fragments.is_sorted());
                if merged_behavioral_fragments == *fragments {
                    Some(*ch)
                } else {
                    None
                }
            })
        }
    }
}

impl<'p> Deref for PropertyBuffer<'p> {
    type Target = HashMap<Cell, &'p Property>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'p> DerefMut for PropertyBuffer<'p> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// convert property buffer to fragment buffer
impl<'p> Into<FragmentBuffer> for &PropertyBuffer<'p> {
    fn into(self) -> FragmentBuffer {
        let mut fb = FragmentBuffer::new();
        for (cell, property) in self.deref() {
            let empty = &&Property::empty();
            let top_left = self.get(&cell.top_left()).unwrap_or(empty);
            let top = self.get(&cell.top()).unwrap_or(empty);
            let top_right = self.get(&cell.top_right()).unwrap_or(empty);
            let left = self.get(&cell.left()).unwrap_or(empty);
            let right = self.get(&cell.right()).unwrap_or(empty);
            let bottom_left = self.get(&cell.bottom_left()).unwrap_or(empty);
            let bottom = self.get(&cell.bottom()).unwrap_or(empty);
            let bottom_right = self.get(&cell.bottom_right()).unwrap_or(empty);
            let cell_fragments = property.fragments(
                top_left,
                top,
                top_right,
                left,
                right,
                bottom_left,
                bottom,
                bottom_right,
            );
            if !cell_fragments.is_empty() {
                fb.add_fragments_to_cell(*cell, cell_fragments);
            } else {
                //If no match make it a text fragment
                if let Some(fragments) = UNICODE_FRAGMENTS.get(&property.ch) {
                    let merged_fragments = Fragment::merge_recursive(fragments.clone());
                    fb.add_fragments_to_cell(*cell, merged_fragments);
                } else {
                    fb.add_fragment_to_cell(*cell, fragment::cell_text(property.ch));
                }
            }
        }
        fb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::{fragment_buffer::fragment::*, Cell, CellGrid};

    #[test]
    fn test_match_char_with_surrounding_properties() {
        let o = CellGrid::o();
        let q = CellGrid::q();
        let u = CellGrid::u();
        let unit4 = Cell::unit(4); // 1.0
                                   //      .-
                                   //     /
                                   // Expecting to match .
        let fragments = vec![arc(o, q, unit4), line(q, u)];
        let top_left = &Property::empty();
        let top = &Property::empty();
        let top_right = &Property::empty();
        let left = &Property::empty();
        let right = Property::from_char('-').expect("must have a property");
        let bottom_left = Property::from_char('/').unwrap();
        let bottom = &Property::empty();
        let bottom_right = &Property::empty();
        let ch = PropertyBuffer::match_char_with_surrounding_properties(
            &fragments,
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        );
        println!("ch: {:?}", ch);
        assert_eq!(ch, Some(','));
    }

    #[test]
    fn test_match_char_with_surrounding_properties_with_diagonal_cross() {
        let k = CellGrid::k();
        let m = CellGrid::m();
        let o = CellGrid::o();
        let e = CellGrid::e();
        let u = CellGrid::u();
        //       /
        //     -+-
        //     /

        let mut fragments = vec![line(k, o), line(e, u)];
        fragments.sort();
        let top_left = &Property::empty();
        let top = &Property::empty();
        let top_right = &Property::from_char('/').unwrap();
        let left = &Property::from_char('-').unwrap();
        let right = Property::from_char('-').expect("must have a property");
        let bottom_left = &Property::from_char('/').unwrap();
        let bottom = &Property::empty();
        let bottom_right = &Property::empty();
        let ch = PropertyBuffer::match_char_with_surrounding_properties(
            &fragments,
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        );

        println!("ch: {:?}", ch);
        assert_eq!(ch, Some('+'));
    }

    #[test]
    fn test_match_char_with_surrounding_properties_with_cross() {
        let k = CellGrid::k();
        let o = CellGrid::o();
        let c = CellGrid::c();
        let m = CellGrid::m();
        let w = CellGrid::w();
        //      |
        //     -+-
        //      |

        let mut fragments = vec![line(k, o), line(c, w)];
        fragments.sort();
        let top_left = &Property::empty();
        let top = &Property::from_char('|').unwrap();
        let top_right = &Property::empty();
        let left = &Property::from_char('-').unwrap();
        let right = Property::from_char('-').expect("must have a property");
        let bottom_left = &Property::empty();
        let bottom = &Property::from_char('|').unwrap();
        let bottom_right = &Property::empty();
        let ch = PropertyBuffer::match_char_with_surrounding_properties(
            &fragments,
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        );
        println!("ch: {:?}", ch);
        assert_eq!(ch, Some('+'));
    }

    #[test]
    fn test_match_char_with_surrounding_properties_with_underscores() {
        let u = CellGrid::u();
        let y = CellGrid::y();
        //     ___
        //     ___

        let mut fragments = vec![line(u, y)];
        fragments.sort();
        let top_left = &Property::from_char('_').unwrap();
        let top = &Property::from_char('_').unwrap();
        let top_right = &Property::from_char('_').unwrap();
        let left = &Property::from_char('_').unwrap();
        let right = Property::from_char('_').expect("must have a property");
        let bottom_left = &Property::empty();
        let bottom = &Property::empty();
        let bottom_right = &Property::empty();
        let ch = PropertyBuffer::match_char_with_surrounding_properties(
            &fragments,
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        );
        println!("ch: {:?}", ch);
        assert_eq!(ch, Some('_'));
    }

    #[test]
    fn test_match_char_with_surrounding_properties_with_2_vertically_aligned_underscore() {
        let u = CellGrid::u();
        let y = CellGrid::y();
        //     _
        //     _

        let mut fragments = vec![line(u, y)];
        fragments.sort();
        let top_left = &Property::empty();
        let top = &Property::from_char('_').unwrap();
        let top_right = &Property::empty();
        let left = &Property::empty();
        let right = &Property::empty();
        let bottom_left = &Property::empty();
        let bottom = &Property::empty();
        let bottom_right = &Property::empty();
        let ch = PropertyBuffer::match_char_with_surrounding_properties(
            &fragments,
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        );
        println!("ch: {:?}", ch);
        assert_eq!(ch, Some('_'));
    }
}
