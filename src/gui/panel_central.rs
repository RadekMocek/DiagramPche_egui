use crate::App;
use crate::gui::widget;
use crate::helper::icon::*;
use const_format::concatcp;

const NODE_TYPE_CHOICES: [&str; 4] = [
    concatcp!(ICON_RECTANGLE_OUTLINE, " Rectangle"),
    concatcp!(ICON_ELLIPSE_OUTLINE, " Ellipse"),
    concatcp!(ICON_RHOMBUS_OUTLINE, " Diamond"),
    concatcp!(ICON_FORMAT_TEXT_VARIANT, " Text"),
];

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
                left_ui.horizontal(|mut ui| {
                    ui.add_space(widget::TINYSKIP);
                    self.widget_text_editor_font_size_setup(&mut ui);
                    ui.separator();
                    ui.label(format!(
                        "Cursor pos: {}, {}",
                        self.editor_cursor_line, self.editor_cursor_column
                    ));
                });

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
                    ui.add_space(widget::TINYSKIP);
                    // .: Color picker :.
                    ui.label("Node color:");
                    ui.separator();

                    // .: Node type combo :.
                    // In different file:
                    ui.label("Type:");
                    let mut current_choice_idx = 0;
                    egui::ComboBox::from_id_salt("NodeTypeCombo")
                        .selected_text(format!("{}", NODE_TYPE_CHOICES[current_choice_idx]))
                        .show_ui(ui, |ui| {
                            // Loop me
                            ui.selectable_value(&mut current_choice_idx, 0, NODE_TYPE_CHOICES[0]);
                            ui.selectable_value(&mut current_choice_idx, 1, NODE_TYPE_CHOICES[1]);
                            ui.selectable_value(&mut current_choice_idx, 2, NODE_TYPE_CHOICES[2]);
                            ui.selectable_value(&mut current_choice_idx, 3, NODE_TYPE_CHOICES[3]);
                        });
                    ui.separator();

                    // .: Node ID label :.
                    ui.label("ID:");
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
