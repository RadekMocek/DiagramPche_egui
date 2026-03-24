use crate::App;
use crate::helper::icon::*;
use crate::logic::app_file::FileExampleId;
use const_format::concatcp;

pub enum ActionAfterUnsavedWarn {
    Invalid,
    Exit,
    New,
    OpenFile,
    LoadExample,
}

impl App {
    pub fn gui_panel_top(&mut self, ctx: &egui::Context) {
        crate::style::conf_style_panel_top_begin(&ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_enabled_ui(!self.benchmark_data.is_running, |ui| {
                // The top panel is often a good place for a menu bar:
                egui::MenuBar::new().ui(ui, |ui| {
                    // .: File :.
                    ui.menu_button("File", |ui| {
                        // . New .
                        if ui
                            .button(concatcp!(ICON_FILE_PLUS_OUTLINE, " New"))
                            .clicked()
                        {
                            if !self.is_source_dirty {
                                self.handle_regular_new();
                            } else {
                                self.action_unsavedwarn_type = ActionAfterUnsavedWarn::New;
                                self.do_open_modal_unsavedwarn = true;
                            }
                        }
                        // . Open .
                        if ui
                            .button(concatcp!(ICON_FOLDER_OPEN_OUTLINE, " Open"))
                            .clicked()
                        {
                            if !self.is_source_dirty {
                                self.handle_regular_open();
                            } else {
                                self.action_unsavedwarn_type = ActionAfterUnsavedWarn::OpenFile;
                                self.do_open_modal_unsavedwarn = true;
                            }
                        }
                        // . Save .
                        if ui
                            .button(concatcp!(ICON_CONTENT_SAVE_OUTLINE, " Save"))
                            .clicked()
                        {
                            self.handle_regular_save();
                        }
                        // . Save as .
                        if ui
                            .button(concatcp!(ICON_CONTENT_SAVE_EDIT_OUTLINE, " Save as"))
                            .clicked()
                        {
                            self.save_source_to_file_from_dialog();
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
                            const ITEMS: [(FileExampleId, &str); 4] = [
                                (FileExampleId::DebugZAxis, "Z-axis, out-of-order"),
                                (FileExampleId::DebugPathLabel, "Path label background"),
                                (FileExampleId::DebugBenchLight, "Benchmark light"),
                                (FileExampleId::DebugBenchHeavy, "Benchmark heavy"),
                            ];
                            for (file_id, gui_name) in ITEMS {
                                if ui.button(gui_name).clicked() {
                                    self.handle_open_example(file_id);
                                }
                            }
                        });
                        // . Benchmark .
                        if ui.button("Benchmark").clicked() {
                            self.do_show_window_benchmark = true;
                        }
                    });
                    // .: Help :.
                    ui.menu_button("Help", |ui| {
                        // .: Examples :.
                        ui.menu_button("Examples", |ui| {
                            // . Example 1 :: CPU block diagram .
                            if ui.button("Example 1: CPU block diagram").clicked() {
                                self.handle_open_example(FileExampleId::ExampleBlockDiag);
                            }
                            // . Example 2 :: BPMN .
                            if ui.button("Example 2: BPMN").clicked() {
                                self.handle_open_example(FileExampleId::ExampleEcoDiag);
                            }
                        });
                        // . About .
                        if ui.button("About").clicked() {
                            self.do_open_modal_about = true;
                        }
                    });
                });
            });
        });

        crate::style::conf_style_panel_top_end(&ctx);
    }
}
