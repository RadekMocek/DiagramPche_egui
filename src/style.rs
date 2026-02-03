use egui::{FontFamily, FontId, TextStyle};
use std::collections::BTreeMap;
use FontFamily::{Monospace, Proportional};

pub(super) fn conf_style_init(ctx: &egui::Context) {
    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::new(30.0, Proportional)),
        (
            TextStyle::Name("Heading2".into()),
            FontId::new(25.0, Proportional),
        ),
        (
            TextStyle::Name("Context".into()),
            FontId::new(23.0, Proportional),
        ),
        (TextStyle::Body, FontId::new(18.0, Proportional)),
        (TextStyle::Monospace, FontId::new(18.0, Monospace)),
        (TextStyle::Button, FontId::new(18.0, Proportional)),
        (TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
    .into();

    ctx.all_styles_mut(|style| style.text_styles = text_styles.clone());
}

pub(super) fn conf_style_panel_top_begin(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style
            .text_styles
            .insert(TextStyle::Button, FontId::new(14.0, Proportional));
    });
}

pub(super) fn conf_style_panel_top_end(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style
            .text_styles
            .insert(TextStyle::Button, FontId::new(18.0, Proportional));
    });
}
