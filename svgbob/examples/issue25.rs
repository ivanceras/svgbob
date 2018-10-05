extern crate svg;
extern crate svgbob;

use svgbob::Grid;
use svgbob::Settings;

fn main() {
    let file = "examples/issue25.svg";
    let arg = r#"
------------.
             \ .-------------
              X
             / '------------>
------------'

  --. .---
     X
  --' '--

    "#;
    let g = Grid::from_str(arg, &Settings::default());
    let svg = g.get_svg();
    svg::save(file, &svg).unwrap();
    println!("Saved to {}", file);
}
