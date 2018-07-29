use std;
use svg::node::element::Circle as SvgCircle;
use svg::node::element::Definitions;
use svg::node::element::Line as SvgLine;
use svg::node::element::Marker;
use svg::node::element::Path as SvgPath;
use svg::node::element::Rectangle as SvgRect;
use svg::node::element::Style;
use svg::node::element::Text as SvgText;
use svg::node::element::SVG;

pub enum SvgElement {
    Circle(SvgCircle),
    Line(SvgLine),
    Path(SvgPath),
    Text(SvgText),
}

impl std::fmt::Debug for SvgElement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            SvgElement::Circle(ref c) => writeln!(fmt, "{}", c.to_string()),
            SvgElement::Line(ref l) => writeln!(fmt, "{}", l.to_string()),
            SvgElement::Path(ref p) => writeln!(fmt, "{}", p.to_string()),
            SvgElement::Text(ref t) => writeln!(fmt, "{}", t.to_string()),
        }
    }
}
