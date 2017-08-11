extern crate svgbob;



#[test]
fn test_dash_alone(){
    use svgbob::{Grid,Settings};

    let g = Grid::from_str("-", &Settings::separate_lines());
    println!("grid: {:?}", g);
    let result = g.get_svg_nodes_only();
    println!("grid: {:?}", g.to_string());
    let expected = r#"<text x="1" y="12">
-
</text>"#;
    assert_eq!(result, expected);
}
#[test]
fn test_vert_alone(){
    use svgbob::{Grid,Settings};

    let g = Grid::from_str("|", &Settings::separate_lines());
    println!("grid: {:?}", g);
    let result = g.get_svg_nodes_only();
    println!("grid: {:?}", g.to_string());
    let expected = r#"<text x="1" y="12">
|
</text>"#;
    assert_eq!(result, expected);
}
#[test]
fn test_dash_line2(){
    use svgbob::{Grid,Settings};

    let g = Grid::from_str("--", &Settings::separate_lines());
    let result = g.get_svg_nodes_only();
    let expected = r#"<line x1="0" x2="16" y1="8" y2="8"/>"#;
    assert_eq!(result, expected);
}

#[test]
fn test_vert_line2(){
    use svgbob::{Grid,Settings};

    let g = Grid::from_str("
|
|
", &Settings::separate_lines());
    let result = g.get_svg_nodes_only();
    let expected = r#"<line x1="4" x2="4" y1="16" y2="48"/>"#;
    assert_eq!(result, expected);
}

#[test]
fn test_dash_cross(){
    use svgbob::{Grid,Settings};
    let g = Grid::from_str("-+-", &Settings::separate_lines());
    let result = g.get_svg_nodes_only();
    let expected = r#"<line x1="0" x2="24" y1="8" y2="8"/>"#;
    assert_eq!(result, expected);
}

#[test]
fn test_dot_dash(){
    use svgbob::{Grid,Settings};
    let g = Grid::from_str(".-", &Settings::separate_lines());
    let result = g.get_svg_nodes_only();
    let expected = r#"<line x1="0" x2="24" y1="8" y2="8"/>"#;
    assert_eq!(result, expected);
}

