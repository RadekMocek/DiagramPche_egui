use crate::App;

impl App {
    pub fn gui_panel_central(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |mut ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.columns(2, |columns| {
                egui::ScrollArea::vertical()
                    .id_salt("source")
                    .show(&mut columns[0], |ui| {
                        self.gui_text_editor(ui);
                    });

                egui::Frame::canvas(&columns[1].style())
                    .fill(crate::config::COLOR_CANVAS_BACKGROUND)
                    .show(&mut columns[1], |ui| {
                        self.gui_canvas(ui);
                    });
            });

            self.gui_modal(&mut ui);
        });
    }
}
