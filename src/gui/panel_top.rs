use crate::App;

impl App {
    pub fn gui_panel_top(&mut self, ctx: &egui::Context) {
        const FONT_SIZE: f32 = 14.0;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
                // .: File :.
                ui.menu_button(egui::RichText::new("File").size(FONT_SIZE), |ui| {
                    // . Exit .
                    if ui
                        .button(egui::RichText::new("Exit").size(FONT_SIZE))
                        .clicked()
                    {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                // .: Help :.
                ui.menu_button(egui::RichText::new("Help").size(FONT_SIZE), |ui| {
                    // . About .
                    if ui
                        .button(egui::RichText::new("About...").size(FONT_SIZE))
                        .clicked()
                    {
                        self.do_open_modal_about = true;
                    }
                });
            });
        });
    }
}
