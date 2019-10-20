extern crate svg;
extern crate svgbob;

use std::time::Instant;
use svgbob::Grid;
use svgbob::Settings;

fn main() {
    let file = "examples/demo.svg";
    let t1 = Instant::now();
    let arg = include_str!("demo.bob");
    let g = Grid::from_str(arg, &Settings::default());
    let svg = g.get_svg();
    println!("took {}ms", t1.elapsed().as_millis());
    svg::save(file, &svg).unwrap();
    println!("Saved to {}", file);
}
