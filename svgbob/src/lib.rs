#![deny(clippy::all)]

pub mod buffer;
mod map;
mod point;
pub mod util;

pub use buffer::{
    fragment, fragment::Fragment, Cell, CellBuffer, Direction, FragmentBuffer, Property, Settings,
    Signal,
};
pub use point::Point;
pub use sauron::{Node, Render};

/// convert svgbob ascii art to svg
pub fn to_svg(ascii: &str) -> String {
    let cb = CellBuffer::from(ascii);
    let node: Node<()> = cb.get_node();
    let mut buffer = String::new();
    node.render(&mut buffer).expect("must render");
    buffer
}

/// convert ascii art into an svg
pub fn to_svg_with_settings(ascii: &str, settings: &Settings) -> String {
    let cb = CellBuffer::from(ascii);
    let (node, _w, _h): (Node<()>, f32, f32) = cb.get_node_with_size(settings);
    let mut buffer = String::new();
    node.render(&mut buffer).expect("must render");
    buffer
}
