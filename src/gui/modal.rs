use crate::App;
use crate::logic::app_dialog::save_svg_dialog;

const SMALLSKIP: f32 = 9.0;
const MEDSKIP: f32 = 13.0;
const BIGSKIP: f32 = 17.0;

#[derive(PartialEq)]
pub enum ActionAfterExport {
    DoNothing,
    OpenFolder,
    OpenFile,
}

fn my_header(ui: &mut egui::Ui, space_height: f32, text: &str) {
    ui.add_space(space_height);
    ui.weak(text);
    ui.separator();
}

impl App {
    pub(super) fn gui_modal(&mut self, ui: &mut egui::Ui) {
        // .: Export modal :.
        // .:==============:.
        if self.do_open_modal_export {
            let modal = egui::Modal::new(egui::Id::new("modal_export")).show(ui.ctx(), |ui| {
                ui.heading("Export to SVG");
                // . Location .
                my_header(ui, SMALLSKIP, "Location");
                ui.horizontal(|ui| {
                    // Location textedit
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.modal_export_path)
                            .hint_text(egui::RichText::new("input path to SVG here").weak()),
                    );
                    if response.changed() {
                        self.modal_export_do_overwrite = false;
                    }
                    // Location Browse... button
                    if ui.button("Browse...").clicked() {
                        if let Some(new_path) = save_svg_dialog() {
                            self.modal_export_path = new_path;
                            self.modal_export_do_overwrite = false;
                        }
                    }
                });

                // . Overwrite guard .
                my_header(ui, MEDSKIP, "Overwrite guard");

                let mut can_export = true;
                let mut is_overwrite_export_needed = false;
                let path = std::path::Path::new(&self.modal_export_path);

                if let Ok(result) = path.try_exists() {
                    if !result {
                        ui.label("Specified path is unique.");
                    } else {
                        can_export = false;
                        if !path.is_dir() {
                            ui.label(
                                egui::RichText::new("File at the specified path already exists.")
                                    .color(crate::config::COLOR_ERROR),
                            );
                            is_overwrite_export_needed = true;
                            if self.modal_export_do_overwrite {
                                can_export = true;
                            }
                        } else {
                            ui.label(
                                egui::RichText::new("The specified path is a directory.")
                                    .color(crate::config::COLOR_ERROR),
                            );
                        }
                    }
                } else {
                    ui.label("Path verification failed.");
                }

                ui.add_enabled_ui(is_overwrite_export_needed, |ui| {
                    ui.checkbox(&mut self.modal_export_do_overwrite, "Overwrite");
                });

                // . Action after export .
                my_header(ui, MEDSKIP, "Action after export");
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut self.modal_export_action_choice,
                        ActionAfterExport::DoNothing,
                        "Nothing",
                    );
                    ui.radio_value(
                        &mut self.modal_export_action_choice,
                        ActionAfterExport::OpenFolder,
                        "Show in explorer",
                    );
                    ui.radio_value(
                        &mut self.modal_export_action_choice,
                        ActionAfterExport::OpenFile,
                        "Open",
                    );
                });

                // . Export & Cancel buttons .
                ui.add_space(BIGSKIP);
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(can_export, |ui| {
                        if ui.button("Export").clicked() {
                            self.svg_exporter.reset();
                            self.do_svg_export_this_iter = true;
                            ui.close();
                        }
                    });
                    if ui.button("Cancel").clicked() {
                        ui.close();
                    }
                });
            });
            if modal.should_close() {
                self.do_open_modal_export = false;
            }
        }

        // .: About modal :.
        // .:=============:.
        if self.do_open_modal_about {
            let modal = egui::Modal::new(egui::Id::new("modal_about")).show(ui.ctx(), |ui| {
                ui.heading("About");
                ui.add_space(SMALLSKIP);
                ui.label("DiagramPche :: egui");
                ui.hyperlink("https://github.com/RadekMocek/DiagramPche_egui");
                ui.add_space(BIGSKIP);
                if ui.button("Close").clicked() {
                    ui.close();
                }
            });
            if modal.should_close() {
                self.do_open_modal_about = false;
            }
        }

        // .: Error modal :.
        // .:=============:.
        if self.do_open_modal_error {
            let modal = egui::Modal::new(egui::Id::new("modal_error")).show(ui.ctx(), |ui| {
                ui.heading("Error");
                ui.add_space(SMALLSKIP);
                ui.label(
                    egui::RichText::new(&self.modal_error_message)
                        .color(crate::config::COLOR_ERROR),
                );
                ui.add_space(BIGSKIP);
                if ui.button("RIP").clicked() {
                    ui.close();
                }
            });
            if modal.should_close() {
                self.do_open_modal_error = false;
            }
        }
    }

    pub fn show_error_modal(&mut self, error_message: &str) {
        self.modal_error_message = String::from(error_message);
        self.do_open_modal_error = true;
    }
}
