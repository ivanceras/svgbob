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
    /// flag whether to include the big rectangle as backdrop
    /// for all of the svg shapes
    pub include_backdrop: bool,
    /// flag whether to include the svg styles and legen css styles
    pub include_styles: bool,
    /// flag whether to include the def of markers, etc in the svg
    pub include_defs: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            font_size: 14,
            font_family: "Iosevka Fixed, monospace".into(),
            fill_color: "black".into(),
            background: "white".into(),
            stroke_color: "black".into(),
            stroke_width: 2.0,
            scale: 8.0,
            include_backdrop: true,
            include_styles: true,
            include_defs: true,
        }
    }
}
