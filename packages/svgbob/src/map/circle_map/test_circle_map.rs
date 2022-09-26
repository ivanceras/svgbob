use super::*;

//#[test]
fn show_circles() {
    println!("CIRCLES ");
    for (circle, span) in CIRCLES_SPAN.iter() {
        println!("diameter: {}", circle.radius * 2.0);
        println!();
        println!("{}", span);
        println!();
        println!();
    }
    println!("QUARTER ARCS:");
    for (diameter, (arc, span)) in FLATTENED_QUARTER_ARC_SPAN.iter() {
        println!("diameter: {}", diameter.diameter);
        println!();
        println!("{}", span);
        println!();
        println!();
    }
    println!("HALF ARCS:");
    for (diameter, (arc, span)) in FLATTENED_HALF_ARC_SPAN.iter() {
        println!("diameter: {}", diameter.diameter);
        println!();
        println!("{}", span);
        println!();
        println!();
    }
    panic!();
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
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let groups: Vec<Contacts> = span1.into();
    for (i, group) in groups.iter().enumerate() {
        println!("group{}\n{}", i, group);
    }
    assert_eq!(11, groups.len());
}

#[test]
fn test_half_arc_span5_top() {
    let art = r#"
           ___
         ,'   `.
        /       \

            "#;
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_half_arc_span(&span1).unwrap();
    assert_eq!(arc.radius, 4.5);
}

#[test]
fn test_half_arc_span5_bottom() {
    let art = r#"
            \       /
             `.___.'

            "#;
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_half_arc_span(&span1).unwrap();
    assert_eq!(arc.radius, 4.5);
}

#[test]
fn test_half_arc_span5_left() {
    let art = r#"
    __
  ,'
 /
 \
  `.__
            "#;
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let result = endorse_half_arc_span(&span1);
    assert!(result.is_some());
    let (arc, _) = result.unwrap();
    assert_eq!(arc.radius, 5.0);
}

#[test]
fn test_half_arc_span5_right() {
    let art = r#"
         __
           `.
             \
             /
         __.'

            "#;
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let result = endorse_half_arc_span(&span1);
    assert!(result.is_some());
    let (arc, _) = result.unwrap();
    assert_eq!(arc.radius, 5.0);
}

#[test]
fn test_arc9_top_right() {
    let art = r#"
            __
              `.
                \
            "#;
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
    assert_eq!(arc.radius, 5.0);
}

#[test]
fn test_arc5_top_right() {
    let art = r#"
            -.
              )
            "#;
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
    assert_eq!(arc.radius, 2.5);
}

#[test]
fn test_arc5_top_left() {
    let art = r#"
           .-
          (
            "#;
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
    assert_eq!(arc.radius, 2.5);
}

#[test]
fn test_arc5_bottom_left() {
    let art = r#"
          (
           `-
            "#;
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
    assert_eq!(arc.radius, 2.5);
}

#[test]
fn test_arc5_bottom_right() {
    let art = r#"
              )
            -'
            "#;
    let cell_buffer = CellBuffer::from(art);
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
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
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
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
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
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
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
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
    let mut spans: Vec<Span> = cell_buffer.into();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
    assert_eq!(arc.radius, 10.0);
}
