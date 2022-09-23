use crate::buffer::fragment_buffer::FragmentSpan;
use crate::{
    buffer::{
        cell_buffer::Contacts, FragmentBuffer, Property, PropertyBuffer,
        StringBuffer,
    },
    fragment,
    map::{circle_map, UNICODE_FRAGMENTS},
    Cell, Fragment, Merge, Settings,
};
use itertools::Itertools;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

/// A describes where a char came from relative to the source ascii text
/// The primary purpose of span is to group adjacent cell together
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span(pub Vec<(Cell, char)>);

impl Deref for Span {
    type Target = Vec<(Cell, char)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Bounds {
    top_left: Cell,
    bottom_right: Cell,
}

impl DerefMut for Span {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<(Cell, char)>> for Span {
    fn from(cell_chars: Vec<(Cell, char)>) -> Self {
        Span(cell_chars)
    }
}

impl Span {
    pub(crate) fn new(cell: Cell, ch: char) -> Self {
        Span(vec![(cell, ch)])
    }

    pub(super) fn is_adjacent(&self, cell: &Cell) -> bool {
        self.iter()
            .rev()
            .any(|(ex_cell, _)| ex_cell.is_adjacent(cell))
    }

    /// if any cell of this span is adjacent to any cell of the other
    /// Use .rev() to check the last cell of this Span agains the first cell of the other Span
    /// They have a high change of matching faster
    pub(super) fn can_merge(&self, other: &Self) -> bool {
        self.iter().rev().any(|(cell, _)| {
            other
                .iter()
                .any(|(other_cell, _)| cell.is_adjacent(other_cell))
        })
    }

    /*
    pub(crate) fn merge(&self, other: &Self) -> Option<Self> {
        if self.can_merge(other) {
            let mut cells = self.0.clone();
            cells.extend(&other.0);
            Some(Span(cells))
        } else {
            None
        }
    }
    */

    /// paste the other Span at cell location `loc`
    pub fn paste_at(&self, loc: Cell, other: &Self) -> Self {
        let mut this = self.clone();
        for (cell, ch) in other.deref() {
            this.push((*cell + loc, *ch));
        }
        this.sort();
        this.dedup();
        this
    }

