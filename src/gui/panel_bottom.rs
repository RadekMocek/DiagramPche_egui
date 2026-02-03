use crate::App;

impl App {
    pub fn gui_panel_bottom(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            if self.parser.is_error {
                ui.label(
                    egui::RichText::new(&self.parser.error_message).color(egui::Color32::DARK_RED),
                );
            } else {
                ui.label(""); // Reserve space
            }
        });
    }
}
