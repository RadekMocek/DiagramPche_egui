use egui::{FontFamily, FontId, TextStyle};
use std::collections::BTreeMap;

pub fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (TextStyle::Name("Heading2".into()), FontId::new(25.0, Proportional)),
        (TextStyle::Name("Context".into()), FontId::new(23.0, Proportional)),
        (TextStyle::Body, FontId::new(18.0, Proportional)),
        (TextStyle::Monospace, FontId::new(18.0, Monospace)),
        (TextStyle::Button, FontId::new(18.0, Proportional)),
        (TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
        .into();

    ctx.all_styles_mut(|style| style.text_styles = text_styles.clone());
}
