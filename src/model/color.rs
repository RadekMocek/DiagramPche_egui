pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn new(field0: u8, field1: u8, field2: u8, field3: u8) -> Self {
        Self(field0, field1, field2, field3)
    }

    pub fn to_egui_color(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied_const(self.0, self.1, self.2, self.3)
    }
}
