use crate::App;
use crate::config;

pub const TINYSKIP: f32 = 4.0;
pub const SMALLSKIP: f32 = 9.0;
pub const MEDSKIP: f32 = 13.0;
pub const BIGSKIP: f32 = 17.0;

pub(super) fn header(ui: &mut egui::Ui, space_height: f32, text: &str) {
    ui.add_space(space_height);
    //ui.weak(text);
    ui.label(egui::RichText::new(text).weak().italics());
    ui.separator();
}

impl App {
    pub(super) fn widget_text_editor_font_size_setup(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Font size:");
            ui.add(egui::DragValue::new(&mut self.source_font_size).speed(0.1));
            if ui.button("-").clicked() {
                self.source_font_size -= config::FONT_SIZE_SOURCE_STEP;
            }
            if ui.button("+").clicked() {
                self.source_font_size += config::FONT_SIZE_SOURCE_STEP;
            }
            // Make sure it's even (step size is 2, but we cannot set that to the DragValue)
            self.source_font_size += self.source_font_size % 2;
            // Clamp (DragValue could do the clamping, but buttons are handled after it, so we would get a one frame delay)
            self.source_font_size = self
                .source_font_size
                .clamp(config::FONT_SIZE_SOURCE_MIN, config::FONT_SIZE_SOURCE_MAX);
        });
    }

    pub(super) fn widget_text_editor_preferred_combo(&mut self, ui: &mut egui::Ui) {
        const CHOICES: [&str; 2] = [
            "Vanilla (TextEdit::multiline)",
            "3rd Party (egui_code_editor)",
        ];

        let current_choice_idx = if !self.do_use_alt_editor { 0 } else { 1 };

        egui::ComboBox::from_id_salt("PreferredTextEditorCombo")
            .selected_text(format!("{}", CHOICES[current_choice_idx]))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.do_use_alt_editor, false, CHOICES[0]);
                ui.selectable_value(&mut self.do_use_alt_editor, true, CHOICES[1]);
            });
    }

    pub(super) fn widget_text_editor_syntax_highlight_checkbox(&mut self, ui: &mut egui::Ui) {
        ui.add_enabled_ui(!self.do_use_alt_editor, |ui| {
            ui.checkbox(&mut self.do_syntax_highlight, "Enable syntax highlight");
        });
    }
}
