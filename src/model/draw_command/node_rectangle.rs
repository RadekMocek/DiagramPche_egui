use crate::logic::svg_exporter::{add_text_to_svg_document, egui_color32_to_svg_rgb};
use crate::model::draw_command::command::DrawCommand;
use egui::{Color32, Galley, Painter, Pos2, Rect, Stroke, StrokeKind, Vec2};
use std::sync::Arc;
use svg::{Document, Node};

pub struct NodeRectangleDrawCommand {
    top_left: Pos2,
    bottom_right: Pos2,
    color_text: Color32,
    color_fill: Color32,
    color_edge: Color32,
    zoom_level: f32,
    label_position: Pos2,
    label_galley: Arc<Galley>,
}

impl NodeRectangleDrawCommand {
    pub fn new(
        top_left: Pos2,
        bottom_right: Pos2,
        color_text: Color32,
        color_fill: Color32,
        color_edge: Color32,
        zoom_level: f32,
        label_position: Pos2,
        label_galley: Arc<Galley>,
    ) -> Self {
        Self {
            top_left,
            bottom_right,
            color_text,
            color_fill,
            color_edge,
            zoom_level,
            label_position,
            label_galley,
        }
    }
}

impl DrawCommand for NodeRectangleDrawCommand {
    fn draw(&self, painter: &Painter) {
        painter.rect(
            Rect::from_min_max(self.top_left, self.bottom_right),
            0,
            self.color_fill,
            Stroke::new(self.zoom_level, self.color_edge),
            StrokeKind::Inside,
        );

        painter.galley(
            self.label_position,
            self.label_galley.clone(),
            self.color_text,
        );
    }

    fn draw_svg(&self, document: &mut Document, offset: Vec2) {
        // == SVG rectangle ==
        let top_left = self.top_left - offset;
        let bottom_right = self.bottom_right - offset;
        let width = bottom_right.x - top_left.x;
        let height = bottom_right.y - top_left.y;

        document.append(
            svg::node::element::Rectangle::new()
                .set("x", top_left.x)
                .set("y", top_left.y)
                .set("width", width)
                .set("height", height)
                .set("fill", egui_color32_to_svg_rgb(self.color_fill))
                .set("stroke", egui_color32_to_svg_rgb(self.color_edge))
                .set("stroke-width", "1")
                .set(
                    "style",
                    format!(
                        "fill-opacity:{}; stroke-opacity:{}",
                        self.color_fill.a() as f32 / 255.0,
                        self.color_edge.a() as f32 / 255.0
                    ),
                ),
        );

        // == SVG text ==
        add_text_to_svg_document(
            document,
            self.label_position,
            self.color_text,
            offset,
            Arc::clone(&self.label_galley),
        );
    }
}
