extern crate svg;
extern crate svgbob;

use svgbob::Grid;
use svgbob::Settings;

fn main() {
    let file = "examples/long.svg";
    let arg = include_str!("long.bob");
    let g = Grid::from_str(arg, &Settings::default());
    let svg = g.get_svg();
    svg::save(file, &svg).unwrap();
    println!("Saved to {}", file);
}
