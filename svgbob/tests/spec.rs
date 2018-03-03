extern crate svgbob;

#[test]
fn test_dash_alone() {
    use svgbob::{Grid, Settings};

    let g = Grid::from_str("-", &Settings::separate_lines());
    println!("grid: {:?}", g);
    let result = g.get_svg_nodes_only();
    println!("grid: {:?}", g.to_string());
    let expected =
        "<line x1=\"0\" x2=\"8\" y1=\"8\" y2=\"8\"/><line x1=\"0\" x2=\"8\" y1=\"8\" y2=\"8\"/>";
    assert_eq!(result, expected);
}
