pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn new(field0: u8, field1: u8, field2: u8, field3: u8) -> Self {
        Self(field0, field1, field2, field3)
    }

    pub fn from_str(color_str: &str) -> Self {
        if color_str.is_ascii() && color_str.len() == 9 && color_str.starts_with('#') {
            if let Ok(parsed) = u32::from_str_radix(&color_str[1..], 16) {
                let bytes = parsed.to_be_bytes();
                return Self(bytes[0], bytes[1], bytes[2], bytes[3]);
            }
        }
        Self(0, 0, 0, 0)
    }

    pub fn to_egui_color(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied_const(self.0, self.1, self.2, self.3)
    }

    pub fn to_picker_arr(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}

pub fn get_rgba_hex_quoted_from_u8arr(arr: [u8; 4]) -> String {
    format!(
        "\"#{:02X?}{:02X?}{:02X?}{:02X?}\"",
        arr[0], arr[1], arr[2], arr[3]
    )
}
