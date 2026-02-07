use crate::model::draw_command::command::DrawCommand;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Galley};
use egui::Painter;
use std::sync::Arc;

pub struct NodeRectangleDrawCommand {
    rect_top_left: Pos2,
    rect_bottom_right: Pos2,
    rect_color: Color32,
    label_position: Pos2,
    label_galley: Arc<Galley>,
}

impl NodeRectangleDrawCommand {
    pub fn new(
        rect_top_left: Pos2,
        rect_bottom_right: Pos2,
        rect_color: Color32,
        label_position: Pos2,
        label_galley: Arc<Galley>,
    ) -> Self {
        Self {
            rect_top_left,
            rect_bottom_right,
            rect_color,
            label_position,
            label_galley,
        }
    }
}

impl DrawCommand for NodeRectangleDrawCommand {
    fn draw(&self, painter: &Painter, zoom_level: f32) {
        painter.rect(
            egui::Rect::from_min_max(self.rect_top_left, self.rect_bottom_right),
            0,
            self.rect_color,
            egui::Stroke::new(zoom_level, Color32::BLACK),
            egui::StrokeKind::Inside,
        );

        painter.galley(
            self.label_position,
            self.label_galley.clone(),
            Color32::BLACK,
        );
    }
}
