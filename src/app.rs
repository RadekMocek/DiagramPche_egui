use crate::config;
use crate::gui::modal::ActionAfterExport;
use crate::gui::panel_top::ActionAfterUnsavedWarn;
use crate::gui::text_editor_alt::AltEditorConfig;
use crate::gui::window::PreferencesTab;
use crate::logic::app_benchmark::BenchmarkData;
use crate::logic::app_file::FileExampleId;
use crate::logic::svg_exporter::Exporter;
use crate::logic::toml::parser::Parser;
use crate::model::canvas_node::CanvasNode;
use crate::model::draw_command::command::DrawCommandOrd;
use crate::model::node_type::NodeType;
use std::collections::{BinaryHeap, HashMap};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
/*
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
*/
pub struct App {
    // Central panel
    pub central_split_ratio: f32, // Textedit on left, canvas on right; this is ratio if their widths user can change
    // Saving
    pub is_source_dirty: bool, // Are there any unsaved changes to the source?
    pub source_filename: Option<String>,
    pub is_action_unsavedwarn_queued: bool, // Did user click something other than cancel in unsavedwarn modal?
    pub do_action_unsavedwarn_save: bool,   // Did user click save in unsavedwarn modal?
    pub action_unsavedwarn_type: ActionAfterUnsavedWarn, // What to do after unsavedwarn modal is processed
    pub action_unsavedwarn_value: FileExampleId, // What file to open if ActionAfterUnsavedWarn is opening an example
    pub should_window_really_close: bool, // Once we set this to true, the app will be definitely closed
    // Text editor
    pub source: String, // Text editor content, the TOML source code that user writes
    pub parser: Parser, // Parses the source into collections of structs which then our app uses to draw the diagram
    pub is_error_span_some: bool, // Did the parser encounter any error?
    pub error_span_line: u32, // On which line of source is the error (we have to compute this)
    pub error_span_column: u32, // At which column of the particular line does the error start (we have to compute this)
    pub error_span_length: u32, // How many chars from the error start should be highlighted (we have to compute this)
    pub editor_cursor_line: usize, // Cursor position is shown in the toolbar
    pub editor_cursor_column: usize,
    pub do_use_alt_editor: bool, // Wether to use 3rd party text editor widget
    pub alt_editor_config: AltEditorConfig,
    pub do_syntax_highlight: bool,
    pub source_font_size: u32,
    // Canvas
    pub canvas_font_size: u32, // Zoom level is based on this
    pub zoom_level: f32,       // Makes rendered diagram smaller/bigger
    pub scrolling: egui::Pos2, // How was the canvas moved by dragging
    pub canvas_nodes: HashMap<String, CanvasNode>, // Storing info about rendered nodes for references etc. to work
    pub draw_commands_ord: BinaryHeap<DrawCommandOrd>, // Commands for egui painter to do the drawing
    pub do_show_grid: bool,                            // Show canvas grid
    // - primary canvas toolbar related
    pub selected_or_hovered_canvas_node_key: Option<String>,
    pub is_canvas_node_selected: bool,
    pub selected_canvas_node_key: String,
    // - secondary canvas toolbar related
    pub is_dragndropping_node: bool,
    pub dragndropping_node_type: NodeType,
    // Toolbar
    pub do_show_toolbar: bool,
    pub do_show_secondary_canvas_toolbar: bool,
    // Non-main window
    pub do_open_modal_about: bool,  // Show the Help → About window
    pub do_open_modal_export: bool, // Show the File → Export to SVG window
    pub do_open_modal_error: bool,  // Shows error message when something went wrong
    pub modal_error_message: String,
    pub do_open_modal_unsavedwarn: bool, // Warns the user when they have unsaved changes and try to discard them, allows to save the document
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
    // Theme
    pub style_is_light_mode: bool,
    pub style_do_force_light_canvas: bool,
    // Benchmark
    pub do_show_window_benchmark: bool,
    pub benchmark_data: BenchmarkData,
    // Misc
    pub no_node_hovered_string: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Central panel
            central_split_ratio: 0.5,
            // Saving
            is_source_dirty: false,
            source_filename: None,
            is_action_unsavedwarn_queued: false,
            do_action_unsavedwarn_save: false,
            action_unsavedwarn_type: ActionAfterUnsavedWarn::Invalid,
            action_unsavedwarn_value: FileExampleId::ExampleBlockDiag,
            should_window_really_close: false,
            // Text editor
            source: String::from(config::WELCOME_TOML),
            parser: Parser::default(),
            is_error_span_some: false,
            error_span_line: 0,
            error_span_column: 0,
            error_span_length: 0,
            editor_cursor_line: 0,
            editor_cursor_column: 0,
            do_use_alt_editor: true,
            alt_editor_config: AltEditorConfig::default(),
            do_syntax_highlight: true,
            source_font_size: config::FONT_SIZE_SOURCE_DEFAULT,
            // Canvas
            canvas_font_size: config::FONT_SIZE_CANVAS_BASE,
            zoom_level: config::ZOOM_LEVEL_DEFAULT,
            scrolling: config::SCROLLING_DEFAULT,
            canvas_nodes: HashMap::new(),
            draw_commands_ord: BinaryHeap::new(),
            do_show_grid: true,
            // - primary canvas toolbar related
            selected_or_hovered_canvas_node_key: None,
            is_canvas_node_selected: false,
            selected_canvas_node_key: String::new(),
            // - secondary canvas toolbar related
            is_dragndropping_node: false,
            dragndropping_node_type: NodeType::Rectangle,
            // Toolbar
            do_show_toolbar: true,
            do_show_secondary_canvas_toolbar: true,
            // Non-main window
            do_open_modal_about: false,
            do_open_modal_export: false,
            do_open_modal_error: false,
            modal_error_message: String::from(""),
            do_open_modal_unsavedwarn: false,
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
            // Theme
            style_is_light_mode: true,
            style_do_force_light_canvas: true,
            // Benchmark
            do_show_window_benchmark: config::DO_OPEN_BENCHMARK_WINDOW_AT_STARTUP,
            benchmark_data: BenchmarkData::default(),
            // Misc
            no_node_hovered_string: String::from("(No node hovered)"),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui
        crate::style::change_appearance_theme(&cc.egui_ctx, true);
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
                // UTF-8 shenanigans
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
                        } else if ch == '\t' {
                            column += 4
                        } else {
                            column += 1;
                        }
                    } else {
                        if i >= error_span.end {
                            break;
                        }
                        column_end += if ch == '\t' { 4 } else { 1 };
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

        // Post modal actions
        if self.is_action_unsavedwarn_queued {
            self.is_action_unsavedwarn_queued = false;
            // Should we really do the action?
            let mut do_the_action = true;
            if self.do_action_unsavedwarn_save {
                // If user pressed cancel on save dialog or saving somehow failed, we won't do the action
                do_the_action = self.handle_regular_save();
            }
            if do_the_action {
                match self.action_unsavedwarn_type {
                    ActionAfterUnsavedWarn::Invalid => {
                        self.show_error_modal("ActionAfterUnsavedWarn::Invalid")
                    }
                    ActionAfterUnsavedWarn::Exit => {
                        self.should_window_really_close = true;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    ActionAfterUnsavedWarn::New => {
                        self.handle_regular_new();
                    }
                    ActionAfterUnsavedWarn::OpenFile => {
                        self.handle_regular_open();
                    }
                    ActionAfterUnsavedWarn::LoadExample => {
                        self.load_source_from_example(self.action_unsavedwarn_value.clone());
                    }
                }
            }
            self.action_unsavedwarn_type = ActionAfterUnsavedWarn::Invalid;
        }

        // Handle native window close pressed or Alt+F4 pressed (cancel the close and show modal if source is dirty)
        if !self.should_window_really_close // If `should_window_really_close` is true, skip modal and definitely close the app
            && self.is_source_dirty
            && ctx.input(|i| i.viewport().close_requested())
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.action_unsavedwarn_type = ActionAfterUnsavedWarn::Exit;
            self.do_open_modal_unsavedwarn = true;
        }

        // Update window title
        let mut native_window_title = String::from("");
        if self.is_source_dirty {
            native_window_title.push('*');
        }
        if let Some(source_filename) = &self.source_filename {
            native_window_title.push_str(source_filename);
        } else {
            native_window_title.push_str("Untitled");
        }
        native_window_title.push_str(" – DiagramPche :: egui");
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(native_window_title));
    }

    /*
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    */
}
