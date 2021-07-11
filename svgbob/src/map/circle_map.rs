use crate::{
    buffer::{CellBuffer, Contacts, Span},
    fragment,
    fragment::Circle,
    Cell, Point, Settings,
};
use lazy_static::lazy_static;
use std::{collections::BTreeMap, iter::FromIterator};

#[derive(PartialEq, Debug, Clone, Copy)]
/// edge cases of the circle art
pub enum EdgeCase {
    /// circle arc is touching the edge of the first cell
    /// ie: if the left most cell is `/` then it is touching the dge
    StartEdge,
    /// if the left most cell is `(` or `|` then it starts at half the cell
    StartHalf,
}

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
    /// art, edge_case, top_offset to center point
    static ref CIRCLE_ART_MAP: Vec<(&'static str, EdgeCase, f32)> =
        vec![
            // CIRCLE_1
            //center 1,0,k, radius = 0.5
            // 2 cell width , radius formula: (n-1)/2 = (2-1)/2 = 0.5
            // where n is the number of cells used
            //  if edge_case starts at edge then n is added by 1.
            //  vert_mid: half (0.5/1.0)
            //  cx_lies: edge
            //  cy_lies: mid
            //
            //  edge_case: start_half = 0.5, start_edge = 0.0
            //  if radius +  edge_case has 0.5 then mid, 0.0 them edge
            //
            //
            (r#"
            ()
            "#, EdgeCase::StartHalf, 0.5),

            // CIRCLE_2
            //center = 1,1,m radius = 1.0
            // 3 cell width, (n-1)/2 = (3-1)/2  = 1.0
            // vert_mid: half  (0.5/1.0)
            // cx_lies: mid
            // cy_lies: mid
            (r#"
            (_)
            "#, EdgeCase::StartHalf, 0.5),

            // CIRCLE_3
            //center = 1,1,o radius = 1.5,
            // 4 cell width, (n-1)/2 = (4-1)/2 = 1.5
            // vert_mid: 3/4 (1.5/2.0)
            // cx_lies: edge
            // cy_lies: mid
            (r#"
             __
            (__)
            "#,  EdgeCase::StartHalf, 1.5),

            // CIRCLE_4
            //center: 2,1,m radius: 2.0
            //  5 cell width, (n-1)/2 = (5-1)/2 = 2.0
            //  vert_mid: half (1.5/3.0)
            // cx_lies: mid
            // cy_lies: mid
            (r#"
             ,-.
            (   )
             `-'
            "#,  EdgeCase::StartHalf, 1.5),

            // CIRCLE_5
            //center: 2,1,o radius: 2.5
            // 6 cell width, (n-1)/2 = (6-1)/2 = 2.5
            // vert_mid: half (1.5/3.0)
            //  cx_lies: edge
            //  cy_lies: mid
            (r#"
             .--.
            (    )
             `--'
            "#, EdgeCase::StartHalf, 1.5),

            // CIRCLE_6
            //center: 3,2,m radius: 3.0
            // 7 cell width, (n-1)/2 = (7-1)/2 = 3.0
            // vert_mid: 2.5/4
            // cx_lies: mid
            // cy_lies: mid
            (r#"
               _
             .' '.
            (     )
             `._.'
            "#, EdgeCase::StartHalf, 2.5),

            // CIRCLE_7
            //center: 3,2,o radius: 3.5
            // 8 cell width, (n-1)/2 = (8-1)/2 = 3.5
            // vert_mid: 2.5/4
            //  cx_lies: edge
            //  cy_lies: mid
            (r#"
               __
             ,'  '.
            (      )
             `.__.'
            "#, EdgeCase::StartHalf, 2.5),

            // CIRCLE_8
            //center: 4,2,m radius:4.0
            // 9 cell width, (n-1)/2 = (9-1)/2 = 4.0
            // vert_mid: half (2.5/5.0)
            // cx_lies: mid
            // cy_lies: mid
            (r#"
               ___
             ,'   '.
            (       )
             `.   .'
               `-'
            "#, EdgeCase::StartHalf, 2.5),

            // circle 9 and up can be divided into 4 quadrants these quadrants can be arcs and can be used as
            // rounded edge with larger radius for rouded rect
            // CIRCLE_9
            //center: 4,2,w radius: 4.5
            // start_edge: true
            // 9 cell width, (n-0)/2 = (9-0)/2 = 4.5
            // vert_mid:  3.0/5.0
            //  cx_lies: mid
            //  cy_lies: edge
            (r#"
               ___
             ,'   `.
            /       \
            \       /
             `.___.'
            "#,  EdgeCase::StartEdge, 3.0 ),

            // CIRCLE_10
            //center: 4,2,y radius: 5.0
            //start_edge: true
            // 10 cell width, (n-0)/2 = (10-0)/2 = 5.0
            // vert_mid:  3.0/5.0
            //  cx_lies: edge
            //  cy_lies: edge
            (r#"
               ____
             ,'    `.
            /        \
            \        /
             `.____.'
            "#,  EdgeCase::StartEdge, 3.0),

            // CIRCLE_11
            //center:5,3,o radius: 5.5
            // 12 cell width, (n-1)/2 = (12-1)/2 = 5.5
            // vert_mid:  3.5/6.0
            //  cx_lies: edge
            //  cy_lies: mid
            (r#"
                ____
              .'    `.
             /        \
            (          )
             \        /
              `.____.'
            "#, EdgeCase::StartHalf , 3.5),

            // CIRCLE_12
            //center:6,3,m radius: 6.0
            // 13 cell width, (n-1)/2 = (13-1)/2 = 6.0
            // vert_mid: 3.5/6.0
            // cx_lies: mid
            // cy_lies: mid
            (r#"
                _____
              ,'     `.
             /         \
            (           )
             \         /
              `._____.'
            "#, EdgeCase::StartHalf, 3.5),

            // CIRCLE_13
            // center: 6,3,y radius: 6.5
            // vert_mid: 4.0/7.0
            //  cx_lies: edge
            //  cy_lies: edge
            (r#"
                ______
              ,'      `.
             /          \
            |            |
            |            |
             \          /
              `.______.'
            "#, EdgeCase::StartHalf, 4.0),

            // CIRCLE_14
            //center: 7,3,w radius: 7.0
            //vert_mid: 4.0/7.0
            //  cx_lies: mid
            //  cy_lies: edge
            (r#"
                _______
              ,'       `.
             /           \
            |             |
            |             |
             \           /
              `._______.'
            "#, EdgeCase::StartHalf , 4.0),


            // CIRCLE_15
            //center: 7,4,o radius: 7.5
            //vert_mid: 4.5/8.0
            //  cx_lies: edge
            //  cy_lies: mid
            (r#"
                ________
              ,'        `.
             /            \
            |              |
            |              |
            |              |
             \            /
              `.________.'
            "#, EdgeCase::StartHalf, 4.5),

            // CIRCLE_16
            //center: 8,4,m radius: 8.0
            //vert_mid: 4.5/9.0
            // cx_lies: mid
            // cy_lies: mid
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
            "#, EdgeCase::StartHalf, 4.5),

            // CIRCLE_17
            //center: 8,4,o radius: 8.5
            // vert_mid:  4.5/9.0
            //  cx_lies: edge
            //  cy_lies: mid
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
            "#, EdgeCase::StartHalf, 4.5),

            // CIRCLE_18
            //center:9,5,m radius: 9.0
            //vert_mid: 5.5/10.0
            // cx_lies: mid
            // cy_lies: mid
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
            "#, EdgeCase::StartHalf, 5.5),

            // CIRCLE_19
            // center: 9,5,o radius: 9.5
            // vert_mid:  5.5/10.0
            //  cx_lies: edge
            //  cy_lies: mid
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
            "#,  EdgeCase::StartHalf, 5.5),


            // CIRCLE_20
            // center: 10,5,m radius: 10
            // vert_mid: 5.5/10.0
            // cx_lies: mid
            // cy_lies: mid
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
            "#, EdgeCase::StartHalf, 5.5),

            // CIRCLE_21
            // center: 10,5,o radius: 10.5
            // vert_mid: 5.5/11.0
            //  cx_lies: edge
            //  cy_lies: mid
            (r#"
                _.-''''''''-._
              ,'              `.
             /                  \
            .                    .
            |                    |
            |                    |
            |                    |
            |                    |
             \                  /
              `._            _.'
                 '-........-'
            "#, EdgeCase::StartHalf, 5.5),

            // CIRCLE_22
            // center: 10,5,m radius: 11
            // radius = (n-1)/2 = (23-1)/2 = 11
            // vert_mid: 5.5/11.0
            // cx_lies: mid
            // cy_lies: mid
            (r#"
                _.-'''''''''-._
              ,'               `.
             /                   \
            .                     .
            |                     |
            |                     |
            |                     |
            |                     |
             \                   /
              `._             _.'
                 '-.........-'
            "#, EdgeCase::StartHalf, 5.5),

        ];


    pub static ref CIRCLE_MAP: Vec<(&'static str, Point, f32, EdgeCase, f32)> =Vec::from_iter(
        CIRCLE_ART_MAP.iter().enumerate().map(|(ndx, (art, edge_case, offset_center_y))|{
            let cb = CellBuffer::from(*art);
            let (lo, hi) = cb.bounds().expect("circle must have bounds");

           let width = match *edge_case {
               EdgeCase::StartEdge => {
                    (hi.x - lo.x) as f32 + 1.0
               }
               EdgeCase::StartHalf => {
                    (hi.x - lo.x) as f32
               }
           };

           let calc_radius = width / 2.0;
           let index = (width - 1.0) as usize;
           assert_eq!(ndx, index);

            let edge_inc_x = match edge_case{
                EdgeCase::StartEdge => 0.0,
                EdgeCase::StartHalf => 0.5,
            };

            let calc_center_x = calc_radius + edge_inc_x;
            let calc_center_y = offset_center_y * 2.0;
            let calc_center = Point::new(calc_center_x, calc_center_y);

            (*art, calc_center, calc_radius, *edge_case, *offset_center_y)
        })
    );

    /// The fragments for each of the circle
    /// Calculate the span and get the group fragments
    pub static ref FRAGMENTS_CIRCLE: Vec<(Vec<Contacts>,Circle)> = Vec::from_iter(
        CIRCLE_MAP.iter().map(|(art, center, radius, edge_case, offset_center_y)|{
            (circle_art_to_group(art, &Settings::default()), Circle::new(*center, *radius, false))
        })
    );

    /// map of circle spans and their radius
    pub static ref DIAMETER_CIRCLE: BTreeMap<i32,(Point,Span)> =BTreeMap::from_iter(
        CIRCLE_MAP.iter().map(|(art, center, radius, edge_case, offset_center_y)|{
            let cb = CellBuffer::from(*art);
            let mut spans = cb.group_adjacents();
            assert_eq!(spans.len(), 1);
            let span = spans.remove(0).localize();
            ((*radius * 2.0) as i32, (*center, span))
        })
    );

    /// There is only 1 span per circle, and localized
    pub static ref CIRCLES_SPAN: BTreeMap<Circle, Span> = BTreeMap::from_iter(
        CIRCLE_MAP.iter().map(|(art, center, radius, edge_case, offset_center_y)|{
            let cb = CellBuffer::from(*art);
            let mut spans = cb.group_adjacents();
            assert_eq!(spans.len(), 1);
            let span = spans.remove(0).localize();
            (Circle::new(*center, *radius, false), span)
        })
    );


    /// Simplified version of fragment arcs derived from CIRCLE_MAP
    /// Algorithm:
    /// 1. Locate the cells corresponding to the 4 quadrant boundary of the circle
    ///
    /// top_left      top       top_right
    ///               p2
    ///          arc2  |    arc1
    ///                |
    ///    left  p3----+----- p1 right
    ///                |
    ///          arc3  |   arc4
    ///               p4
    /// bottom_left  bottom   bottom_right
    ///
    ///   p1 = center.x + radius, center.y
    ///   p2 = center.x, center.y - radius
    ///   p3 = center.x - radius, center.y
    ///   p4 = center.x + radius, center.y + radius
    ///
    ///  right,top,left,bottom, top_left, top_right,bottom_left,bottom_right are cells dervied from
    ///  bounds.
    ///
    ///  assert that snapping points to cell corresponds respectively:
    ///     p1 -> right cell
    ///     p2 -> top cell
    ///     p3 -> left cell
    ///     p4 -> bottom cell
    ///
    /// 2. Locate the span of each quadrants
    ///    arc4 span is always center_cell, to bottom_right
    ///    arc2 span is conditional
    ///     if center cell lies on mid, the span coverage is inclusive of the center cell.
    ///     otherwise if the center cell lies on the edge, then the cell coverage is exclusive of
    ///     the center cell
    ///
    pub static ref FRAGMENTS_ARC: Vec<(Vec<Contacts>,fragment::Arc)> =Vec::from_iter(
            CIRCLE_MAP.iter().skip(3).flat_map(|(art, center, radius, edge_case, offset_center_y)|{
                let cb = CellBuffer::from(*art);
                let mut spans = cb.group_adjacents();
                assert_eq!(spans.len(), 1);
                let span = spans.remove(0).localize();

                let (start_bound, end_bound) = span.bounds().expect("There should be bounds");

                let center_cell = center.cell();

                let right = Cell::new(end_bound.x, center_cell.y);
                let top = Cell::new(center_cell.x, start_bound.y);
                let left = Cell::new(start_bound.x, center_cell.y);
                let bottom = Cell::new(center_cell.x, end_bound.y);

                let top_left = Cell::new(left.x, top.y);
                let top_right = Cell::new(right.x, top.y);
                let bottom_left = Cell::new(left.x, bottom.y);
                let bottom_right = Cell::new(right.x, bottom.y);
                assert_eq!(top_left, start_bound);
                assert_eq!(bottom_right, end_bound);


                // include cx if the center lies on the horizontal midline
                // include cy if the center lies on the veritcal midline
                let cx_adjusted = if !center.is_edge_x(){ center_cell.x }else{ center_cell.x - 1 };
                let cy_adjusted = if !center.is_edge_y(){ center_cell.y }else{ center_cell.y - 1 };

                let arc1_bounds = Cell::rearrange_bound(Cell::new(center_cell.x, cy_adjusted), top_right);
                let arc2_bounds = Cell::rearrange_bound(top_left, Cell::new( cx_adjusted, cy_adjusted));
                let arc3_bounds = Cell::rearrange_bound(bottom_left, Cell::new(cx_adjusted, center_cell.y));
                let arc4_bounds = Cell::rearrange_bound(center_cell, bottom_right);

                let arc1_span = span.extract(arc1_bounds.0, arc1_bounds.1);
                let arc2_span = span.extract(arc2_bounds.0, arc2_bounds.1);
                let arc3_span = span.extract(arc3_bounds.0, arc3_bounds.1);
                let arc4_span = span.extract(arc4_bounds.0, arc4_bounds.1);

                let p1 = Point::new(center.x + radius, center.y);
                let p2 = Point::new(center.x, center.y - radius);
                let p3 = Point::new(center.x - radius, center.y);
                let p4 = Point::new(center.x, center.y + radius);

                let arc1_start  = arc1_bounds.0.localize_point(p1);
                let arc1_end = arc1_bounds.0.localize_point(p2);

                let arc2_start = arc2_bounds.0.localize_point(p2);
                let arc2_end = arc2_bounds.0.localize_point(p3);

                let arc3_start = arc3_bounds.0.localize_point(p3);
                let arc3_end = arc3_bounds.0.localize_point(p4);

                let arc4_start = arc4_bounds.0.localize_point(p4);
                let arc4_end = arc4_bounds.0.localize_point(p1);

                let arc1  = fragment::Arc::new(arc1_start, arc1_end, *radius);
                let arc2  = fragment::Arc::new(arc2_start, arc2_end, *radius);
                let arc3  = fragment::Arc::new(arc3_start, arc3_end, *radius);
                let arc4  = fragment::Arc::new(arc4_start, arc4_end, *radius);

                //TODO: get the settings supplied by the user
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
fn is_subset_of(
    subset: &Vec<Contacts>,
    big_set: &Vec<Contacts>,
) -> (bool, Vec<usize>) {
    let mut unmatched = vec![];
    let mut matched = 0;
    for (_i, set) in subset.iter().enumerate() {
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
pub fn endorse_arc(
    search: &Vec<Contacts>,
) -> Option<(&fragment::Arc, Vec<usize>)> {
    FRAGMENTS_ARC.iter().rev().find_map(|(contacts, arc)| {
        let (matched, unmatched) = is_subset_of(contacts, search);
        if matched {
            Some((arc, unmatched))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_circles() {
        let len = DIAMETER_CIRCLE.len();
        println!("len: {}", len);
        assert_eq!(len, 22);
        let frag_len = FRAGMENTS_ARC.len();
        println!("frag len: {}", frag_len);
        assert_eq!(frag_len, 76);
    }

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
        let (arc, _) = endorse_arc(&groups).unwrap();
        assert_eq!(arc.radius, 5.0);
    }

    #[test]
    fn test_arc5_top_right() {
        let art = r#"
            -.
              )
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        let (arc, _) = endorse_arc(&groups).unwrap();
        assert_eq!(arc.radius, 2.5);
    }

    #[test]
    fn test_arc5_top_left() {
        let art = r#"
           .-
          (
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        let (arc, _) = endorse_arc(&groups).unwrap();
        assert_eq!(arc.radius, 2.5);
    }

    #[test]
    fn test_arc5_bottom_left() {
        let art = r#"
          (
           `-
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        let (arc, _) = endorse_arc(&groups).unwrap();
        assert_eq!(arc.radius, 2.5);
    }

    #[test]
    fn test_arc5_bottom_right() {
        let art = r#"
              )
            -'
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        let (arc, _) = endorse_arc(&groups).unwrap();
        assert_eq!(arc.radius, 2.5);
    }

    #[test]
    fn test_arc20_top_right() {
        let art = r#"
            ''''-._
                   `.
                     \
                      .
                      |
                      |
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        let (arc, _) = endorse_arc(&groups).unwrap();
        assert_eq!(arc.radius, 10.5); //also matched the arc21 radius and since larger it will matched it instead of arc20
    }

    #[test]
    fn test_arc20_top_left() {
        let art = r#"
                    _.-''''
                  ,'
                 /
                .
                |
                |
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        let (arc, _) = endorse_arc(&groups).unwrap();
        assert_eq!(arc.radius, 10.5); //also matched the arc21 radius and since larger it will matched it instead of arc20
    }

    #[test]
    fn test_arc20_bottom_left() {
        let art = r#"
                |
                |
                 \
                  `._
                     '-....
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        let (arc, _) = endorse_arc(&groups).unwrap();
        assert_eq!(arc.radius, 10.0);
    }

    #[test]
    fn test_arc20_bottom_right() {
        let art = r#"
                          |
                          |
                         /
                      _.'
                ....-'
            "#;
        let cell_buffer = CellBuffer::from(art);
        let mut spans: Vec<Span> = cell_buffer.group_adjacents();
        assert_eq!(spans.len(), 1);
        let span1 = spans.remove(0);
        let groups = span1.get_contacts(&Settings::default());
        for (i, group) in groups.iter().enumerate() {
            println!("group{}\n{}", i, group);
        }
        let (arc, _) = endorse_arc(&groups).unwrap();
        assert_eq!(arc.radius, 10.0);
    }
}
