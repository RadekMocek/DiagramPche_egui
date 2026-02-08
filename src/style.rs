use crate::config::*;
use egui::{Color32, FontFamily, FontId, TextStyle};
use std::collections::BTreeMap;
use FontFamily::{Monospace, Proportional};

pub(super) fn conf_style_init(ctx: &egui::Context) {
    let text_styles: BTreeMap<TextStyle, FontId> = [
        (
            TextStyle::Heading,
            FontId::new(FONT_SIZE_DEFAULT + 12.0, Proportional),
        ),
        (
            TextStyle::Name("Heading2".into()),
            FontId::new(FONT_SIZE_DEFAULT + 7.0, Proportional),
        ),
        (
            TextStyle::Name("Context".into()),
            FontId::new(FONT_SIZE_DEFAULT + 5.0, Proportional),
        ),
        (
            TextStyle::Body,
            FontId::new(FONT_SIZE_DEFAULT, Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(FONT_SIZE_DEFAULT, Monospace),
        ),
        (
            TextStyle::Button,
            FontId::new(FONT_SIZE_DEFAULT, Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(FONT_SIZE_DEFAULT - 8.0, Proportional),
        ),
    ]
    .into();

    ctx.all_styles_mut(|style| {
        // Set font sizes defined above
        style.text_styles = text_styles.clone();
        // Black font instead of dark gray
        // (This may need to be changed later if we implement dark/light mode switch)
        style.visuals.override_text_color = Some(Color32::BLACK);
    });
}

pub(super) fn conf_style_panel_top_begin(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(FONT_SIZE_MAIN_MENU_BAR, Proportional),
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(FONT_SIZE_MAIN_MENU_BAR, Proportional),
        );
    });
}

pub(super) fn conf_style_panel_top_end(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(FONT_SIZE_DEFAULT, Proportional),
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(FONT_SIZE_DEFAULT, Proportional),
        );
    });
}

pub(super) fn replace_fonts(ctx: &egui::Context) {
    // Start with the default fonts
    let mut fonts = egui::FontDefinitions::default();

    // Install custom font
    fonts.font_data.insert(
        "inconsolata".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/font/Inconsolata-Medium.ttf"
        ))),
    );

    // Put custom font first (highest priority)
    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(0, "inconsolata".to_owned());

    fonts
        .families
        .entry(Monospace)
        .or_default()
        .insert(0, "inconsolata".to_owned());

    // Tell egui to use these fonts
    ctx.set_fonts(fonts);
}
