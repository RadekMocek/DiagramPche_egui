use svg::Node;

pub const SVG_PADDING: f32 = 25.0;
const SVG_FONT_SIZE: u32 = 18;

// == Helper functions and values ==
pub fn egui_color32_to_svg_rgb(c: egui::Color32) -> String {
    let s = c.to_srgba_unmultiplied();
    format!("rgb({}, {}, {})", s[0], s[1], s[2])
}

pub fn egui_pos2_to_svg_point(v: egui::Pos2) -> String {
    format!("{},{}", v.x, v.y)
}

pub fn add_text_to_svg_document(
    document: &mut svg::Document,
    label_position: egui::Pos2,
    offset: egui::Vec2,
    label_galley: std::sync::Arc<egui::Galley>,
) {
    let label_x = label_position.x - offset.x;
    let mut label_y = label_position.y - offset.y;

    label_y += (SVG_FONT_SIZE * 5 / 6) as f32; // Magic

    for line in label_galley.job.text.lines() {
        document.append(
            svg::node::element::Text::new(line)
                .set("x", label_x)
                .set("y", label_y)
                .set("font-size", SVG_FONT_SIZE)
                .set("font-family", "Inconsolata"),
        );
        label_y += SVG_FONT_SIZE as f32;
    }
}

// == Exporter struct ==
pub struct Exporter {
    // Boundaries take origin/scrolling and zoom_level into account, it will be "corrected" just before creating the svg,
    // so we don't have to subtract and divide with each `update_boundaries` call.
    boundaries_min: (f32, f32),
    boundaries_max: (f32, f32),
    // Offset from the SVG origin
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

    pub fn apply_boundaries(&mut self) {
        let x_min = self.boundaries_min.0;
        let x_max = self.boundaries_max.0;
        let y_min = self.boundaries_min.1;
        let y_max = self.boundaries_max.1;
        let width = x_max - x_min;
        let height = y_max - y_min;
        self.svg_document.assign("width", width);
        self.svg_document.assign("height", height);

        self.svg_document.assign(
            "viewBox",
            format!(
                "{} {} {} {}",
                -SVG_PADDING,
                -SVG_PADDING,
                width + 2.0 * SVG_PADDING,
                height + 2.0 * SVG_PADDING
            ),
        );

        self.offset = egui::Vec2::new(x_min, y_min);
    }

    pub fn save(&mut self, location: &str) -> std::io::Result<()> {
        svg::save(location, &self.svg_document)
    }
}
