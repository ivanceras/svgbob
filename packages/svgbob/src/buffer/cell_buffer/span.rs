use crate::{
    buffer::{
        cell_buffer::{Contacts, Endorse},
        fragment_buffer::FragmentSpan,
        FragmentBuffer, Property, PropertyBuffer, StringBuffer,
    },
    fragment,
    fragment::Circle,
    map::{circle_map, UNICODE_FRAGMENTS},
    Cell, Fragment, Merge, Point, Settings,
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

    fn top_left(&self) -> Cell {
        let bounds = self.bounds().expect("must have bounds");
        bounds.0
    }

    pub fn localize_point(&self, point: Point) -> Point {
        self.top_left().localize_point(point)
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

    /// convert this span into fragments applying endorsement
    /// of group into fragments
    ///
    ///
    /// TODO: return the rejects as Span, instead of Contacts
    pub(crate) fn endorse(self) -> Endorse<FragmentSpan, Span> {
        // try to endorse as circles or arcs
        let (mut accepted, un_endorsed_span): (Vec<FragmentSpan>, Span) =
            self.endorse_to_arcs_and_circles();

        // convert into contacts and try to endorse as rects fragments
        let un_endorsed_contacts: Vec<Contacts> = un_endorsed_span.into();
        let rect_endorsed: Endorse<FragmentSpan, Contacts> =
            Contacts::endorse_rects(un_endorsed_contacts);

        accepted.extend(rect_endorsed.accepted);

        let re_endorsed = Self::re_endorse(rect_endorsed.rejects);

        let mut endorsed = Endorse {
            accepted,
            rejects: vec![],
        };
        endorsed.extend(re_endorsed);
        endorsed
    }

    /// re try endorsing the contacts into arc and circles by converting it to span first
    fn re_endorse(rect_rejects: Vec<Contacts>) -> Endorse<FragmentSpan, Span> {
        // convert back to span
        let span_rejects: Vec<Span> = rect_rejects
            .into_iter()
            .map(|contact| contact.span())
            .collect();

        let span_rejects: Vec<Span> = Span::merge_recursive(span_rejects);

        // try to endorse as circles or arcs one more time
        let (accepted, rejects): (Vec<Vec<FragmentSpan>>, Vec<Span>) =
            span_rejects
                .into_iter()
                .map(|span| span.endorse_to_arcs_and_circles())
                .unzip();

        Endorse {
            accepted: accepted.into_iter().flatten().collect(),
            rejects,
        }
    }

    /// endorse this span into circles, half_circle, quarter_circle only
    fn endorse_to_arcs_and_circles(self) -> (Vec<FragmentSpan>, Span) {
        let mut accepted = vec![];
        let (top_left, _) = self.bounds().expect("must have bounds");
        let un_endorsed_span: Span = if let Some((circle, un_endorsed_span)) =
            circle_map::endorse_circle_span(&self)
        {
            let circle = circle.absolute_position(top_left);
            let circle_frag_span =
                FragmentSpan::new(self.clone(), circle.into());
            accepted.push(circle_frag_span);
            un_endorsed_span
        } else if let Some((three_quarters_arc, un_endorsed_span)) =
            circle_map::endorse_three_quarters_arc_span(&self)
        {
            let three_quarters_arc =
                three_quarters_arc.absolute_position(top_left);
            let three_quarters_arc_frag_span =
                FragmentSpan::new(self.clone(), three_quarters_arc.into());
            accepted.push(three_quarters_arc_frag_span);
            un_endorsed_span
        } else if let Some((half_arc, un_endorsed_span)) =
            circle_map::endorse_half_arc_span(&self)
        {
            let half_arc = half_arc.absolute_position(top_left);
            let half_arc_frag_span =
                FragmentSpan::new(self.clone(), half_arc.into());
            accepted.push(half_arc_frag_span);
            un_endorsed_span
        } else if let Some((arc, un_endorsed_span)) =
            circle_map::endorse_quarter_arc_span(&self)
        {
            let arc = arc.absolute_position(top_left);
            let arc_frag_span = FragmentSpan::new(self.clone(), arc.into());
            accepted.push(arc_frag_span);
            un_endorsed_span
        } else {
            self
        };
        (accepted, un_endorsed_span)
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

    /// merge as is without checking it it can
    pub fn merge_no_check(&self, other: &Self) -> Self {
        let mut cells = self.0.clone();
        cells.extend(&other.0);
        Span(cells)
    }
}

impl Merge for Span {
    fn merge(&self, other: &Self) -> Option<Self> {
        if self.can_merge(other) {
            Some(self.merge_no_check(other))
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

/// Grouping cell by adjacents are not enough
///
/// grouping them together when they are actually connected
/// is the most approprivate way of grouping
/// Span just provides an optimization of the number
/// of elements to be checked.
/// Only elements on the same span are checked to see if they
/// belong on the same group
///
impl From<Span> for Vec<Contacts> {
    fn from(span: Span) -> Vec<Contacts> {
        let fb = FragmentBuffer::from(span);
        let merged_fragments: Vec<FragmentSpan> = fb.merge_fragment_spans();
        let contacts: Vec<Contacts> = merged_fragments
            .into_iter()
            .map(|frag| Contacts::new(frag))
            .collect();
        Contacts::merge_recursive(contacts)
    }
}

/// First we crate a property buffer based on the cell,char content of this span
/// and then based on the property, we extract the accurate fragments
///
/// If a character has no property, try to see if has equivalent fragments from unicode_map
/// otherwise add it to the fragment_buffer as a text fragment
impl From<Span> for FragmentBuffer {
    fn from(span: Span) -> FragmentBuffer {
        let pb = PropertyBuffer::from(span.clone());
        let mut fb = FragmentBuffer::from(pb.clone());
        for (cell, ch) in span.iter() {
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
