use crate::{
    buffer::{cell_buffer::Contacts, FragmentBuffer, Property, PropertyBuffer, StringBuffer},
    fragment,
    map::{circle_map, UNICODE_FRAGMENTS},
    Cell, Fragment, Settings,
};
use itertools::Itertools;
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

/// A describes where a char came from relative to the source ascii text
/// The primary purpose of span is to group adjacent cell together
#[derive(Clone)]
pub struct Span(Vec<(Cell, char)>);

impl Deref for Span {
    type Target = Vec<(Cell, char)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Span {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Span {
    pub(super) fn new(cell: Cell, ch: char) -> Self {
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

    pub(super) fn merge(&mut self, other: &Self) {
        self.extend_from_slice(&*other)
    }

    /// returns the top_left most cell which aligns the top most and the left most cell.
    pub(crate) fn bounds(&self) -> Option<(Cell, Cell)> {
        if let Some((min_y, max_y)) = self.iter().map(|(cell, _)| cell.y).minmax().into_option() {
            if let Some((min_x, max_x)) = self.iter().map(|(cell, _)| cell.x).minmax().into_option()
            {
                Some((Cell::new(min_x, min_y), Cell::new(max_x, max_y)))
            } else {
                None
            }
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
    pub(crate) fn get_contacts(self, settings: &Settings) -> Vec<Contacts> {
        let localize_self = self.localize();
        let fb: FragmentBuffer = (&localize_self).into_fragment_buffer(settings);

        let mut groups: Vec<Contacts> = vec![];
        let merged_fragments = fb.merge_fragments();
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
        Self::group_recursive(groups)
    }

    fn group_recursive(groups: Vec<Contacts>) -> Vec<Contacts> {
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
    fn endorse_rects(groups: Vec<Contacts>) -> (Vec<Fragment>, Vec<Contacts>) {
        let mut fragments = vec![];
        let mut un_endorsed_rect: Vec<Contacts> = vec![];
        for group in groups {
            if let Some(fragment) = group.endorse_rects() {
                fragments.push(fragment);
            } else {
                un_endorsed_rect.push(group);
            }
        }
        (fragments, un_endorsed_rect)
    }

    /// [X](Done) TODO: this is trying to match all the members of this contact
    /// to each specific circle, this can be improve by checking
    /// subsets of the contacts to match the circles.
    ///
    /// This function is calling on endorse algorithmn on fragments that
    /// are neighbors, but not necessarily touching to be promoted to a shape.
    /// These includes: circle, arc, and line with arrow heads.
    fn endorse_circles_and_arcs(groups: Vec<Contacts>) -> (Vec<Fragment>, Vec<Contacts>) {
        let mut fragments = vec![];
        let mut un_endorsed_circles: Vec<Contacts> = vec![];
        if let Some((circle, unmatched)) = circle_map::endorse_circle(&groups) {
            fragments.push(circle.clone().into());
            for um in unmatched {
                un_endorsed_circles.push(groups[um].clone());
            }
        } else if let Some(arc) = circle_map::endorse_arc(&groups) {
            fragments.push(arc.clone().into());
        } else {
            un_endorsed_circles.extend(groups)
        }
        (fragments, un_endorsed_circles)
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
    pub(crate) fn endorse(self, settings: &Settings) -> (Vec<Fragment>, Vec<Contacts>) {
        let (top_left, _) = self.bounds().expect("mut have bounds");
        let groups: Vec<Contacts> = self.get_contacts(settings);
        // 1st phase, successful_endorses fragments, unendorsed one)
        let (mut fragments, un_endorsed) = Self::endorse_rects(groups);
        // 2nd phase, try to endorse to circles and arcs from the rejects of the 1st phase
        let (circle_fragments, un_endorsed) = Self::endorse_circles_and_arcs(un_endorsed);

        fragments.extend(circle_fragments);
        (
            fragments
                .iter()
                .map(|frag| frag.absolute_position(top_left))
                .collect(),
            un_endorsed
                .iter()
                .map(|group| group.absolute_position(top_left))
                .collect(),
        )
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
}

/// create a property buffer for all the cells of this span
impl<'p> Into<PropertyBuffer<'p>> for &Span {
    fn into(self) -> PropertyBuffer<'p> {
        let mut pb = PropertyBuffer::new();
        for (cell, ch) in self.iter() {
            if let Some(property) = Property::from_char(*ch) {
                pb.as_mut().insert(*cell, property);
            } else {
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
    fn into_fragment_buffer(&self, settings: &Settings) -> FragmentBuffer {
        let pb: PropertyBuffer = self.into();
        let mut fb: FragmentBuffer = pb.into_fragment_buffer(settings);
        for (cell, ch) in self.iter() {
            if pb.as_ref().get(cell).is_none() {
                if let Some(fragments) = UNICODE_FRAGMENTS.get(ch) {
                    fb.add_fragments_to_cell(*cell, fragments.clone());
                } else {
                    fb.add_fragment_to_cell(*cell, fragment::cell_text(*ch));
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
mod tests {
    use super::*;
    use crate::{
        buffer::{
            fragment::{Circle, Rect},
            CellBuffer,
        },
        Point,
    };

    #[test]
    fn test_bounds() {
        let art = r#"
.--------.
|________|
    "#;
        let buffer = CellBuffer::from(art);
        let adjacents = buffer.group_adjacents();
        assert_eq!(1, adjacents.len());
        let (min, max) = adjacents[0].bounds().unwrap();
        assert_eq!(min, Cell::new(0, 1));
        assert_eq!(max, Cell::new(9, 2));
    }

    #[test]
    fn test_localize() {
        let art = r#"



        .--------.
        |________|
    "#;
        let buffer = CellBuffer::from(art);
        let adjacents = buffer.group_adjacents();
        assert_eq!(1, adjacents.len());
        let (min, max) = adjacents[0].bounds().unwrap();
        assert_eq!(min, Cell::new(8, 4));
        assert_eq!(max, Cell::new(17, 5));

        let localize = adjacents[0].clone().localize();
        let (min, max) = localize.bounds().unwrap();
        assert_eq!(min, Cell::new(0, 0));
        assert_eq!(max, Cell::new(9, 1));
    }

    #[test]
    fn test_1span_1group() {
        let art = r#"
+--------+
|        |
+--------+
    "#;
        let buffer = CellBuffer::from(art);
        println!("buffer: {}", buffer);
        let mut adjacents = buffer.group_adjacents();
        println!("There are {} adjacents", adjacents.len());
        assert_eq!(1, adjacents.len());
        let span = adjacents.remove(0);
        let (top_left, _) = span.bounds().unwrap();
        assert_eq!(top_left, Cell::new(0, 1));
        let groups: Vec<Contacts> = span.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group {} is: \n{}", i, group);
        }
        assert_eq!(groups.len(), 1);
        let rect = groups[0].endorse_rects().unwrap();
        let expected = Fragment::Rect(Rect::new(
            Point::new(0.5, 1.0),
            Point::new(9.5, 5.0),
            false,
            false,
        ));
        assert_eq!(rect, expected);
    }

    #[test]
    /// 2 spans and 1 group for each span
    fn test_2spans_1group_for_each_span() {
        let art = r#"
+--------+
|        |
+--------+

+--------+
|        |
+--------+
    "#;
        let buffer = CellBuffer::from(art);
        println!("buffer: {}", buffer);
        let mut spans = buffer.group_adjacents();
        println!("There are {} adjacents", spans.len());
        assert_eq!(2, spans.len());
        let span2 = spans.remove(1);
        let span1 = spans.remove(0);

        let (bound1, _) = span1.bounds().unwrap();
        let (bound2, _) = span2.bounds().unwrap();
        assert_eq!(bound1, Cell::new(0, 1));
        assert_eq!(bound2, Cell::new(0, 5));
        let settings = &Settings::default();
        let groups1: Vec<Contacts> = span1.get_contacts(settings);
        let groups2: Vec<Contacts> = span2.get_contacts(settings);
        assert_eq!(groups1.len(), 1);
        assert_eq!(groups2.len(), 1);

        let rect1 = groups1[0].endorse_rects().unwrap();
        let rect2 = groups2[0].endorse_rects().unwrap();
        assert_eq!(
            rect1,
            Fragment::Rect(Rect::new(
                Point::new(0.5, 1.0),
                Point::new(9.5, 5.0),
                false,
                false
            ))
        );
        assert_eq!(
            rect2,
            Fragment::Rect(Rect::new(
                Point::new(0.5, 1.0),
                Point::new(9.5, 5.0),
                false,
                false
            ))
        );
    }

    #[test]
    /// 2 spans and 1 group for each span
    fn test_1spans_2group_for_each_span() {
        let art = r#"
.--------.
|        |
'--------'
.--------.
|        |
'--------'
    "#;
        let buffer = CellBuffer::from(art);
        println!("buffer: {}", buffer);
        let mut spans = buffer.group_adjacents();
        println!("There are {} adjacents", spans.len());
        assert_eq!(1, spans.len());
        let span1 = spans.remove(0);
        let (bound1, _) = span1.bounds().unwrap();
        assert_eq!(bound1, Cell::new(0, 1));
        let groups: Vec<Contacts> = span1.get_contacts(&Settings::default());
        assert_eq!(groups.len(), 2);

        let rect1 = groups[0].endorse_rects().unwrap();
        let rect2 = groups[1].endorse_rects().unwrap();
        assert_eq!(
            rect1,
            Fragment::Rect(Rect::rounded_new(
                Point::new(0.5, 1.0),
                Point::new(9.5, 5.0),
                false,
                0.5,
                false,
            ))
        );
        assert_eq!(
            rect2,
            Fragment::Rect(Rect::rounded_new(
                Point::new(0.5, 7.0),
                Point::new(9.5, 11.0),
                false,
                0.5,
                false
            ))
        );
    }

    #[test]
    fn test_endorse_circle() {
        let art = r#"
         .--.
        (    )
         `--'
    "#;
        let buffer = CellBuffer::from(art);
        println!("buffer: {}", buffer);
        let mut adjacents = buffer.group_adjacents();
        println!("There are {} adjacents", adjacents.len());
        assert_eq!(1, adjacents.len());
        let span = adjacents.remove(0);
        let (top_left, _) = span.bounds().unwrap();
        assert_eq!(top_left, Cell::new(8, 1));
        let (mut fragments, _groups) = span.endorse(&Settings::default());
        for (i, frag) in fragments.iter().enumerate() {
            println!("frag {}:\n{}", i, frag);
        }
        assert_eq!(fragments.len(), 1);
        let circle = fragments.remove(0);
        assert_eq!(
            circle,
            Fragment::Circle(Circle::new(Point::new(11.0, 5.0), 2.5, false))
        );
    }

    #[test]
    fn test_endorse_circle_with_rect() {
        let art = r#"
         .--.
        (    )
         `--'
         +---------+
         |         |
         +---------+

    "#;
        let buffer = CellBuffer::from(art);
        println!("buffer: {}", buffer);
        let mut adjacents = buffer.group_adjacents();
        println!("There are {} adjacents", adjacents.len());
        assert_eq!(1, adjacents.len());
        let span1 = adjacents.remove(0);
        let (top_left1, _) = span1.bounds().unwrap();
        assert_eq!(top_left1, Cell::new(8, 1));
        let (mut fragments, _groups) = span1.endorse(&Settings::default());
        assert_eq!(fragments.len(), 2);

        let rect = fragments.remove(0);
        assert!(rect.is_rect());
        assert_eq!(
            rect,
            Fragment::Rect(Rect::new(
                Point::new(9.5, 9.0),
                Point::new(19.5, 13.0),
                false,
                false
            ))
        );

        let circle = fragments.remove(0);
        assert!(circle.is_circle());
        assert_eq!(
            circle,
            Fragment::Circle(Circle::new(Point::new(11.0, 5.0), 2.5, false))
        );
    }

    #[test]
    fn test_endorse_with_big_circle() {
        let art = r#"


                _.-''''''-._
              ,'            `.
             /                \
            .                  .
            |                  |
            |                  |
            |                  |
             \                /
              `._          _.'
                 '-......-'

    "#;
        let buffer = CellBuffer::from(art);
        println!("buffer: {}", buffer);
        let mut adjacents = buffer.group_adjacents();
        println!("There are {} adjacents", adjacents.len());
        assert_eq!(1, adjacents.len());
        let span1 = adjacents.remove(0);
        let (top_left1, _) = span1.bounds().unwrap();
        assert_eq!(top_left1, Cell::new(12, 3));
        let (mut fragments, _groups) = span1.endorse(&Settings::default());
        assert_eq!(fragments.len(), 1);

        let circle = fragments.remove(0);
        assert!(circle.is_circle());
        assert_eq!(
            circle,
            Fragment::Circle(Circle::new(Point::new(22.0, 17.0), 9.5, false))
        );
    }

    #[test]
    fn test_endorse_with_big_circle_extra_match() {
        let art = r#"


                _.-''''''-._
              ,'            `.
             /                \
            .                  .
            |                  |
            | ---------------- |
            |                  |
             \                /
              `._          _.'
                 '-......-'

    "#;
        let buffer = CellBuffer::from(art);
        println!("buffer: {}", buffer);
        let mut adjacents = buffer.group_adjacents();
        println!("There are {} adjacents", adjacents.len());
        assert_eq!(2, adjacents.len());
        let span1 = adjacents.remove(0);
        let (top_left1, _) = span1.bounds().unwrap();
        assert_eq!(top_left1, Cell::new(12, 3));
        let (fragments, groups) = span1.endorse(&Settings::default());
        assert_eq!(fragments.len(), 1);
        assert_eq!(groups.len(), 0);
        for (i, fragment) in groups.iter().enumerate() {
            println!("fragment {}: \n{}", i, fragment);
        }
    }

    #[test]
    fn test_absolute_positions() {
        let art = r#"
        This is some sort of a paragraph
        with long words such as supercalifragilisticexpialiducios
        and pneumonoultramicrospocisilicovulcanoconiosis

        Loren ipsum is a second paragrapah.
        This is in one span since all the letters
        are adjacent since they are right next to
        each other taking account the top and bottom
        neighbor cell
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 2);
        let span1 = spans.remove(1);
        let span0 = spans.remove(0);
        let (top_left0, _br) = span0.bounds().unwrap();
        assert_eq!(top_left0, Cell::new(8, 1));
        let (top_left1, _br) = span1.bounds().unwrap();
        assert_eq!(top_left1, Cell::new(8, 5));

        let groups1 = span1.get_contacts(&Settings::default());
        let since = groups1[11].as_ref()[0].as_cell_text().unwrap();
        assert_eq!(since.content, "since");
        assert_eq!(since.start, Cell::new(20, 1));
        let abs_since = since.absolute_position(top_left1);
        assert_eq!(abs_since.content, "since");
        assert_eq!(abs_since.start, Cell::new(28, 6));
    }
}
