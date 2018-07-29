
///  optimization options:
///  1. None -> Fastest, but not correct looking (paths and text are not reduced)
///  2. Fast -> Fast and correct looking (text are reduced)
///  3. All -> Correct looking but slow (paths and text are reduced)
#[derive(Debug, Clone)]
pub struct Settings {
    pub text_width: f32,
    pub text_height: f32,
    /// do optimization? if false then every piece are disconnected
    pub optimize: bool,
    /// if optmization is enabled,
    /// true means all reduceable paths will be in 1 path definition
    pub compact_path: bool,
    /// the svg class of the generated svg
    pub class: Option<String>,
    /// the id of the generated svg
    pub id: Option<String>,
    /// the font family used for text (default: arial)
    pub font_family: String,
    /// the font size used for text (default: 14)
    pub font_size: f32,
    /// stroke width for all lines (default: 2.0)
    pub stroke_width: f32,
    /// stroke color, default black
    pub stroke_color: String,
    /// background color: default white
    pub background_color: String
}

impl Settings {
    pub fn set_size(&mut self, text_width: f32, text_height: f32) {
        self.text_width = text_width;
        self.text_height = text_height;
    }

    pub fn scale(&mut self, scale: f32) {
        self.text_width = self.text_width * scale;
        self.text_height = self.text_height * scale;
        self.font_size = self.font_size * scale;
        self.stroke_width = self.stroke_width * scale;
    }

    pub fn no_optimization() -> Settings {
        let mut settings = Settings::default();
        settings.optimize = false;
        settings.compact_path = false;
        settings
    }

    pub fn separate_lines() -> Settings {
        let mut settings = Settings::default();
        settings.optimize = true;
        settings.compact_path = false;
        settings
    }

    pub fn compact() -> Settings {
        let mut settings = Settings::default();
        settings.optimize = true;
        settings.compact_path = true;
        settings
    }

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn set_class(&mut self, class: String) {
        self.class = Some(class);
    }

    pub fn set_selector(&mut self, id: Option<String>, class: Option<String>) {
        if let Some(id) = id {
            self.set_id(id);
        }
        if let Some(class) = class {
            self.set_class(class);
        }
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            text_width: 8.0,
            text_height: 16.0,
            optimize: true,
            compact_path: true,
            class: Some("bob".to_string()),
            id: None,
            font_family: "arial".to_string(),
            font_size: 14.0,
            stroke_width: 2.0,
            stroke_color: "black".into(),
            background_color: "white".into(),
        }
    }
}
