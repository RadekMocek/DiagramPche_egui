use crate::model::draw_command::command::DrawCommand;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Galley};
use egui::Painter;
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

    fn draw_svg(&self, document: &mut Document, origin: Pos2, zoom_level: f32) {
        let top_left = (self.rect_top_left - origin) / zoom_level;
        let bottom_right = (self.rect_bottom_right - origin) / zoom_level;
        let width = bottom_right.x - top_left.x;
        let height = bottom_right.y - top_left.y;

        document.append(
            svg::node::element::Rectangle::new()
                .set("x", top_left.x)
                .set("y", top_left.y)
                .set("width", width)
                .set("height", height)
                .set("fill", "green"),
        );
    }
}
