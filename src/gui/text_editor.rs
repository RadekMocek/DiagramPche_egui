use crate::App;
use crate::config;
use egui::text_selection::CCursorRange;
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
            let text_edit_output = TextEdit::multiline(&mut self.source)
                .desired_width(f32::INFINITY)
                .code_editor()
                .layouter(&mut layouter)
                .show(ui);

            self.textedit_error_highlight(ui, &text_edit_output.response);

            self.textedit_update_cursor_position_info(&text_edit_output.cursor_range);

            if text_edit_output.response.changed() {
                self.is_source_dirty = true;
            }
        });
    }

    pub(super) fn textedit_error_highlight(&self, ui: &mut egui::Ui, response: &Response) {
        // Error highlight logic, ("eh" == error highlight)
        if self.is_error_span_some {
            let char_size = self.textedit_get_char_size(&ui);
            let text_edit_origin = response.rect.min;
            let eh_top = char_size.y * self.error_span_line as f32;
            let eh_left = char_size.x * self.error_span_column as f32;

            ui.painter().rect_filled(
                Rect::from_min_size(
                    text_edit_origin + vec2(eh_left, eh_top),
                    vec2(char_size.x * self.error_span_length as f32, char_size.y),
                ),
                0,
                config::COLOR_ERROR_HIGHLIGHT,
            );
        }
    }

    pub(super) fn textedit_update_cursor_position_info(
        &mut self,
        cursor_range: &Option<CCursorRange>,
    ) {
        if let Some(cursor_range) = cursor_range {
            // UTF-8 shenanigans
            let mut n_newlines = 0;
            let mut n_chars_in_line = 0;
            for char in self.source.chars().take(cursor_range.primary.index) {
                n_chars_in_line += 1;
                if char == '\n' {
                    n_chars_in_line = 0;
                    n_newlines += 1;
                }
            }
            self.editor_cursor_line = n_newlines;
            self.editor_cursor_column = n_chars_in_line;
        }
    }

    fn textedit_get_char_size(&self, ui: &egui::Ui) -> egui::Vec2 {
        ui.painter()
            .layout_no_wrap(
                String::from("A"),
                FontId::new(self.source_font_size as f32, FontFamily::Monospace),
                Color32::PLACEHOLDER,
            )
            .rect
            .size()
    }

    #[allow(dead_code)]
    fn textedit_jump_to_position(ui: &egui::Ui, textedit_id: egui::Id, index: usize) {
        // FOCUS
        ui.ctx().memory_mut(|m| m.request_focus(textedit_id));

        // SET CURSOR POS
        let mut state = TextEdit::load_state(ui.ctx(), textedit_id).unwrap();
        let cursor = egui::text::CCursor::new(index);
        state.cursor.set_char_range(Some(CCursorRange::one(cursor)));
        state.store(ui.ctx(), textedit_id);

        // SCROLL
        // ???
    }
}
