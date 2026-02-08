use egui::{pos2, Color32, Pos2};

pub const FONT_SIZE_DEFAULT: f32 = 20.0;
pub const FONT_SIZE_MAIN_MENU_BAR: f32 = 16.0;

pub const COLOR_ERROR: Color32 = Color32::from_rgb(211, 1, 2);
pub const COLOR_ERROR_HIGHLIGHT: Color32 = Color32::from_rgba_unmultiplied_const(211, 1, 2, 80);

// Canvas
pub const COLOR_CANVAS_BACKGROUND: Color32 = Color32::from_gray(240);

pub const SCROLLING_DEFAULT: Pos2 = pos2(5.0, 5.0);

pub const GRID_STEP_BASE: f32 = 100.0;
pub const COLOR_GRID_LINE: Color32 = Color32::from_rgba_unmultiplied_const(200, 200, 200, 40);

pub const CANVAS_FONT_SIZE_BASE: i32 = 18;
pub const CANVAS_FONT_SIZE_STEP: i32 = 4;
pub const CANVAS_FONT_SIZE_MIN: i32 = 6;
pub const CANVAS_FONT_SIZE_MAX: i32 = 30;

pub const NODE_BORDER_OFFSET_BASE: f32 = 18.0;
