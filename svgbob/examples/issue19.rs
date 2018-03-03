extern crate svg;
extern crate svgbob;

use svgbob::Grid;
use svgbob::Settings;

fn main() {
    let file = "examples/issue19.svg";
    let arg = r#"

Issue 19 demo test  -------

    +-----------+---+
    | "a.to(b)" | c |
    +-----------+---+

    +-----------+---+
    | "a.to(b)" | c |""
    +-----------+---+
    "#;
    let g = Grid::from_str(arg, &Settings::compact());
    let svg = g.get_svg();
    svg::save(file, &svg).unwrap();
    println!("Saved to {}", file);
}
