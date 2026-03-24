use egui::{Color32, Pos2, pos2};

pub const DO_OPEN_BENCHMARK_WINDOW_AT_STARTUP: bool = true;

//
pub const FONT_SIZE_DEFAULT: f32 = 20.0;
pub const FONT_SIZE_MAIN_MENU_BAR: f32 = 18.0;

pub const FONT_SIZE_SOURCE_DEFAULT: u32 = 20;
pub const FONT_SIZE_SOURCE_MIN: u32 = 8;
pub const FONT_SIZE_SOURCE_MAX: u32 = 40;

pub const COLOR_ERROR: Color32 = Color32::from_rgb(211, 1, 2);
pub const COLOR_ERROR_HIGHLIGHT: Color32 = Color32::from_rgba_unmultiplied_const(211, 1, 2, 80);

pub const COLOR_BTN_GOOD_NORMAL: Color32 = Color32::from_rgba_unmultiplied_const(192, 230, 0, 128);
pub const COLOR_BTN_GOOD_HOVER: Color32 = Color32::from_rgba_unmultiplied_const(183, 220, 0, 128);
pub const COLOR_BTN_GOOD_CLICK: Color32 = Color32::from_rgba_unmultiplied_const(138, 165, 0, 128);
pub const COLOR_BTN_BAD_NORMAL: Color32 = Color32::from_rgba_unmultiplied_const(230, 69, 69, 128);
pub const COLOR_BTN_BAD_HOVER: Color32 = Color32::from_rgba_unmultiplied_const(220, 66, 66, 128);
pub const COLOR_BTN_BAD_CLICK: Color32 = Color32::from_rgba_unmultiplied_const(165, 49, 49, 128);

// Canvas
pub const COLOR_CANVAS_BACKGROUND: Color32 = Color32::from_gray(240);

pub const SCROLLING_DEFAULT: Pos2 = pos2(5.0, 5.0);
pub const ZOOM_LEVEL_DEFAULT: f32 = 1.0;

pub const GRID_STEP_BASE: f32 = 100.0;
pub const COLOR_GRID_LINE: Color32 = Color32::from_rgba_unmultiplied_const(200, 200, 200, 40);

pub const COLOR_GHOST_EDGE: Color32 = Color32::from_rgba_unmultiplied_const(0, 0, 0, 128);
pub const COLOR_GHOST_FILL: Color32 = Color32::from_rgba_unmultiplied_const(255, 255, 255, 128);

pub const FONT_SIZE_CANVAS_BASE: u32 = 18;
pub const FONT_SIZE_CANVAS_STEP: u32 = 4;
pub const FONT_SIZE_CANVAS_MIN: u32 = 6;
pub const FONT_SIZE_CANVAS_MAX: u32 = 30;

pub const NODE_BORDER_OFFSET_BASE: f32 = 18.0;

pub const TIP_ARROW_LENGTH: f32 = 12.0;
pub const TIP_ARROW_SPAN: f32 = 4.0;

//
pub const WELCOME_TOML: &str = r##"[variables]
node_offset = 14
path_offset = 40

[node.hello]
value = "Hello,"
color = "#ff99b880"
type = "ellipse"

[node.world]
value = "world!"
color = "#ffcfb380"
pivot = "top-left"
xy = ["hello", "bottom-right", "node_offset", "node_offset"]
type = "diamond"

[[path]]
start = ["hello", "bottom", 0, 0]
end = ["world", "bottom", 0, 0]
shift = [0, "path_offset"]
points = [["", "start", 0, "", "end", 0]]
color = [40, 40, 40, 255]
"##;
