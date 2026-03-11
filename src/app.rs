use crate::gui::modal::ActionAfterExport;
use crate::logic::svg_exporter::Exporter;
use crate::logic::toml::parser::Parser;
use crate::model::canvas_node::CanvasNode;
use crate::model::draw_command::command::DrawCommandOrd;
use std::collections::{BinaryHeap, HashMap};
use crate::gui::window::PreferencesTab;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/*
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
*/
pub struct App {
    // Central panel
    pub central_split_ratio: f32, // Textedit on left, canvas on right; this is ratio if their widths user can change
    // Text editor
    pub source: String, // Text editor content, the TOML source code that user writes
    pub parser: Parser, // Parses the source into collections of structs which then our app uses to draw the diagram
    pub is_error_span_some: bool, // Did the parser encounter any error?
    pub error_span_line: u32, // On which line of source is the error (we have to compute this)
    pub error_span_column: u32, // At which column of the particular line does the error start (we have to compute this)
    pub error_span_length: u32, // How many chars from the error start should be highlighted (we have to compute this)
    // Canvas
    pub canvas_font_size: i32,   // Zoom level is based on this
    pub zoom_level: f32,         // Makes rendered diagram smaller/bigger
    pub scrolling: egui::Pos2,   // How was the canvas moved by dragging
    pub canvas_nodes: HashMap<String, CanvasNode>, // Storing info about rendered nodes for references etc. to work
    pub draw_commands_ord: BinaryHeap<DrawCommandOrd>, // Commands for egui painter to do the drawing
    pub do_show_grid: bool,                            // Show canvas grid
    // Non-main window
    pub do_open_modal_about: bool,  // Show the Help → About window
    pub do_open_modal_export: bool, // Show the File → Export to SVG window
    pub do_open_modal_error: bool,  // Shows error message when something went worng
    pub modal_error_message: String,
    // SVG export
    // - svg logic
    pub svg_exporter: Exporter,
    pub do_svg_export_this_iter: bool,
    // - svg modal
    pub modal_export_path: String,
    pub modal_export_do_overwrite: bool,
    pub modal_export_action_choice: ActionAfterExport,
    // Modeless windows
    pub do_show_window_preferences: bool,
    pub window_preferences_selected_tab: PreferencesTab,
    // Misc
    pub font_char_size: egui::Vec2, // Cache for error highlight, how big is the character (counting on monospace font)
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Central panel
            central_split_ratio: 0.5,
            // Text editor
            source: String::from(crate::config::WELCOME_TOML),
            parser: Parser::default(),
            is_error_span_some: false,
            error_span_line: 0,
            error_span_column: 0,
            error_span_length: 0,
            // Canvas
            canvas_font_size: crate::config::CANVAS_FONT_SIZE_BASE,
            zoom_level: crate::config::ZOOM_LEVEL_DEFAULT,
            scrolling: crate::config::SCROLLING_DEFAULT,
            canvas_nodes: HashMap::new(),
            draw_commands_ord: BinaryHeap::new(),
            do_show_grid: true,
            // Non-main window
            do_open_modal_about: false,
            do_open_modal_export: false,
            do_open_modal_error: false,
            modal_error_message: String::from(""),
            // SVG export
            // - svg logic
            svg_exporter: Exporter::default(),
            do_svg_export_this_iter: false,
            // - svg modal
            modal_export_path: crate::logic::app_file::get_default_svg_path(),
            modal_export_do_overwrite: false,
            modal_export_action_choice: ActionAfterExport::DoNothing,
            // Modeless windows
            // - preferences
            do_show_window_preferences: false,
            window_preferences_selected_tab: PreferencesTab::Appearance,
            // Misc
            font_char_size: egui::Vec2::ZERO,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        crate::style::replace_fonts(&cc.egui_ctx);
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

    /// (Expecting monospace font)
    pub fn update_font_char_size(&mut self, ui: &egui::Ui) {
        self.font_char_size = ui
            .painter()
            .layout_no_wrap(
                String::from("A"),
                egui::FontId::new(
                    crate::config::FONT_SIZE_DEFAULT,
                    eframe::epaint::FontFamily::Monospace,
                ),
                egui::Color32::PLACEHOLDER,
            )
            .rect
            .size();
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // Parse the TOML
        self.parser.parse(&self.source);

        // Parse the error
        self.is_error_span_some = false;
        if self.parser.is_error {
            // Convert TOML error span into line no. and column no. so it can be drawn onto a textedit
            if let Some(error_span) = &self.parser.error_span {
                let mut line = 0;
                let mut column = 0;
                let mut column_end = 0;
                let mut is_end_column_processed = false;
                // UTF8 shenanigans
                for (i, ch) in self.source.char_indices() {
                    if !is_end_column_processed {
                        if i >= error_span.start {
                            column_end = column;
                            is_end_column_processed = true;
                            continue;
                        }
                        if ch == '\n' {
                            line += 1;
                            column = 0;
                        } else {
                            column += 1;
                        }
                    } else {
                        if i >= error_span.end {
                            break;
                        }
                        column_end += 1;
                    }
                }
                self.error_span_line = line;
                self.error_span_column = column;
                self.error_span_length = column_end - column + 1;
                self.is_error_span_some = true;
            }
        }

        // Draw GUI
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
