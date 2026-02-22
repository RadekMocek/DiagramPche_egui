use crate::logic::svg_exporter::egui_color32_to_svg_rgb;
use crate::model::draw_command::command::DrawCommand;
use egui::{Color32, Galley, Painter, Pos2, Vec2};
use std::sync::Arc;
use svg::{Document, Node};

pub struct NodeRectangleDrawCommand {
    rect_top_left: Pos2,
    rect_bottom_right: Pos2,
    rect_color: Color32,
    zoom_level: f32,
    label_position: Pos2,
    label_galley: Arc<Galley>,
}

impl NodeRectangleDrawCommand {
    pub fn new(
        rect_top_left: Pos2,
        rect_bottom_right: Pos2,
        rect_color: Color32,
        zoom_level: f32,
        label_position: Pos2,
        label_galley: Arc<Galley>,
    ) -> Self {
        Self {
            rect_top_left,
            rect_bottom_right,
            rect_color,
            zoom_level,
            label_position,
            label_galley,
        }
    }
}

impl DrawCommand for NodeRectangleDrawCommand {
    fn draw(&self, painter: &Painter) {
        painter.rect(
            egui::Rect::from_min_max(self.rect_top_left, self.rect_bottom_right),
            0,
            self.rect_color,
            egui::Stroke::new(self.zoom_level, Color32::BLACK),
            egui::StrokeKind::Inside,
        );

        painter.galley(
            self.label_position,
            self.label_galley.clone(),
            Color32::BLACK,
        );
    }

    fn draw_svg(&self, document: &mut Document, origin: Pos2, offset: Vec2) {
        // == SVG rectangle ==
        let top_left = ((self.rect_top_left - origin) / self.zoom_level) - offset;
        let bottom_right = ((self.rect_bottom_right - origin) / self.zoom_level) - offset;
        let width = bottom_right.x - top_left.x;
        let height = bottom_right.y - top_left.y;

        document.append(
            svg::node::element::Rectangle::new()
                .set("x", top_left.x)
                .set("y", top_left.y)
                .set("width", width)
                .set("height", height)
                .set("fill", egui_color32_to_svg_rgb(self.rect_color))
                .set("stroke", "rgb(0,0,0)")
                .set("stroke-width", "1")
                .set(
                    "style",
                    format!("fill-opacity:{}", self.rect_color.a() as f32 / 255.0),
                ),
        );

        // == SVG text ==
        const FONT_SIZE: u32 = 18;
        let label_x = ((self.label_position.x - origin.x) / self.zoom_level) - offset.x;
        let mut label_y = ((self.label_position.y - origin.y) / self.zoom_level) - offset.y;

        label_y += (FONT_SIZE * 5 / 6) as f32; // Magic

        for line in self.label_galley.job.text.lines() {
            document.append(
                svg::node::element::Text::new(line)
                    .set("x", label_x)
                    .set("y", label_y)
                    .set("font-size", FONT_SIZE)
                    .set("font-family", "Inconsolata"),
            );
            label_y += FONT_SIZE as f32;
        }
    }
}
