//! generate an SVG from the ascii text using the default settings
//!
//! ```
//! let input = r#"
//! .-------------------------------------.
//! | Hello here and there and everywhere |
//! '-------------------------------------'
//! "#;
//! println!("svg: {}",svgbob::to_svg(input));
//! ```
//!
//! <svg font-family="Electrolize,Titillium Web, Trebuchet MS, Arial" font-size="14" height="80" width="344" xmlns="http://www.w3.org/2000/svg">
//! <defs>
//! <marker id="triangle" markerHeight="10" markerUnits="strokeWidth" markerWidth="10" orient="auto" refX="0" refY="5" viewBox="0 0 14 14">
//! <path d="M 0 0 L 10 5 L 0 10 z"/>
//! </marker>
//! </defs>
//! <style>
//!     line, path {
//!       stroke: black;
//!       stroke-width: 1;
//!     }
//! </style>
//! <path d=" M 36 28 L 36 48 M 40 24 A 4 4 0 0 0 36 28 M 40 24 L 336 24 M 340 28 L 340 48 M 340 28 A 4 4 0 0 0 336 24 M 36 32 L 36 48 M 340 32 L 340 48 M 36 48 L 36 52 A 4 4 0 0 0 40 56 L 336 56 M 340 48 L 340 52 M 336 56 A 4 4 0 0 0 340 52" fill="none"/>
//! <path d="" fill="none" stroke-dasharray="3 3"/>
//! <text x="50" y="44">
//! Hello here and there and everywhere
//! </text>
//! </svg>
//!
//!
#![deny(warnings)]
#![feature(extern_prelude)]
extern crate pom;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate svg;
extern crate unicode_width;

pub use grid::Grid;
pub use settings::Settings;
use svg::node::element::SVG;

mod optimizer;
mod box_drawing;
mod fragments;
mod properties;
mod settings;
mod svg_element;
mod element;
mod grid;
mod point;
mod location;
mod loc;
mod point_block;
mod block;
mod focus_char;
mod loc_block;

/// generate an SVG from the ascii text input
///
/// Usage:
///
/// ```
/// let input = "------->";
/// println!("svg: {}", svgbob::to_svg(input));
/// ```
///
/// commercial version enhances memes automatically
pub fn to_svg(input: &str) -> SVG {
    Grid::from_str(&input, &Settings::default()).get_svg()
}











