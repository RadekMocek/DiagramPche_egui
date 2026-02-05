use crate::App;
use egui::{Painter, Pos2};

impl App {
    pub(super) fn gui_canvas_draw_nodes(&mut self, painter: &Painter, origin: &Pos2) {
        const COLOR_BLACK: egui::Color32 = egui::Color32::BLACK;
        const NODE_BORDER_OFFSET_BASE: f32 = 10.0;
        let node_padding = NODE_BORDER_OFFSET_BASE * self.zoom_level;

        for (_key, node) in &self.parser.result_nodes {
            let x = node.position.x as f32;
            let y = node.position.y as f32;

            painter.rect_stroke(
                egui::Rect {
                    min: egui::pos2(origin.x + x - node_padding, origin.y + y - node_padding),
                    max: egui::pos2(origin.x + x + node_padding, origin.y + y + node_padding),
                },
                2.0,
                egui::Stroke {
                    width: 2.0 * self.zoom_level,
                    color: COLOR_BLACK,
                },
                egui::StrokeKind::Inside,
            );
        }
    }
}
