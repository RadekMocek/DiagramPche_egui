use crate::App;

pub(super) const TINYSKIP: f32 = 4.0;
pub(super) const SMALLSKIP: f32 = 9.0;
pub(super) const MEDSKIP: f32 = 13.0;
pub(super) const BIGSKIP: f32 = 17.0;

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
            ui.add(egui::DragValue::new(&mut self.source_font_size).speed(1));
            if ui.button("-").clicked() {
                self.source_font_size -= 1;
            }
            if ui.button("+").clicked() {
                self.source_font_size += 1;
            }
            self.source_font_size = self.source_font_size.clamp(
                crate::config::FONT_SIZE_SOURCE_MIN,
                crate::config::FONT_SIZE_SOURCE_MAX,
            );
        });
    }
}