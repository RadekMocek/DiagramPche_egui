use crate::App;

impl App {
    pub(super) fn gui_canvas(&mut self, ui: &mut egui::Ui) -> egui::Response {
        // .: Options and state :.
        // .:===================:.
        // ...

        // .: Canvas init :.
        // .:=============:.
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
        self.canvas_nodes.clear();
        self.gui_canvas_draw_nodes(&painter, &origin);

        //
        response
    }
}
