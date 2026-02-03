use crate::logic::parser::Parser;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/*
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
*/
pub struct App {
    // Text editor
    pub source: String,
    pub parser: Parser,
    // Canvas
    pub zoom_level: f32,
    pub is_canvas_dragged: bool,
    pub scrolling: egui::Pos2,
    // Non-main window
    pub do_open_modal_about: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Text editor
            parser: Parser::default(),

            source: String::from(
                r#"[variables]
w = 110
h = 72

[node.test]
xy = [70, 70]
                "#,
            ),

            /*
            source: String::from(
                r#"[variables]
w = 110
h = 72

[node."0,0"]

[node.Cache]
xy = [70, 70]
size = ["w", "h"]

[node.ALU]
pivot = "top"
xy = ["Cache", "bottom", 0, 35]
size = ["w", "h"]
z = 6

[node."Řídící\njednotka"]
pivot = "top"
xy = ["ALU", "bottom", 0, 35]
size = ["w", "h"]

[node."Datové\nregistry"]
pivot = "left"
xy = ["ALU", "right", 35, 0]
size = ["w", "h"]

[node."Stavové\nregistry"]
pivot = "left"
xy = ["Řídící\njednotka", "right", 35, 0]
size = ["w", "h"]

[[path]]
start=["Cache", "left", 0, 0]
ends=[
  ["ALU", "left", 0, 0],
  ["Řídící\njednotka", "bottom",0,0]
]
shift = 25
points=[
  ["", "start", -25, "", "", 5],
  ["Datové\nregistry", "top", 0, "", "", 5],
  ["", "prev", 0, "", "end", 0]
]
tips="<>"

[[path]]
start=["Cache","top-left",0,0]
end=["Cache","bottom-right",0,0]
color=[150,0,0,255]
tips="<>"

[[path]]
start=[400,400]
end=[400,500]
points=[
  ["","",500,"","",400],
  ["","",500,"","",500],
]
tips="<>""#,
            ),*/
            // Canvas
            zoom_level: 1.0,
            is_canvas_dragged: false,
            scrolling: egui::pos2(0.0, 0.0),
            // Non-main window
            do_open_modal_about: false,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        crate::style::conf_style_init(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
        */
        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        self.parser.parse(&self.source);
        self.gui_panel_top(&ctx);
        self.gui_panel_bottom(&ctx);
        self.gui_panel_central(&ctx); // Central called after bottom oterwise bottom would cover a little bit of central
    }

    /*
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    */
}
