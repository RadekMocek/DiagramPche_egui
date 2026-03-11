use crate::App;
use egui::{Color32, FontFamily, FontId, Rect, Response, TextEdit, TextStyle, vec2};

impl App {
    pub(super) fn gui_text_editor(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            ui.style_mut().text_styles.insert(
                TextStyle::Monospace,
                FontId::new(self.source_font_size as f32, FontFamily::Monospace),
            );

            let theme = egui_extras::syntax_highlighting::CodeTheme::from_style(ui.style());

            let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, _wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    ui.style(),
                    &theme,
                    buf.as_str(),
                    if self.do_syntax_highlight { "toml" } else { "" },
                );
                layout_job.wrap.max_width = f32::INFINITY; // ignore `_wrap_width`, this will make horizontal scrolling work
                ui.fonts_mut(|f| f.layout_job(layout_job))
            };

            // The text editor itself
            let response = ui.add(
                TextEdit::multiline(&mut self.source)
                    .desired_width(f32::INFINITY)
                    .code_editor()
                    .layouter(&mut layouter),
            );

            self.error_highlight(ui, &response);
        });
    }

    pub(super) fn error_highlight(&self, ui: &mut egui::Ui, response: &Response) {
        // Error highlight logic, ("eh" == error highlight)
        if self.is_error_span_some {
            let char_size = ui
                .painter()
                .layout_no_wrap(
                    String::from("A"),
                    FontId::new(self.source_font_size as f32, FontFamily::Monospace),
                    Color32::PLACEHOLDER,
                )
                .rect
                .size();

            let text_edit_origin = response.rect.min;
            let eh_top = char_size.y * self.error_span_line as f32;
            let eh_left = char_size.x * self.error_span_column as f32;

            ui.painter().rect_filled(
                Rect::from_min_size(
                    text_edit_origin + vec2(eh_left, eh_top),
                    vec2(char_size.x * self.error_span_length as f32, char_size.y),
                ),
                0,
                crate::config::COLOR_ERROR_HIGHLIGHT,
            );
        }
    }
}
