use crate::App;

impl App {
    pub fn gui_panel_top(&mut self, ctx: &egui::Context) {
        crate::style::conf_style_panel_top_begin(&ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
                // .: File :.
                ui.menu_button("File", |ui| {
                    // . Exit .
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                // .: Help :.
                ui.menu_button("Help", |ui| {
                    // . About .
                    if ui.button("About...").clicked() {
                        self.do_open_modal_about = true;
                    }
                });
            });
        });

        crate::style::conf_style_panel_top_end(&ctx);
    }
}
