use crate::App;
use crate::gui::widget;

impl App {
    pub fn gui_panel_central(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |mut ui| {
            let available_space = ui.available_rect_before_wrap();
            let split_position =
                available_space.left() + available_space.width() * self.central_split_ratio;

            const SEPARATOR_HALF_WIDTH: f32 = 8.0 / 2.0;

            // Left panel :: text editor
            let left_rect = egui::Rect::from_min_max(
                available_space.min,
                egui::pos2(split_position - SEPARATOR_HALF_WIDTH, available_space.max.y),
            );
            // Right panel :: canvas
            let right_rect = egui::Rect::from_min_max(
                egui::pos2(split_position + SEPARATOR_HALF_WIDTH, available_space.min.y),
                available_space.max,
            );
            // Separator
            let separator_rect = egui::Rect::from_min_max(
                egui::pos2(split_position - SEPARATOR_HALF_WIDTH, available_space.min.y),
                egui::pos2(split_position + SEPARATOR_HALF_WIDTH, available_space.max.y),
            );
            let separator_response = ui.interact(
                separator_rect,
                ui.id().with("separator"),
                egui::Sense::drag(),
            );

            // Handle separator dragging
            if separator_response.dragged() {
                self.central_split_ratio = (self.central_split_ratio
                    + separator_response.drag_delta().x / available_space.width())
                .clamp(0.1, 0.9);
            }

            // Change cursor when hovering separator
            if separator_response.hovered() || separator_response.dragged() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            }

            // Draw left panel (text editor)
            let mut left_ui = ui.new_child(egui::UiBuilder::new().max_rect(left_rect));
            left_ui.set_clip_rect(left_rect);

            // Left toolbar
            if self.do_show_toolbar {
                self.widget_text_editor_font_size_setup(&mut left_ui);
                left_ui.add_space(widget::TINYSKIP);
            }

            // Text editor
            egui::ScrollArea::both()
                .id_salt("source")
                .auto_shrink(false)
                .show(&mut left_ui, |ui| {
                    if !self.do_use_alt_editor {
                        self.gui_text_editor(ui);
                    } else {
                        self.gui_text_editor_alt(ui);
                    }
                });

            // Draw right panel (canvas)
            let mut right_ui = ui.new_child(egui::UiBuilder::new().max_rect(right_rect));

            // Right toolbar
            if self.do_show_toolbar {
                right_ui.horizontal(|ui| {
                    ui.label("Color:");
                });
                right_ui.add_space(widget::TINYSKIP);
            }

            // Canvas
            egui::Frame::canvas(&right_ui.style())
                .fill(crate::config::COLOR_CANVAS_BACKGROUND)
                .show(&mut right_ui, |ui| {
                    self.gui_canvas(ui);
                });

            // --- --- --- --- --- ---

            // Modeless windows logic
            self.gui_window(&mut ui);

            // Modals logic
            self.gui_modal(&mut ui);
        });
    }
}
