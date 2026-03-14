use egui::{Color32, Pos2, pos2};

pub const FONT_SIZE_DEFAULT: f32 = 20.0;
pub const FONT_SIZE_MAIN_MENU_BAR: f32 = 18.0;

pub const FONT_SIZE_SOURCE_DEFAULT: u32 = 20;
pub const FONT_SIZE_SOURCE_MIN: u32 = 8;
pub const FONT_SIZE_SOURCE_MAX: u32 = 40;

pub const COLOR_ERROR: Color32 = Color32::from_rgb(211, 1, 2);
pub const COLOR_ERROR_HIGHLIGHT: Color32 = Color32::from_rgba_unmultiplied_const(211, 1, 2, 80);

// Canvas
pub const COLOR_CANVAS_BACKGROUND: Color32 = Color32::from_gray(240);

pub const SCROLLING_DEFAULT: Pos2 = pos2(5.0, 5.0);
pub const ZOOM_LEVEL_DEFAULT: f32 = 1.0;

pub const GRID_STEP_BASE: f32 = 100.0;
pub const COLOR_GRID_LINE: Color32 = Color32::from_rgba_unmultiplied_const(200, 200, 200, 40);

pub const FONT_SIZE_CANVAS_BASE: u32 = 18;
pub const FONT_SIZE_CANVAS_STEP: u32 = 4;
pub const FONT_SIZE_CANVAS_MIN: u32 = 6;
pub const FONT_SIZE_CANVAS_MAX: u32 = 30;

pub const NODE_BORDER_OFFSET_BASE: f32 = 18.0;

pub const TIP_ARROW_LENGTH: f32 = 12.0;
pub const TIP_ARROW_SPAN: f32 = 4.0;

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

# ---------------

[node.testp]
xy = [250, 250]
value = "aaaaa\nbbbbbbbbbbbbb\nccccc\nddddd"
type = "text"

[node.teststh]
xy=[300, 0]
color_border = "#FF0000FF"
label_shift = [-50, -50]
color=[0,
0,
0,
255]

[node.testm]
xy = [-250, -250]
value = "aaaaa\nbbbbb\nccccc\nddddd"
type = "text"

[node.testr]
color = "#FFFFFFC0"
color_border = "#0000FF2F"
pivot = "center"
xy = ["testp", "center", 0, 0]
value = "       "
z = 5
type = "rectangle"

[[path]]
start=["world", "right", 0 ,0]
end=["testp", "left", 0 ,0]
points=[
    ["teststh","bottom",0,"","start",0],
    ["hello","bottom",0,"","prev",200]
]
label=["huh?", 1, 150, 15]
label_bg = [255,255,255,255]

[[path]]
start=["testp", "right", 0,0]
ends=[
    ["teststh", "right", 0,0],
    ["teststh", "bottom", 20,0]
]
shift=[200,100]
label=["AAA\nBBB\nCCC\nDDD", 1, 250, 0]
label_bg = "#00FF008F"
"##;
