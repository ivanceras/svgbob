#[derive(Debug, Clone)]
pub struct Settings {
    /// font-size of the text
    pub font_size: usize,
    /// the font used for the text
    pub font_family: String,
    /// the color fill used in filled solid shape
    pub fill_color: String,
    /// the backdrop background color
    pub background: String,
    /// the stroke color of lines, and shapes
    pub stroke_color: String,
    /// the width of the stroke
    pub stroke_width: f32,
    /// the scale multiplier
    pub scale: f32,
    /// flag whether to enhace circuitries or not, default true
    pub enhance_circuitries: bool,
    /// flag whether to include the big rectangle as backdrop
    /// for all of the svg shapes
    pub include_backdrop: bool,
    /// flag whether to include the svg styles and legen css styles
    pub include_styles: bool,
    /// flag whether to include the def of markers, etc in the svg
    pub include_defs: bool,
    /// merge lines and shapes like triangle,circle,rect to form arrow lines or marker line
    pub merge_line_with_shapes: bool,
}
impl Settings {
    /// the inverse of the default scale 10
    pub fn scale_inverse(&self) -> f32 {
        1.0 / self.scale
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            font_size: 14,
            font_family: "monospace".into(),
            fill_color: "black".into(),
            background: "white".into(),
            stroke_color: "black".into(),
            stroke_width: 2.0,
            scale: 8.0,
            enhance_circuitries: true,
            include_backdrop: true,
            include_styles: true,
            include_defs: true,
            merge_line_with_shapes: false,
        }
    }
}
