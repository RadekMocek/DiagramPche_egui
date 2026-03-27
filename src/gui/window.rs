use crate::App;
use crate::gui::widget;
use crate::style::change_appearance_theme;

#[derive(PartialEq)]
pub enum PreferencesTab {
    Appearance,
    TextEditor,
    View,
}

impl App {
    pub(super) fn gui_window(&mut self, ui: &mut egui::Ui) {
        let mut do_show_preferences = self.do_show_window_preferences; // to satisfy the borrow checker

        // .: Preferences :.
        // .:=============:.
        egui::Window::new("Preferences")
            .open(&mut do_show_preferences)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.window_preferences_selected_tab,
                        PreferencesTab::Appearance,
                        "Appearance",
                    );
                    ui.selectable_value(
                        &mut self.window_preferences_selected_tab,
                        PreferencesTab::TextEditor,
                        "Text editor",
                    );
                    ui.selectable_value(
                        &mut self.window_preferences_selected_tab,
                        PreferencesTab::View,
                        "View",
                    );
                });
                ui.separator();
                match self.window_preferences_selected_tab {
                    PreferencesTab::Appearance => {
                        // .::.
                        widget::header(ui, widget::SMALLSKIP, "App color theme");
                        let prev_choice = self.style_is_light_mode;
                        ui.horizontal(|ui| {
                            ui.radio_value(&mut self.style_is_light_mode, true, "Light");
                            ui.add_space(widget::SMALLSKIP);
                            ui.radio_value(&mut self.style_is_light_mode, false, "Dark");
                        });
                        if prev_choice != self.style_is_light_mode {
                            change_appearance_theme(ui.ctx(), self.style_is_light_mode);
                        }
                        // .::.
                        widget::header(ui, widget::SMALLSKIP, "Canvas color theme");
                        ui.add_enabled_ui(!self.style_is_light_mode, |ui| {
                            ui.checkbox(&mut self.style_do_force_light_canvas, "Keep canvas light");
                        });
                    }
                    PreferencesTab::TextEditor => {
                        // .::.
                        widget::header(ui, widget::SMALLSKIP, "Text editor font size");
                        self.widget_text_editor_font_size_setup(ui);
                        // .::.
                        widget::header(ui, widget::SMALLSKIP, "Preferred text editor");
                        self.widget_text_editor_preferred_combo(ui);
                        // .::.
                        widget::header(ui, widget::SMALLSKIP, "Text editor syntax highlight");
                        self.widget_text_editor_syntax_highlight_checkbox(ui);
                    }
                    PreferencesTab::View => {
                        ui.checkbox(&mut self.do_show_toolbar, "Toolbar");
                        ui.checkbox(&mut self.do_show_grid, "Canvas grid");
                        ui.checkbox(
                            &mut self.do_show_secondary_canvas_toolbar,
                            "Secondary canvas toolbar",
                        );
                    }
                }
            });

        self.do_show_window_preferences = do_show_preferences;

        // .: Benchmark :.
        // .:===========:.
        let mut do_show_benchmark = self.do_show_window_benchmark;

        egui::Window::new("Benchmark nodes")
            .open(&mut do_show_benchmark)
            .resizable(false)
            .title_bar(!self.benchmark_data.is_running)
            .anchor(egui::Align2::LEFT_TOP, [8.0, 25.0])
            .show(ui.ctx(), |ui| {
                if !self.benchmark_data.is_running {
                    ui.label("Syntax highlight may affect performance:");
                    self.widget_text_editor_preferred_combo(ui);
                    self.widget_text_editor_syntax_highlight_checkbox(ui);
                    ui.add_space(widget::SMALLSKIP);

                    ui.label("Choose one of the three benchmarks:");
                    const CHOICES: [&str; 3] = [
                        "Light",
                        "Heavy",
                        "Gradual",
                    ];
                    egui::ComboBox::from_id_salt("BenchmarkTypeCombo")
                        .selected_text(format!("{}", CHOICES[self.benchmark_data.type_choice_idx]))
                        .show_ui(ui, |ui| {
                            for i in 0..CHOICES.len() {
                                ui.selectable_value(&mut self.benchmark_data.type_choice_idx, i, CHOICES[i]);
                            }
                        });
                    ui.add_space(widget::SMALLSKIP);

                    ui.label("Or hide the text editor completely:");
                    ui.checkbox(&mut self.do_skip_text_edit, "hide text editor");
                    ui.add_space(widget::SMALLSKIP);

                    ui.separator();
                    ui.add_space(widget::TINYSKIP);
                    if self.is_source_dirty {
                        ui.label(
                            egui::RichText::new("You have unsaved changes, save your work before running the benchmark.")
                                .color(crate::config::COLOR_ERROR),
                        );
                        ui.label("(If you don't wish to save this, select File → New → Discard.)");
                    } else {
                        if ui.button("Start benchmark").clicked() {
                            self.benchmark_start(ui.ctx());
                        }
                    }
                } else {
                    self.benchmark_gui_update(ui);
                }
            });

        self.do_show_window_benchmark = do_show_benchmark;
    }
}
