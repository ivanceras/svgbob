#![deny(warnings)]
#![allow(unused)]
#![deny(clippy::all)]

pub mod buffer;
pub mod map;
mod merge;
mod point;
mod settings;
pub mod util;

pub use buffer::{
    fragment, fragment::Fragment, Cell, CellBuffer, Direction, FragmentBuffer,
    FragmentSpan, Property, Signal,
};
pub use merge::Merge;
pub use nalgebra;
pub use point::Point;
/// reexport sauron
pub use sauron;
pub use sauron::Node;
pub use settings::Settings;

pub fn to_svg(ascii: &str) -> String {
    to_svg_string_pretty(ascii)
}

/// convert svgbob ascii art to svg string with indentions
pub fn to_svg_string_pretty(ascii: &str) -> String {
    let cb = CellBuffer::from(ascii);
    let node: Node<()> = cb.get_node();
    let mut buffer = String::new();
    node.render(&mut buffer).expect("must render");
    buffer
}

/// convert svgbob ascii art to svg string
pub fn to_svg_string_compressed(ascii: &str) -> String {
    let cb = CellBuffer::from(ascii);
    let node: Node<()> = cb.get_node();
    node.render_to_string()
}

/// convert ascii art into an svg
pub fn to_svg_with_settings(ascii: &str, settings: &Settings) -> String {
    let cb = CellBuffer::from(ascii);
    let (node, _w, _h): (Node<()>, f32, f32) = cb.get_node_with_size(settings);
    let mut buffer = String::new();
    node.render(&mut buffer).expect("must render");
    buffer
}

/// convert ascii art to svg using the size supplied
pub fn to_svg_with_override_size(
    ascii: &str,
    settings: &Settings,
    w: f32,
    h: f32,
) -> String {
    let cb = CellBuffer::from(ascii);
    let node: Node<()> = cb.get_node_override_size(settings, w, h);
    let mut buffer = String::new();
    node.render(&mut buffer).expect("must render");
    buffer
}
