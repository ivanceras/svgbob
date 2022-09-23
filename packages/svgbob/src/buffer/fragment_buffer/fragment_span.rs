use crate::buffer::Span;
use crate::Cell;
use crate::Fragment;
use crate::Merge;
use crate::Settings;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentSpan {
    pub span: Span,
    pub fragment: Fragment,
}

impl fmt::Display for FragmentSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.fragment)
    }
}

impl FragmentSpan {
    pub fn new(span: Span, fragment: Fragment) -> Self {
        Self { span, fragment }
    }

    pub fn cells(&self) -> Vec<Cell> {
        self.span.0.iter().map(|(cell, _ch)| *cell).collect()
    }

    pub fn scale(&self, scale: f32) -> Self {
        Self {
            span: self.span.clone(),
            fragment: self.fragment.scale(scale),
        }
    }

    pub(crate) fn is_contacting(&self, other: &Self) -> bool {
        self.fragment.is_contacting(&other.fragment)
    }

    pub fn absolute_position(&self, cell: Cell) -> Self {
        Self {
            span: self.span.clone(),
            fragment: self.fragment.absolute_position(cell),
        }
    }

    pub fn is_bounded(&self, bound1: Cell, bound2: Cell) -> bool {
        self.span.is_bounded(bound1, bound2)
    }

    pub fn hit_cell(&self, needle: Cell) -> bool {
        self.span.hit_cell(needle)
    }
}

impl Merge for FragmentSpan {
    fn merge(&self, other: &Self) -> Option<Self> {
        if let Some(new_merge) = self.fragment.merge(&other.fragment) {
            let new_span =
                self.span.merge(&other.span).expect("must merge the spans");
            Some(Self {
                span: new_span,
                fragment: new_merge,
            })
        } else {
            None
        }
    }
}