    /// returns the top_left most cell which aligns the top most and the left most cell.
    pub(crate) fn bounds(&self) -> Option<(Cell, Cell)> {
        if let Some((min_y, max_y)) =
            self.iter().map(|(cell, _)| cell.y).minmax().into_option()
        {
            if let Some((min_x, max_x)) =
                self.iter().map(|(cell, _)| cell.x).minmax().into_option()
            {
                Some((Cell::new(min_x, min_y), Cell::new(max_x, max_y)))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn cell_bounds(&self) -> Option<Bounds> {
        if let Some((top_left, top_right)) = self.bounds() {
            Some(Bounds::new(top_left, top_right))
        } else {
            None
        }
    }

    /// shift the cells relative to the top_left most bound
    pub(crate) fn localize(self) -> Self {
        if let Some((tl, _br)) = self.bounds() {
            let mut new_self = Span(vec![]);
            for (cell, ch) in self.iter() {
                let local_cell = tl.localize_cell(*cell);
                new_self.push((local_cell, *ch));
            }
            new_self
        } else {
            self
        }
    }

    /// Grouping cell by adjacents are not enough
    ///
    /// grouping them together when they are actually connected
    /// is the most approprivate way of grouping
    /// Span just provides an optimization of the number
    /// of elements to be checked.
    /// Only elements on the same span are checked to see if they
    /// belong on the same group
    ///
    pub(crate) fn get_contacts(&self) -> Vec<Contacts> {
        let fb: FragmentBuffer = self.clone().into_fragment_buffer();

        let mut groups: Vec<Contacts> = vec![];
        let merged_fragments = fb.merge_fragment_spans();
        for fragment in merged_fragments.into_iter() {
            let belongs_to_group = groups.iter_mut().rev().any(|group| {
                if group.is_contacting_frag(&fragment) {
                    group.as_mut().push(fragment.clone());
                    true
                } else {
                    false
                }
            });
            if !belongs_to_group {
                groups.push(Contacts::new(fragment))
            }
        }
        Contacts::group_recursive(groups)
    }

    /// convert this span into fragments applying endorsement
    /// of group into fragments
    ///
    /// returns (fragments, contacts) -
    /// The first element of the tuple: `fragments` are the resulting fragment after
    /// the endorsement such as rect, rounded rect from lines and arcs.
    ///
    /// The second element of the tuple: `contacts` are fragments that are touching together
    /// but can not form a fragment shape. These will be grouped in the svg nodes
    /// to keep them go together, when dragged (editing)
    pub(crate) fn endorse(self) -> (Vec<FragmentSpan>, Vec<Contacts>) {
        let mut fragments = vec![];
        let (top_left, _) = self.bounds().expect("must have bounds");
        let un_endorsed_span = if let Some((circle, un_endorsed_span)) =
            circle_map::endorse_circle_span(&self)
        {
            let circle = circle.absolute_position(top_left);
            let circle_frag_span =
                FragmentSpan::new(self.clone(), circle.into());
            fragments.push(circle_frag_span);
            un_endorsed_span
        } else if let Some((arc, un_endorsed_span)) =
            circle_map::endorse_quarter_arc_span(&self)
        {
            let arc = arc.absolute_position(top_left);
            let arc_frag_span = FragmentSpan::new(self.clone(), arc.into());
            fragments.push(arc_frag_span);
            un_endorsed_span
        } else {
            self
        };

        let groups: Vec<Contacts> = un_endorsed_span.get_contacts();
        let (rect_fragments, un_endorsed) = Contacts::endorse_rects(groups);

        fragments.extend(rect_fragments);

        (fragments, un_endorsed)
    }

    /// create a span of the cells that is inside of the start and end bound cells
    pub(crate) fn extract(&self, bound1: Cell, bound2: Cell) -> Self {
        Span(
            self.iter()
                .map(|(cell, ch)| (*cell, *ch))
                .filter(|(cell, _ch)| cell.is_bounded(bound1, bound2))
                .collect(),
        )
    }

    /// returns true if any cell on this span
    /// is within the bounds of `bound1` and `bound2`
    pub fn is_bounded(&self, bound1: Cell, bound2: Cell) -> bool {
        self.iter()
            .all(|(cell, ch)| cell.is_bounded(bound1, bound2))
    }

    pub fn hit_cell(&self, needle: Cell) -> bool {
        self.iter().any(|(cell, ch)| *cell == needle)
    }

    /// Convert a group of fragment span
    /// that didn't make it into an endorsed single shape fragment
    /// We try it again for endorsing to circle
    fn re_endorsed(
        grouped: Vec<Vec<FragmentSpan>>,
    ) -> (Vec<FragmentSpan>, Vec<Contacts>) {
        let spans = Self::extract_spans(grouped);
        let merged_spans: Vec<Span> = Self::merge_recursive(spans);

        let (endorsed_frag_spans, endorsed_contacts): (
            Vec<Vec<FragmentSpan>>,
            Vec<Vec<Contacts>>,
        ) = merged_spans.into_iter().map(|span| span.endorse()).unzip();

        (
            endorsed_frag_spans.into_iter().flatten().collect(),
            endorsed_contacts.into_iter().flatten().collect(),
        )
    }
    fn extract_spans(grouped: Vec<Vec<FragmentSpan>>) -> Vec<Self> {
        grouped
            .into_iter()
            .flat_map(|group| group.into_iter().map(|frag_span| frag_span.span))
            .collect()
    }
}

impl Merge for Span {
    fn merge(&self, other: &Self) -> Option<Self> {
        if self.can_merge(other) {
            let mut cells = self.0.clone();
            cells.extend(&other.0);
            Some(Span(cells))
        } else {
            None
        }
    }
}

impl Bounds {
    pub fn new(cell1: Cell, cell2: Cell) -> Self {
        let (top_left, bottom_right) = Cell::rearrange_bound(cell1, cell2);
        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn top_left(&self) -> Cell {
        self.top_left
    }

    pub fn bottom_right(&self) -> Cell {
        self.bottom_right
    }

    pub fn top_right(&self) -> Cell {
        Cell::new(self.bottom_right.x, self.top_left.y)
    }
    pub fn bottom_left(&self) -> Cell {
        Cell::new(self.top_left.x, self.bottom_right.y)
    }
}

/// create a property buffer for all the cells of this span
impl<'p> From<Span> for PropertyBuffer<'p> {
    fn from(span: Span) -> Self {
        let mut pb = PropertyBuffer::new();
        for (cell, ch) in span.iter() {
            if let Some(property) = Property::from_char(*ch) {
                pb.as_mut().insert(*cell, property);
            }
        }
        pb
    }
}

/// First we crate a property buffer based on the cell,char content of this span
/// and then based on the property, we extract the accurate fragments
///
/// If a character has no property, try to see if has equivalent fragments from unicode_map
/// otherwise add it to the fragment_buffer as a text fragment
impl Span {
    fn into_fragment_buffer(self) -> FragmentBuffer {
        let pb: PropertyBuffer = self.clone().into();
        let mut fb: FragmentBuffer = pb.clone().into_fragment_buffer();
        for (cell, ch) in self.iter() {
            if pb.as_ref().get(cell).is_none() {
                if let Some(fragments) = UNICODE_FRAGMENTS.get(ch) {
                    fb.add_fragments_to_cell(*cell, *ch, fragments.clone());
                } else {
                    fb.add_fragment_to_cell(
                        *cell,
                        *ch,
                        fragment::cell_text(*ch),
                    );
                }
            }
        }
        fb
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = StringBuffer::new();
        if let Some((tl, _br)) = self.bounds() {
            for (cell, ch) in self.iter() {
                if *ch != '\0' && !ch.is_whitespace() {
                    let local = tl.localize_cell(*cell);
                    buffer.add_char(local.x, local.y, *ch);
                }
            }
        }
        write!(f, "{}", buffer.to_string())
    }
}

#[cfg(test)]
mod test_span;
