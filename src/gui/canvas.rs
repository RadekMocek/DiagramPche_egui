use crate::config::*;
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
        let response_rect = response.rect;

        // Draw grid
        if self.do_show_grid {
            let grid_step = CANVAS_GRID_STEP_BASE * self.zoom_level;
            let grid_stroke = egui::Stroke::new(1.0, COLOR_CANVAS_GRID_LINE);

            let mut x = self.scrolling.x.rem_euclid(grid_step);
            while x < response_rect.width() {
                painter.vline(
                    response_rect.left() + x,
                    response_rect.y_range(),
                    grid_stroke,
                );
                x += grid_step;
            }

            let mut y = self.scrolling.y.rem_euclid(grid_step);
            while y < response_rect.height() {
                painter.hline(
                    response_rect.x_range(),
                    response_rect.top() + y,
                    grid_stroke,
                );
                y += grid_step;
            }
        }

        // Origin ([0,0]) of the canvas in screen space coordinates, which painter uses
        let origin = egui::pos2(
            response_rect.min.x + self.scrolling.x,
            response_rect.min.y + self.scrolling.y,
        );

        // .: User interaction :.
        // .:==================:.
        if response.hovered() {
            // MW to zoom
            let scroll = ui.input(|i| {
                i.events.iter().find_map(|e| match e {
                    egui::Event::MouseWheel {
                        unit: _,
                        delta,
                        modifiers: _,
                    } => Some(*delta),
                    _ => None,
                })
            });
            if let Some(scroll) = scroll {
                const ZOOM_STEP: f32 = 0.2;
                const ZOOM_MIN: f32 = 0.3;
                const ZOOM_MAX: f32 = 2.0;
                self.zoom_level =
                    (self.zoom_level + scroll.y * ZOOM_STEP).clamp(ZOOM_MIN, ZOOM_MAX);
            }

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
        }

        // .: Draw on canvas :.
        // .:================:.
        self.canvas_nodes.clear();
        self.gui_canvas_prepare_nodes(&painter, &origin);
        self.gui_canvas_prepare_paths(&origin);

        while !self.draw_commands_ord.is_empty() {
            if let Some(draw_command_ord) = self.draw_commands_ord.pop() {
                draw_command_ord
                    .draw_command
                    .draw(&painter, self.zoom_level);
            }
        }

        // .: User AABR interaction :.
        // .:=======================:.
        // TODO NOT IDEAL
        /*
        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let pointer_pos_in_canvas = pointer_pos - origin;

            // Show tooltip with Node ID on hover
            let mut tooltip = String::new();
            let mut is_first_id = true;
            for (key, value) in &self.canvas_nodes {
                if value.is_point_inside_incl(pointer_pos_in_canvas) {
                    if !is_first_id {
                        tooltip.push_str(", ");
                    }
                    tooltip.push_str(key);
                    is_first_id = false;
                }
            }
            if !tooltip.is_empty() {
                response.show_tooltip_text(tooltip);
            }
        }
        */

        //
        response
    }
}
