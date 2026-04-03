use crate::App;
use crate::config;

impl App {
    pub fn gui_panel_bottom(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            if self.parser.is_error {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(&self.parser.error_message.replace('\n', " "))
                            .color(config::COLOR_ERROR),
                    )
                    .truncate(),
                );
            } else {
                ui.label(""); // Reserve space
            }
        });
    }
}
