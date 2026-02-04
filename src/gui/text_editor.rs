use crate::App;
use egui::TextStyle;

impl App {
    pub(super) fn gui_text_editor(&mut self, ui: &mut egui::Ui) {
        let response = ui.add(
            egui::TextEdit::multiline(&mut self.source)
                .desired_width(f32::INFINITY)
                .code_editor(),
        );

        if self.is_error_span_some {
            let text_edit_origin = egui::pos2(response.rect.min.x, response.rect.min.y);
            let text_line_height = ui.text_style_height(&TextStyle::Monospace) + 0.04; // TODO magic number

            // TODO this is terrible
            let char_width_x = ui
                .painter()
                .layout_no_wrap(
                    String::from("A"),
                    egui::FontId::new(18.0, eframe::epaint::FontFamily::Monospace),
                    egui::Color32::PLACEHOLDER,
                )
                .rect
                .size()
                .x;

            // eh == error highlight
            let eh_top = text_line_height * self.error_span_line as f32;
            let eh_left = char_width_x * self.error_span_column as f32;

            ui.painter().rect_filled(
                egui::Rect {
                    min: text_edit_origin + egui::vec2(eh_left, eh_top),
                    max: text_edit_origin
                        + egui::vec2(
                            eh_left + char_width_x * self.error_span_length as f32,
                            eh_top + text_line_height,
                        ),
                },
                0,
                egui::Color32::from_rgba_unmultiplied_const(211, 1, 2, 80),
            );
        }
    }
}
