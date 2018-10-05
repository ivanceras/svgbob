extern crate svg;
extern crate svgbob;

use svgbob::Grid;
use svgbob::Settings;

fn main() {
    let file = "examples/spec1.svg";
    let g = Grid::from_str(get_arg(), &Settings::default());
    let svg = g.get_svg();
    svg::save(file, &svg).unwrap();
    println!("Saved to {}", file);
}

fn get_arg() -> &'static str {
    r#"-+-"#
}
