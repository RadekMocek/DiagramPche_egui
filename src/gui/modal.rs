use crate::App;

impl App {
    pub fn gui_modal(&mut self, ui: &mut egui::Ui) {
        if self.do_open_modal_about {
            let modal = egui::Modal::new(egui::Id::new("modal_about")).show(ui.ctx(), |ui| {
                ui.heading("About");
                ui.add_space(6.0);
                ui.label("DiagramPche :: egui");
                ui.hyperlink("https://github.com/RadekMocek/DiagramPche_egui");
                ui.add_space(12.0);
                if ui.button("Close").clicked() {
                    ui.close();
                }
            });

            if modal.should_close() {
                self.do_open_modal_about = false;
            }
        }
    }
}
