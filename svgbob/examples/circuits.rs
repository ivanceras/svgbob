
extern crate svgbob;
extern crate svg;

use svgbob::Grid;
use svgbob::Settings;


fn main() {
    let file = "examples/circuits.svg";
    let arg = include_str!("circuits.bob");
    let g = Grid::from_str(arg, &Settings::compact());
    let svg = g.get_svg();
    svg::save(file, &svg).unwrap();
    println!("Saved to {}",file);
}

