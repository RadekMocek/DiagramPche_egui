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
                // .: View :.
                ui.menu_button("View", |ui| {
                    // . Grid .
                    if ui.button("Grid").clicked() {
                        //todo
                    }
                });
                // .: Debug :.
                ui.menu_button("Debug", |ui| {
                    // .: Debug diagrams :.
                    ui.menu_button("Debug diagrams", |ui| {
                        // . Z-axis, out-of-order .
                        if ui.button("Z-axis, out-of-order").clicked() {
                            self.load_source_from_example("debug1");
                        }
                    });
                });
                // .: Help :.
                ui.menu_button("Help", |ui| {
                    // .: Examples :.
                    ui.menu_button("Examples", |ui| {
                        // . Example 1 .
                        if ui.button("Example 1").clicked() {
                            self.load_source_from_example("example1");
                        }
                    });
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
