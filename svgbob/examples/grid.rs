
extern crate svgbob;

use svgbob::Grid;
use svgbob::Settings;

fn main(){
    let g = Grid::from_str("a统öo͡͡͡", &Settings::compact());
}
