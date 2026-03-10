pub(super) const SMALLSKIP: f32 = 9.0;
pub(super) const MEDSKIP: f32 = 13.0;
pub(super) const BIGSKIP: f32 = 17.0;

pub(super) fn header(ui: &mut egui::Ui, space_height: f32, text: &str) {
    ui.add_space(space_height);
    ui.weak(text);
    ui.separator();
}