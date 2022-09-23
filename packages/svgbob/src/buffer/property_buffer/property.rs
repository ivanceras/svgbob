#![allow(clippy::type_complexity)]
use self::Signal::Strong;
use crate::{
    map::{ASCII_PROPERTIES, UNICODE_PROPERTIES},
    Fragment, Point,
};
use std::{cmp, fmt, sync::Arc};

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

/// At least sum = 6
/// Medium + Medium connects (3 + 3)
/// Strong + Weak connects ( 4 + 2 )
#[derive(PartialEq, Eq, Clone)]
pub enum Signal {
    Faint,
    Weak,
    Medium,
    Strong,
}

impl Signal {
    fn intensity(&self) -> u8 {
        match self {
            Signal::Faint => 1,
            Signal::Weak => 2,
            Signal::Medium => 3,
            Signal::Strong => 4,
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.intensity().cmp(&other.intensity()))
    }
}

#[derive(Clone)]
pub struct Property {
    pub ch: char,
    /// the signal signature and the corresponding fragment with that signal
    /// This is used in the first pass of checking the surrounding characters for properties
    /// if it meets the required condition of the character in spot
    signature: Vec<(Signal, Vec<Fragment>)>,
    /// behavior is the final output of fragments of the spot character
    /// depending on flag that is meet when checked agains the surrounding characters
    pub behavior: Arc<
        dyn Fn(
                &Property,
                &Property,
                &Property,
                &Property,
                &Property,
                &Property,
                &Property,
                &Property,
            ) -> Vec<(bool, Vec<Fragment>)>
            + Sync
            + Send,
    >,
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{{ char: {}, has {} signature }}",
            self.ch,
            self.signature.len()
        )
    }
}

impl Property {
    pub fn new(
        ch: char,
        signature: Vec<(Signal, Vec<Fragment>)>,
        behavior: Arc<
            dyn Fn(
                    &Property,
                    &Property,
                    &Property,
                    &Property,
                    &Property,
                    &Property,
                    &Property,
                    &Property,
                ) -> Vec<(bool, Vec<Fragment>)>
                + Sync
                + Send,
        >,
    ) -> Self {
        Property {
            ch,
            signature,
            behavior,
        }
    }

    /// get the matching property of this char
    /// start from the ascii_map lookup
    /// then to the unicode_map lookup when it can't find from the first map.
    pub(crate) fn from_char<'a>(ch: char) -> Option<&'a Property> {
        match ASCII_PROPERTIES.get(&ch) {
            Some(property) => Some(property),
            None => UNICODE_PROPERTIES.get(&ch),
        }
    }

    /// empty property serves as a substitue for None property for simplicity in
    /// the behavior code, never have to deal with Option
    pub fn empty() -> Self {
        Property {
            ch: ' ',
            signature: vec![],
            behavior: Arc::new(|_, _, _, _, _, _, _, _| vec![]),
        }
    }

    /// derive a strong property with a strong signal
    pub fn with_strong_fragments(ch: char, fragments: Vec<Fragment>) -> Self {
        Property {
            ch,
            signature: vec![(Signal::Strong, fragments)],
            //TODO find a way to move the fragments here
            behavior: Arc::new(|_, _, _, _, _, _, _, _| vec![(true, vec![])]),
        }
    }

    fn signature_fragments_with_signal(&self, signal: Signal) -> Vec<Fragment> {
        let mut fragments: Vec<Fragment> = self
            .signature
            .iter()
            .filter_map(|(sig, fragments)| {
                if *sig == signal {
                    Some(fragments)
                } else {
                    None
                }
            })
            .flatten()
            .map(Clone::clone)
            .collect();
        fragments.sort();
        fragments.dedup();
        fragments
    }

    /// Check if the property is exactly this character
    /// returns true if this property is derive from character `ch`
    pub(crate) fn is(&self, ch: char) -> bool {
        self.ch == ch
    }

    pub(crate) fn is_alphabet(&self) -> bool {
        self.ch.is_alphabetic() && self.ch != '_' // since space is used when a property is derived from strong
    }

    pub fn match_signature(&self, fragments: &[Fragment]) -> bool {
        let signature_fragments = self.signature_fragments_with_signal(Strong);
        signature_fragments == *fragments
    }

