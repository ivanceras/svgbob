use super::*;
use crate::{
    buffer::{
        fragment::{Arc, Circle, Rect},
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
    let groups: Vec<Contacts> =
        span.localize().get_contacts(&Settings::default());
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
    let groups1: Vec<Contacts> = span1.localize().get_contacts(settings);
    let groups2: Vec<Contacts> = span2.localize().get_contacts(settings);
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
    let groups: Vec<Contacts> =
        span1.localize().get_contacts(&Settings::default());
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

    let circle = fragments.remove(0);
    assert!(circle.is_circle());
    assert_eq!(
        circle,
        Fragment::Circle(Circle::new(Point::new(11.0, 5.0), 2.5, false))
    );

    let rect = fragments.remove(0);
    dbg!(&rect);
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

    let groups1 = span1.localize().get_contacts(&Settings::default());
    let since = groups1[11].as_ref()[0].as_cell_text().unwrap();
    assert_eq!(since.content, "since");
    assert_eq!(since.start, Cell::new(20, 1));
    let abs_since = since.absolute_position(top_left1);
    assert_eq!(abs_since.content, "since");
    assert_eq!(abs_since.start, Cell::new(28, 6));
}
#[test]
fn test_endorse_arc() {
    let art = r#"
         .-
        (
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
    let arc = fragments.remove(0);
    assert_eq!(
        arc,
        Fragment::Arc(Arc::new(
            Point::new(11.0, 2.5),
            Point::new(8.5, 5.0),
            2.5
        ))
    );
}
