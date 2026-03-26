use crate::App;

const N_BATCH_LABELS: u32 = 20;
const N_BATCH_TEXTEDITS: u32 = 20;
const N_BATCH_BUTTONS: u32 = 20;
const N_BATCH_CHECKBOXES: u32 = 48;
const N_BATCH_RADIOS: u32 = 48;
const N_BATCH_SLIDERS: u32 = 16;
//
const N_BATCH_WIDGETS: u32 = N_BATCH_LABELS
    + N_BATCH_TEXTEDITS
    + N_BATCH_BUTTONS
    + N_BATCH_CHECKBOXES
    + N_BATCH_RADIOS
    + N_BATCH_SLIDERS;
//

impl App {
    pub(super) fn gui_widgetbench(&mut self, ui: &mut egui::Ui) {
        egui::Window::new(format!(
            "Widget spree {} x {}",
            self.widgetbench_data.n_batches, N_BATCH_WIDGETS
        ))
        .open(&mut self.widgetbench_data.do_show_window)
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::default())
        .show(ui.ctx(), |ui| {
            ui.label(
                egui::RichText::new(
                    "This benchmark is run from terminal: '.\\diagram_pche_egui.exe w'\n\
                [!] epilepsy warning: this window will flicker during the benchmark",
                )
                .color(crate::config::COLOR_ERROR),
            );

            for _ in 0..self.widgetbench_data.n_batches {
                ui.horizontal(|ui| {
                    for _ in 0..N_BATCH_LABELS {
                        ui.label("ABCČDĎEF");
                    }
                });
                ui.horizontal(|ui| {
                    for _ in 0..N_BATCH_TEXTEDITS {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.widgetbench_data.textedittext)
                                .desired_width(72.0),
                        );
                    }
                });
                ui.horizontal(|ui| {
                    for _ in 0..N_BATCH_BUTTONS {
                        _ = ui.button("LMNŇOPQ");
                    }
                });
                ui.horizontal(|ui| {
                    for _ in 0..N_BATCH_CHECKBOXES {
                        ui.checkbox(&mut true, "R");
                    }
                });
                ui.horizontal(|ui| {
                    for _ in 0..N_BATCH_RADIOS {
                        _ = ui.radio(true, "Ř");
                    }
                });
                ui.horizontal(|ui| {
                    for _ in 0..N_BATCH_SLIDERS {
                        ui.add(egui::Slider::new(&mut 5, 0..=9).show_value(false));
                    }
                });
            }
        });

        //self.widgetbench_data.do_show_window = do_show_window;
    }
}
