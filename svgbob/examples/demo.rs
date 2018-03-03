extern crate svg;
extern crate svgbob;

use svgbob::Grid;
use svgbob::Settings;

fn main() {
    let file = "examples/demo.svg";
    let arg = include_str!("demo.bob");
    let g = Grid::from_str(arg, &Settings::compact());
    let svg = g.get_svg();
    svg::save(file, &svg).unwrap();
    println!("Saved to {}", file);
}
