use crate::App;
use crate::helper::icon::*;
use const_format::concatcp;

impl App {
    pub fn gui_panel_top(&mut self, ctx: &egui::Context) {
        crate::style::conf_style_panel_top_begin(&ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
                // .: File :.
                ui.menu_button("File", |ui| {
                    // . New .
                    if ui
                        .button(concatcp!(ICON_FILE_PLUS_OUTLINE, " New"))
                        .clicked()
                    {
                        //todo
                    }
                    // . Open .
                    if ui
                        .button(concatcp!(ICON_FOLDER_OPEN_OUTLINE, " Open"))
                        .clicked()
                    {
                        //todo
                    }
                    // . Save .
                    if ui
                        .button(concatcp!(ICON_CONTENT_SAVE_OUTLINE, " Save"))
                        .clicked()
                    {
                        //todo
                    }
                    // . Save as .
                    if ui
                        .button(concatcp!(ICON_CONTENT_SAVE_EDIT_OUTLINE, " Save as"))
                        .clicked()
                    {
                        //todo
                    }
                    // . Export to SVG .
                    if ui
                        .button(concatcp!(ICON_EXPORT, " Export to SVG"))
                        .clicked()
                    {
                        self.modal_export_do_overwrite = false;
                        self.do_open_modal_export = true;
                    }
                    ui.separator();
                    // . Preferences .
                    if ui
                        .button(concatcp!(ICON_WRENCH_OUTLINE, " Preferences"))
                        .clicked()
                    {
                        self.do_show_window_preferences = true;
                    }
                    ui.separator();
                    // . Exit .
                    if ui.button(concatcp!(ICON_EXIT_RUN, " Exit")).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                // .: View :.
                ui.menu_button("View", |ui| {                    
                    // . Toolbar .
                    ui.checkbox(&mut self.do_show_toolbar, "Toolbar");
                    ui.separator();
                    // . Canvas grid .
                    ui.checkbox(&mut self.do_show_grid, "Canvas grid");
                    // . Secondary canvas toolbar .
                    ui.checkbox(
                        &mut self.do_show_secondary_canvas_toolbar,
                        "Secondary canvas toolbar",
                    );
                    // . Jump to canvas origin .
                    ui.separator();
                    if ui.button("Jump to canvas origin").clicked() {
                        self.reset_canvas_scrolling_and_zoom();
                    }
                });
                // .: Debug :.
                ui.menu_button("Debug", |ui| {
                    // .: Render tests :.
                    ui.menu_button("Render tests", |ui| {
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
                        // . Example 1 :: CPU block diagram .
                        if ui.button("Example 1: CPU block diagram").clicked() {
                            self.load_source_from_example("example1");
                        }
                        // . Example 2 :: BPMN .
                        if ui.button("Example 2: BPMN").clicked() {
                            self.load_source_from_example("example2");
                        }
                    });
                    // . About .
                    if ui.button("About").clicked() {
                        self.do_open_modal_about = true;
                    }
                });
            });
        });

        crate::style::conf_style_panel_top_end(&ctx);
    }
}
