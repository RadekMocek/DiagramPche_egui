use crate::App;
use crate::gui::widget;

#[derive(PartialEq)]
pub enum PreferencesTab {
    Appearance,
    TextEditor,
    View,
}

impl App {
    pub(super) fn gui_window(&mut self, ui: &mut egui::Ui) {
        let mut open = self.do_show_window_preferences; // to satisfy the borrow checker

        // .: Preferences :.
        // .:=============:.
        egui::Window::new("Preferences")
            .open(&mut open)
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
                        widget::header(ui, widget::SMALLSKIP, "App color theme");
                        widget::header(ui, widget::SMALLSKIP, "Canvas color theme");
                    }
                    PreferencesTab::TextEditor => {
                        // .::.
                        widget::header(ui, widget::SMALLSKIP, "Text editor font size");
                        self.widget_text_editor_font_size_setup(ui);
                        // .::.
                        widget::header(ui, widget::SMALLSKIP, "Preferred text editor");

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
                        // .::.
                        widget::header(ui, widget::SMALLSKIP, "Text editor syntax highlight");
                        ui.add_enabled_ui(!self.do_use_alt_editor, |ui| {
                            ui.checkbox(&mut self.do_syntax_highlight, "Enable syntax highlight");
                        });
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

        self.do_show_window_preferences = open;
    }
}
