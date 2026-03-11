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
        // .: Preferences :.
        // .:=============:.
        egui::Window::new("Preferences").open(&mut self.do_show_window_preferences).show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.window_preferences_selected_tab, PreferencesTab::Appearance, "Appearance");
                ui.selectable_value(&mut self.window_preferences_selected_tab, PreferencesTab::TextEditor, "Text editor");
                ui.selectable_value(&mut self.window_preferences_selected_tab, PreferencesTab::View, "View");
            });
            ui.separator();
            match self.window_preferences_selected_tab {
                PreferencesTab::Appearance => {
                    widget::header(ui, widget::SMALLSKIP, "App color theme");
                    widget::header(ui, widget::SMALLSKIP, "Canvas color theme");
                }
                PreferencesTab::TextEditor => {
                    widget::header(ui, widget::SMALLSKIP, "Text editor font size");
                    widget::header(ui, widget::SMALLSKIP, "Text editor syntax highlight");
                    ui.checkbox(&mut self.do_syntax_highlight,"Enable syntax highlight");
                }
                PreferencesTab::View => {
                    ui.label("3");
                }
            }
        });
    }
}