use crate::App;
use egui::{pos2, vec2, Color32, Rect, TextEdit, TextStyle, Vec2};

impl App {
    pub(super) fn gui_text_editor(&mut self, ui: &mut egui::Ui) {
        const COLOR_ERROR_HIGHLIGHT: Color32 = Color32::from_rgba_unmultiplied_const(211, 1, 2, 80);

        // The text editor itself
        let response = ui.add(
            TextEdit::multiline(&mut self.source)
                .desired_width(f32::INFINITY)
                .code_editor(),
        );

        // Error highlight logic, ("eh" == error highlight)
        if self.is_error_span_some {
            if self.font_char_size == Vec2::ZERO {
                self.update_font_char_size(ui);
            }

            let text_edit_origin = pos2(response.rect.min.x, response.rect.min.y);
            let text_line_height = ui.text_style_height(&TextStyle::Monospace);
            let char_width = self.font_char_size.x;

            let eh_top = text_line_height * self.error_span_line as f32;
            let eh_left = char_width * self.error_span_column as f32;
            let eh_magic_offset = vec2(4.0, 2.2);

            ui.painter().rect_filled(
                Rect::from_min_size(
                    text_edit_origin + vec2(eh_left, eh_top) + eh_magic_offset,
                    vec2(char_width * self.error_span_length as f32, text_line_height),
                ),
                0,
                COLOR_ERROR_HIGHLIGHT,
            );
        }
    }
}
