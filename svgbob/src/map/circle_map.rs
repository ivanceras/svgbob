use crate::{
    buffer::{CellBuffer, Contacts, Span},
    fragment,
    fragment::Circle,
    Cell, Point, Settings,
};
use lazy_static::lazy_static;
use std::{collections::BTreeMap, iter::FromIterator};

// These are circle map, when a group is detected to have these set of characters
// arrange together in such this way, then endorse them as a circle
// Each of these character formation will have a certain circle parameters: center, and radius.
//
lazy_static! {

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
    ///
    pub static ref CIRCLE_MAP: Vec<(&'static str, Cell, Point, f32)> =
        vec![
            // CIRCLE_1
            //center 0,0,o, radius = 0.5
            (r#"
            ()
            "#, Cell::new(0,0), Cell::new(0,0).o(), 0.5),

            // CIRCLE_2
            //center = 1,1,m radius = 1.0
            (r#"
            (_)
            "#, Cell::new(1,0), Cell::new(1,0).m(), 1.0),

            // CIRCLE_3
            //center = 1,1,o radius = 1.5,
            (r#"
             __
            (__)
            "#, Cell::new(1,1), Cell::new(1,1).o(), 1.5),

            // CIRCLE_4
            //center: 2,1,m radius: 2.0
            (r#"
             ,-.
            (   )
             `-'
            "#, Cell::new(2,1), Cell::new(2,1).m(), 2.0),

            // CIRCLE_5
            //center: 2,1,o radius: 2.5
            (r#"
             .--.
            (    )
             `--'
            "#, Cell::new(2,1), Cell::new(2,1).o(), 2.5),

            // CIRCLE_6
            //center: 3,2,m radius: 3.0
            (r#"
               _
             .' '.
            (     )
             `._.'
            "#, Cell::new(3,2), Cell::new(3,2).m(), 3.0),

            // CIRCLE_7
            //center: 3,2,o radius: 3.5
            (r#"
               __
             ,'  '.
            (      )
             `.__.'
            "#, Cell::new(3,2), Cell::new(3,2).o(), 3.5),

            // CIRCLE_8
            //center: 4,2,m radius:4.0
            (r#"
               ___
             ,'   '.
            (       )
             `.   .'
               `-'
            "#, Cell::new(4,2), Cell::new(4,2).m(), 4.0),

            // circle 9 and up can be divided into 4 quadrants these quadrants can be arcs and can be used as
            // rounded edge with larger radius for rouded rect
            // CIRCLE_9
            //center: 4,2,w radius: 4.5
            (r#"
               ___
             ,'   `.
            /       \
            \       /
             `.___.'
            "#, Cell::new(4,2), Cell::new(4,2).w(), 4.5),

            // CIRCLE_10
            //center: 4,2,y radius: 5.0
            (r#"
               ____
             ,'    `.
            /        \
            \        /
             `.____.'
            "#, Cell::new(4,2), Cell::new(4,2).y(), 5.0),

            // CIRCLE_11
            //center:5,3,o radius: 5.5
            (r#"
                ____
              .'    `.
             /        \
            (          )
             \        /
              `.____.'
            "#, Cell::new(5,3), Cell::new(5,3).o(), 5.5),

            // CIRCLE_12
            //center:6,3,m radius: 6.0
            (r#"
                _____
              ,'     `.
             /         \
            (           )
             \         /
              `._____.'
            "#, Cell::new(6,3), Cell::new(6,3).m(), 6.0),

            // CIRCLE_13
            //center: 6,3,y radius: 6.5
            (r#"
                ______
              ,'      `.
             /          \
            |            |
            |            |
             \          /
              `.______.'
            "#, Cell::new(6,3), Cell::new(6,3).y(), 6.5),

            // CIRCLE_14
            //center: 7,3,w radius: 7.0
            (r#"
                _______
              ,'       `.
             /           \
            |             |
            |             |
             \           /
              `._______.'
            "#, Cell::new(7,3), Cell::new(7,3).w(), 7.0),


            // CIRCLE_15
            //center: 7,4,o radius: 7.5
            (r#"
                ________
              ,'        `.
             /            \
            |              |
            |              |
            |              |
             \            /
              `.________.'
            "#, Cell::new(7,4), Cell::new(7,4).o(), 7.5),

            // CIRCLE_16
            //center: 8,4,m radius: 8.0
            (r#"
                __-----__
              ,'         `.
             /             \
            |               |
            |               |
            |               |
             \             /
              `.         .'
                `-------'
            "#, Cell::new(8,4), Cell::new(8,4).m(), 8.0),

            // CIRCLE_17
            //center: 8,4,o radius: 8.5
            (r#"
                .--------.
              ,'          `.
             /              \
            |                |
            |                |
            |                |
             \              /
              `.          .'
                `--------'
            "#, Cell::new(8,4), Cell::new(8,4).o(), 8.5),

            // CIRCLE_18
            //center:9,5,m radius: 9.0
            (r#"
                _.-'''''-._
              ,'           `.
             /               \
            .                 .
            |                 |
            |                 |
            |                 |
             \               /
              `._         _.'
                 '-.....-'
            "#, Cell::new(9,5), Cell::new(9,5).m(), 9.0),

            // CIRCLE_19
            // center: 9,5,o radius: 9.5
            (r#"
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
            "#, Cell::new(9,5), Cell::new(9,5).o(), 9.5),


            // CIRCLE_20
            // center: 10,5,m radius: 10
            (r#"
                _.-'''''''-._
              ,'             `.
             /                 \
            .                   .
            |                   |
            |                   |
            |                   |
             \                 /
              `._           _.'
                 '-.......-'
            "#, Cell::new(10,5), Cell::new(10,5).m(), 10.0),
        ];


    /// The fragments for each of the circle
    /// Calculate the span and get the group fragments
    pub static ref FRAGMENTS_CIRCLE: Vec<(Vec<Contacts>,Circle)> = Vec::from_iter(
        CIRCLE_MAP.iter().map(|(art, center_cell, center, radius)|{
            (circle_art_to_group(art, &Settings::default()), Circle::new(*center, *radius, false))
        })
    );

    /// There is only 1 span per circle, and localized
    pub static ref CIRCLES_SPAN: BTreeMap<Circle, Span> = BTreeMap::from_iter(
        CIRCLE_MAP.iter().map(|(art, center_cell, center, radius)|{
            let cb = CellBuffer::from(*art);
            let mut spans = cb.group_adjacents();
            assert_eq!(spans.len(), 1);
            let span = spans.remove(0).localize();
            (Circle::new(*center, *radius, false), span)
        })
    );

    /// Build a fragment for the characters in the range
    /// extract the characters
    ///
    ///  top_left arc:   top_left to center cell
    ///
    ///  top_right arc:  top_right to center_cell
    ///
    ///  bottom_left arc: bottom_left to center_cell
    ///
    ///  bottom_right arc: bottom_right to center_cell
    pub static ref FRAGMENTS_ARC: Vec<(Vec<Contacts>,fragment::Arc)> =Vec::from_iter(
            CIRCLE_MAP.iter().skip(3).flat_map(|(art, center_cell, center, radius)|{
                let cb = CellBuffer::from(*art);
                let mut spans = cb.group_adjacents();
                assert_eq!(spans.len(), 1);
                let span = spans.remove(0).localize();
                let (tl_bounds, br_bounds) = span.bounds().expect("There should be bounds");

                let center_cell_br_bound = center_cell.bottom_right_most();
                // inlude the center cell for x when this is true, when deriving a span for arc1, arc3, arc4
                let eq_include_center_x = center.x <= center_cell_br_bound.x;
                // include the center cell for y when this is true, when deriving a span for arc1, arc3, arc4
                let eq_include_center_y = center.y <= center_cell_br_bound.y;

                let strict_include_center_x = center.x < center_cell_br_bound.x;
                let strict_include_center_y = center.y < center_cell_br_bound.y;

                //
                // ARC 1
                //    __
                //   |  '.
                //   |    \
                //   |_____|
                //
                let arc1_top_right = Cell::new(br_bounds.x, tl_bounds.y);
                let arc1_center_cell = Cell::new(
                    if strict_include_center_x { center_cell.x } else { center_cell.x + 1},
                    if eq_include_center_y { center_cell.y } else { center_cell.y + 1});

                let arc1_top_left = Cell::new(arc1_center_cell.x, tl_bounds.y);
                let arc1_center = arc1_top_left.localize_point(*center);


                let arc1_span = span.extract(arc1_center_cell, arc1_top_right);
                let arc1  = fragment::Arc::new(Point::new(arc1_center.x + *radius, arc1_center.y), Point::new(arc1_center.x, arc1_center.y - *radius) , *radius);

                //
                // ARC 2
                //      __
                //    .'  |
                //   /    |
                //  |_____|
                //

                let arc2_top_left = Cell::new(tl_bounds.x, tl_bounds.y);
                let arc2_span = span.extract(arc2_top_left, *center_cell);
                let arc2 = fragment::Arc::new(Point::new(center.x, center.y - radius), Point::new(center.x - radius , center.y), *radius);


                //
                //  ARC 3
                //  ______.
                //  |     |
                //   \    |
                //    `.__|
                //
                //
                let arc3_center_cell = Cell::new(
                    if eq_include_center_x { center_cell.x } else { center_cell.x + 1},
                    if strict_include_center_y { center_cell.y } else { center_cell.y + 1}
                );
                let arc3_top_left = Cell::new(tl_bounds.x, arc3_center_cell.y);
                let arc3_bottom_right = Cell::new(arc3_center_cell.x, br_bounds.y);
                let arc3_center = arc3_top_left.localize_point(*center);
                let arc3_span = span.extract(arc3_top_left, arc3_bottom_right);
                let arc3 = fragment::Arc::new(Point::new(arc3_center.x - radius, arc3_center.y), Point::new(arc3_center.x, arc3_center.y + radius), *radius);


                //
                // ARC 4
                //
                //   .______
                //   |     |
                //   |    /
                //   |__.'
                //
                let arc4_center_cell = Cell::new(
                    if strict_include_center_x { center_cell.x } else { center_cell.x + 1},
                    if strict_include_center_y { center_cell.y } else { center_cell.y + 1}
                );
                let arc4_top_left = Cell::new(arc4_center_cell.x, arc4_center_cell.y);
                let arc4_bottom_right = Cell::new(br_bounds.x, br_bounds.y);
                let arc4_center = arc4_top_left.localize_point(*center);
                let arc4_span = span.extract(arc4_top_left, arc4_bottom_right);
                let arc4 = fragment::Arc::new(Point::new(arc4_center.x, arc4_center.y + radius), Point::new(arc4_center.x + radius, arc4_center.y), *radius);

                let settings = &Settings::default();

                vec![
                    (arc1_span.get_contacts(settings), arc1),
                    (arc2_span.get_contacts(settings), arc2),
                    (arc3_span.get_contacts(settings), arc3),
                    (arc4_span.get_contacts(settings), arc4),
                ]
            })
    );
}

fn circle_art_to_group(art: &str, settings: &Settings) -> Vec<Contacts> {
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.group_adjacents();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    span1.get_contacts(settings)
}

/// [X](Done) TODO: search only the subset of contacts that matches the circle.
/// if it is a subset then the circle is matched and the non-matching ones are returned
pub fn endorse_circle(search: &Vec<Contacts>) -> Option<(&Circle, Vec<usize>)> {
    FRAGMENTS_CIRCLE
        .iter()
        .rev()
        .find_map(|(contacts, circle)| {
            let (matched, unmatched) = is_subset_of(contacts, search);
            if matched {
                Some((circle, unmatched))
            } else {
                None
            }
        })
}

/// returns true if all the contacts in subset is in big_set
/// This also returns the indices of big_set that are not found in the subset
fn is_subset_of(subset: &Vec<Contacts>, big_set: &Vec<Contacts>) -> (bool, Vec<usize>) {
    let mut unmatched = vec![];
    let mut matched = 0;
    for (i, set) in subset.iter().enumerate() {
        if big_set.contains(set) {
            matched += 1;
        }
    }
    for (bi, bset) in big_set.iter().enumerate() {
        if !subset.contains(bset) {
            unmatched.push(bi);
        }
    }
    (matched == subset.len(), unmatched)
}

/// searches from the largest arc first
/// [x](solved) ISSUE: An arc can have multiple matches at different radius.
/// searching from the largest or from the smallest will still result in 4 parts not equally
/// proportional, due to 1 of the arc not matching on the same arc level since the
/// arc parameters is determine only by the contacts, and have no clue about it's radius.
/// Multiple arc contacts can be the same while not having the same arc radius.
pub fn endorse_arc(search: &Vec<Contacts>) -> Option<&fragment::Arc> {
    FRAGMENTS_ARC
        .iter()
        .rev()
        .find_map(|(contacts, arc)| if contacts == search { Some(arc) } else { None })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle1() {
        let art = r#"
                _.-'''''''-._
              ,'             `.
             /                 \
            .                   .
            |                   |
            |                   |
            |                   |
             \                 /
              `._           _.'
                 '-.......-'
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        assert_eq!(11, groups.len());
    }

    #[test]
    fn test_arc9_top_right() {
        let art = r#"
            __
              `.
                \
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        assert_eq!(2, groups.len());
        let arc = endorse_arc(&groups);
        assert!(arc.is_some());
    }
}
