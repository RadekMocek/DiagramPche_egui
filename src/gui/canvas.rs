use crate::model::position::Position;
use crate::App;

impl App {
    pub(super) fn gui_canvas(&mut self, ui: &mut egui::Ui) -> egui::Response {
        // .: Options and state :.
        // .:===================:.
        const COLOR_BLACK: egui::Color32 = egui::Color32::BLACK;
        const NODE_BORDER_OFFSET_BASE: f32 = 10.0;
        let node_padding = NODE_BORDER_OFFSET_BASE * self.zoom_level;

        // .: Prepare ground for the canvas :.
        // .:===============================:.
        // Painter is our canvas
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

        // Origin ([0,0]) of the canvas in screen space coordinates, which painter uses
        let origin = egui::pos2(
            response.rect.min.x + self.scrolling.x,
            response.rect.min.y + self.scrolling.y,
        );

        // .: User interaction :.
        // .:==================:.
        /*
        // Handle mouse events
        let pointer_pos_in_canvas;
        if let Some(pointer_pos) = response.interact_pointer_pos() {
            pointer_pos_in_canvas = pointer_pos - origin;
        }
        */

        // RMB to move canvas ("scrolling")
        if response.drag_started_by(egui::PointerButton::Secondary) {
            self.is_canvas_dragged = true;
        }
        if self.is_canvas_dragged {
            self.scrolling += response.drag_delta();
        }
        if response.drag_stopped_by(egui::PointerButton::Secondary) {
            self.is_canvas_dragged = false;
        }

        // .: Draw on canvas :.
        // .:================:.

        for node in &self.parser.nodes {
            let Position::Absolute(x, y) = node.position else {
                continue;
            };
            let x = x as f32;
            let y = y as f32;

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

        //
        response
    }
}
