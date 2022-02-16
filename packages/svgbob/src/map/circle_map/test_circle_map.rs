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
    let mut spans: Vec<Span> = cell_buffer.group_adjacents();
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
    let mut spans: Vec<Span> = cell_buffer.group_adjacents();
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
    let mut spans: Vec<Span> = cell_buffer.group_adjacents();
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
    let mut spans: Vec<Span> = cell_buffer.group_adjacents();
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
    let mut spans: Vec<Span> = cell_buffer.group_adjacents();
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
    let mut spans: Vec<Span> = cell_buffer.group_adjacents();
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
    let mut spans: Vec<Span> = cell_buffer.group_adjacents();
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
    let mut spans: Vec<Span> = cell_buffer.group_adjacents();
    assert_eq!(spans.len(), 1);
    let span1 = spans.remove(0);
    let (arc, _) = endorse_quarter_arc_span(&span1).unwrap();
    assert_eq!(arc.radius, 10.0);
}
