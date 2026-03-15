use crate::config;
use FontFamily::{Monospace, Proportional};
use egui::{Color32, FontFamily, FontId, TextStyle};
use std::collections::BTreeMap;
use eframe::epaint::AlphaFromCoverage;

pub(super) fn conf_style_init(ctx: &egui::Context) {
    let text_styles: BTreeMap<TextStyle, FontId> = [
        (
            TextStyle::Heading,
            FontId::new(config::FONT_SIZE_DEFAULT + 9.0, Proportional),
        ),
        (
            TextStyle::Name("Heading2".into()),
            FontId::new(config::FONT_SIZE_DEFAULT + 7.0, Proportional),
        ),
        (
            TextStyle::Name("Context".into()),
            FontId::new(config::FONT_SIZE_DEFAULT + 5.0, Proportional),
        ),
        (
            TextStyle::Body,
            FontId::new(config::FONT_SIZE_DEFAULT, Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(config::FONT_SIZE_DEFAULT, Monospace),
        ),
        (
            TextStyle::Button,
            FontId::new(config::FONT_SIZE_DEFAULT, Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(config::FONT_SIZE_DEFAULT - 8.0, Proportional),
        ),
    ]
    .into();

    ctx.all_styles_mut(|style| {
        // Set font sizes defined above
        style.text_styles = text_styles.clone();
    });
}

pub(super) fn conf_style_panel_top_begin(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(config::FONT_SIZE_MAIN_MENU_BAR, Proportional),
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(config::FONT_SIZE_MAIN_MENU_BAR, Proportional),
        );
    });
}

pub(super) fn conf_style_panel_top_end(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(config::FONT_SIZE_DEFAULT, Proportional),
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(config::FONT_SIZE_DEFAULT, Proportional),
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

// This should be called inside `ui.scope`
pub(super) fn set_unsavedwarn_modal_button_colors(ui: &mut egui::Ui, is_good: bool) {
    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = if is_good {
        config::COLOR_BTN_GOOD_NORMAL
    } else {
        config::COLOR_BTN_BAD_NORMAL
    };
    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = if is_good {
        config::COLOR_BTN_GOOD_HOVER
    } else {
        config::COLOR_BTN_BAD_HOVER
    };
    ui.style_mut().visuals.widgets.active.weak_bg_fill = if is_good {
        config::COLOR_BTN_GOOD_CLICK
    } else {
        config::COLOR_BTN_BAD_CLICK
    };
}

pub(super) fn change_appearance_theme(ctx: &egui::Context, is_light: bool) {
    if is_light {
        ctx.set_visuals(egui::Visuals::light());
        ctx.all_styles_mut(|style| {
            // Black font instead of dark gray
            style.visuals.override_text_color = Some(Color32::BLACK);
        });
    } else {
        ctx.set_visuals(egui::Visuals::dark());
        ctx.all_styles_mut(|style| {
            // Dark mode default makes the font blurry...
            style.visuals.text_alpha_from_coverage = AlphaFromCoverage::LIGHT_MODE_DEFAULT;
            // Lighter font
            style.visuals.override_text_color = Some(Color32::from_gray(233));
        });
    }
}
