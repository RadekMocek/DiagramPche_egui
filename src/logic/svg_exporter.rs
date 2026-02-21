use svg::Node;

// == Helper functions and values ==
pub fn egui_color32_to_svg_rgb(c: egui::Color32) -> String {
    let s = c.to_srgba_unmultiplied();
    format!("rgb({}, {}, {})", s[0], s[1], s[2])
}

pub fn egui_vec2_to_svg_point(v: egui::Vec2) -> String {
    format!("{},{}", v.x, v.y)
}

pub const SVG_PADDING: f32 = 25.0;
pub const SVG_PADDING_VEC: egui::Vec2 = egui::Vec2::new(SVG_PADDING, SVG_PADDING);

// == Exporter struct ==
pub struct Exporter {
    // Boundaries take origin/scrolling and zoom_level into account, it will be "corrected" just before creating the svg,
    // so we don't have to subtract and divide with each `update_boundaries` call.
    boundaries_min: (f32, f32),
    boundaries_max: (f32, f32),
    //
    pub offset: egui::Vec2,
    //
    pub svg_document: svg::Document,
}

impl Default for Exporter {
    fn default() -> Self {
        Self {
            boundaries_min: (f32::MAX, f32::MAX),
            boundaries_max: (f32::MIN, f32::MIN),
            offset: egui::Vec2::new(0.0, 0.0),
            svg_document: svg::Document::new(),
        }
    }
}

impl Exporter {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn update_boundaries(&mut self, x_min: f32, y_min: f32, x_max: f32, y_max: f32) {
        if x_min < self.boundaries_min.0 {
            self.boundaries_min.0 = x_min;
        }
        if y_min < self.boundaries_min.1 {
            self.boundaries_min.1 = y_min;
        }
        if x_max > self.boundaries_max.0 {
            self.boundaries_max.0 = x_max;
        }
        if y_max > self.boundaries_max.1 {
            self.boundaries_max.1 = y_max;
        }
    }

    pub fn apply_boundaries(&mut self, origin_x: f32, origin_y: f32, zoom_level: f32) {
        let x_min = (self.boundaries_min.0 - origin_x) / zoom_level;
        let x_max = (self.boundaries_max.0 - origin_x) / zoom_level;
        let y_min = (self.boundaries_min.1 - origin_y) / zoom_level;
        let y_max = (self.boundaries_max.1 - origin_y) / zoom_level;
        self.svg_document
            .assign("width", x_max - x_min + 2.0 * SVG_PADDING);
        self.svg_document
            .assign("height", y_max - y_min + 2.0 * SVG_PADDING);
        self.offset = egui::Vec2::new(x_min, y_min);
    }

    pub fn save(&mut self) {
        let result = svg::save("diagram.svg", &self.svg_document);

        if let Err(err) = result {
            println!("{err}");
        } else {
            println!("svg ok");
        }
    }
}
