use egui::{Color32, Pos2, pos2};

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

// == Temporary ======================================
pub const SOURCE_INITIAL_VALUE: &str = r##"[variables]
w = 110
h = 72

[node."0,0"]

[node.custom_size]
value = "it is\nwhat\nit is"
xy = ["Datové\nregistry", "top-right", -10, 10]
pivot = "bottom-left"
size = [200, 150]
label_pos = "bottom-right"
z = 3

[node.Cache]
xy = [70, 70]
#size = ["w", "h"]

[node.ALU]
pivot = "top"
xy = ["Cache", "bottom", 0, 35]
#size = ["w", "h"]
z = 6
color = "#006db680"

[node."Řídící\njednotka"]
pivot = "top"
xy = ["ALU", "bottom", 0, 35]
#size = ["w", "h"]

[node."Datové\nregistry"]
pivot = "left"
xy = ["ALU", "right", 35, 0]
#size = ["w", "h"]

[node."Stavové\nregistry"]
pivot = "left"
xy = ["Řídící\njednotka", "right", 35, 0]
#size = ["w", "h"]

[[path]]
start=["Cache", "left", 0, 0]
ends=[
  ["ALU", "left", 0, 0],
  ["Řídící\njednotka", "bottom",0,0]
]
shift = 30
points=[
  ["", "start", -25, "", "start", -15],
  ["Datové\nregistry", "top", 0, "", "", 5],
  ["", "prev", 0, "", "end", 0]
]
tips="<>"

[[path]]
start=["Cache","top-left",0,0]
end=["Cache","bottom-right",0,0]
color=[150,0,0,80]
tips="<>"

[[path]]
start=[400,400]
end=[400,500]
points=[
  ["","",500,"","",400],
  ["","",500,"","",500],
]
tips="<>"

[[path]]
start=["Datové\nregistry","right",0,20]
ends=[
  ["Datové\nregistry","right",200,0],
  ["Datové\nregistry","right",200,15],
  ["Datové\nregistry","right",200,30],
]
points=[["","start",50,"","end",0]]
tips="<-"
"##;
