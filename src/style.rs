use crate::config::*;
use FontFamily::{Monospace, Proportional};
use egui::{Color32, FontFamily, FontId, TextStyle};
use std::collections::BTreeMap;

pub(super) fn conf_style_init(ctx: &egui::Context) {
    let text_styles: BTreeMap<TextStyle, FontId> = [
        (
            TextStyle::Heading,
            FontId::new(FONT_SIZE_DEFAULT + 9.0, Proportional),
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

    // Install custom fonts
    let inconsolata_id = String::from("inconsolata");
    fonts.font_data.insert(
        inconsolata_id.clone(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/font/Inconsolata-Medium.ttf"
        ))),
    );

    let icons_id = String::from("icons");
    fonts.font_data.insert(
        icons_id.clone(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/font/pictogrammers-materialdesignicons.ttf"
        ))),
    );

    // Put custom font first (highest priority)
    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(0, inconsolata_id.clone());

    fonts
        .families
        .entry(Monospace)
        .or_default()
        .insert(0, inconsolata_id.clone());

    // Then icons
    fonts
        .families
        .entry(Proportional)
        .or_default()
        .insert(1, icons_id.clone());

    fonts
        .families
        .entry(Monospace)
        .or_default()
        .insert(1, icons_id.clone());

    // Tell egui to use these fonts
    ctx.set_fonts(fonts);
}