    /// evaluate this property together with the supplied surrounding
    /// to see if the resulting fragments is equal to the supplied fragments
    pub(crate) fn match_property(&self, _fragments: &[Fragment]) -> bool {
        false
    }

    /// Check to see if this spot can overal the line a b with at least Medium signal
    pub(crate) fn line_overlap(&self, a: Point, b: Point) -> bool {
        self.line_overlap_with_signal(a, b, Signal::Medium)
    }

    pub(crate) fn line_strongly_overlap(&self, a: Point, b: Point) -> bool {
        self.line_overlap_with_signal(a, b, Signal::Strong)
    }

    pub(crate) fn line_weakly_overlap(&self, a: Point, b: Point) -> bool {
        self.line_overlap_with_signal(a, b, Signal::Weak)
    }

    /// Check to see if this spot has an endpoint to p
    pub(crate) fn has_endpoint(&self, p: Point) -> bool {
        self.signature.iter().any(|(_signal, signature)| {
            signature.iter().any(|fragment| fragment.has_endpoint(p))
        })
    }

    /// Check to see if any fragment that is generated in this character
    /// can overlap (completely covered) line a b
    fn line_overlap_with_signal(
        &self,
        a: Point,
        b: Point,
        required_signal: Signal,
    ) -> bool {
        self.signature
            .iter()
            .filter(|(signal, _signature)| *signal >= required_signal)
            .any(|(_signal, signature)| {
                signature.iter().any(|fragment| fragment.line_overlap(a, b))
            })
    }

    /// Check to see if any fragment that is generated in this character
    /// can arc from a to b regardless of the radius
    pub(crate) fn arcs_to(&self, a: Point, b: Point) -> bool {
        self.signature.iter().any(|(_signal, signature)| {
            signature.iter().any(|fragment| fragment.arcs_to(a, b))
        })
    }

    /// the fragments of this property when the surrounding properties is supplied
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn fragments(
        &self,
        top_left: &Property,
        top: &Property,
        top_right: &Property,
        left: &Property,
        right: &Property,
        bottom_left: &Property,
        bottom: &Property,
        bottom_right: &Property,
    ) -> Vec<Fragment> {
        let bool_fragments = self.behavior.as_ref()(
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        );
        let cell_fragments: Vec<Fragment> = bool_fragments.into_iter().fold(
            vec![],
            |mut acc, (passed, fragments)| {
                if passed {
                    acc.extend(fragments);
                };
                acc
            },
        );
        cell_fragments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::CellGrid;

    #[test]
    fn test_overlap() {
        let _a = CellGrid::a();
        let _b = CellGrid::b();
        let c = CellGrid::c();
        let _d = CellGrid::d();
        let _e = CellGrid::e();
        let _f = CellGrid::f();
        let _g = CellGrid::g();
        let _h = CellGrid::h();
        let _i = CellGrid::i();
        let _j = CellGrid::j();
        let k = CellGrid::k();
        let _l = CellGrid::l();
        let m = CellGrid::m();
        let _n = CellGrid::n();
        let o = CellGrid::o();
        let _p = CellGrid::p();
        let _q = CellGrid::q();
        let _r = CellGrid::r();
        let _s = CellGrid::s();
        let _t = CellGrid::t();
        let _u = CellGrid::u();
        let _v = CellGrid::v();
        let w = CellGrid::w();
        let _x = CellGrid::x();
        let _y = CellGrid::y();

        let dash = Property::from_char('-').expect("should have 1");
        assert!(dash.line_overlap(k, o));
        assert!(!dash.line_overlap(c, w));

        let vert = Property::from_char('|').expect("should have 1");
        assert!(!vert.line_overlap(k, o));
        assert!(vert.line_overlap(c, w));

        let plus = Property::from_char('+').expect("should have 1");
        assert!(plus.line_overlap(k, o));
        assert!(plus.line_overlap(c, w));
        assert!(plus.line_overlap(m, o));
        assert!(plus.line_overlap(m, w));
        assert!(plus.line_overlap(c, m));
        assert!(plus.line_overlap(k, m));
    }
}
