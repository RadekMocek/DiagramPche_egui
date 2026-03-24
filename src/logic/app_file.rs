use crate::App;
use crate::gui::panel_top::ActionAfterUnsavedWarn;
use crate::logic::app_dialog::{open_toml_dialog, save_toml_dialog};

#[derive(Clone)]
pub enum FileExampleId {
    ExampleBlockDiag,
    ExampleEcoDiag,
    DebugZAxis,
    DebugPathLabel,
    DebugBenchLight,
    DebugBenchHeavy,
}

impl App {
    // == Logic for buttons in MainMenuBar ==========================================

    pub fn handle_regular_new(&mut self) {
        self.source.clear();
        self.source_filename = None;
        self.is_source_dirty = false;
    }

    pub fn handle_regular_open(&mut self) {
        if let Some(path) = open_toml_dialog() {
            self.load_source_from_file(&path);
        }
    }

    pub fn handle_regular_save(&mut self) -> bool {
        let source_filename = self.source_filename.clone();

        // (This whole if let else acts as a return)
        if let Some(source_filename) = source_filename {
            if self.save_source_to_file(&source_filename) {
                self.is_source_dirty = false;
                return true;
            }
            false
        } else {
            self.save_source_to_file_from_dialog()
        }
    }

    pub fn handle_open_example(&mut self, id: FileExampleId) {
        if !self.is_source_dirty {
            self.load_source_from_example(id);
        } else {
            self.action_unsavedwarn_type = ActionAfterUnsavedWarn::LoadExample;
            self.action_unsavedwarn_value = id;
            self.do_open_modal_unsavedwarn = true;
        }
    }

    // == Underlying logic ==========================================================

    pub fn load_source_from_file(&mut self, filename: &str) {
        let result = std::fs::read_to_string(filename);

        if let Ok(content) = result {
            self.source = content.replace("\r\n", "\n");
            self.reset_canvas_scrolling_and_zoom();
            self.source_filename = Some(String::from(filename));
            self.is_source_dirty = false;
        } else if let Err(err) = result {
            self.show_error_modal(&err.to_string());
        }
    }

    /// Saves `self.source` to `filename`, returns true if successful
    pub fn save_source_to_file(&mut self, filename: &str) -> bool {
        if let Err(err) = std::fs::write(filename, &self.source) {
            self.show_error_modal(&err.to_string());
            false
        } else {
            true
        }
    }

    pub fn save_source_to_file_from_dialog(&mut self) -> bool {
        if let Some(path) = save_toml_dialog() {
            if self.save_source_to_file(&path) {
                self.source_filename = Some(path);
                self.is_source_dirty = false;
                return true;
            }
        }
        false
    }

    pub fn load_source_from_example(&mut self, id: FileExampleId) {
        // Not good solution, but right now I don't want to spend time making build scripts to copy files to target directory
        let cow = match id {
            FileExampleId::ExampleBlockDiag => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/Example1.toml"))
            }
            FileExampleId::ExampleEcoDiag => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/Example2.toml"))
            }
            FileExampleId::DebugZAxis => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/debug/Z-axis.toml"))
            }
            FileExampleId::DebugPathLabel => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/debug/PathLabel.toml"))
            }
            FileExampleId::DebugBenchLight => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/debug/BenchmarkLight.toml"))
            }
            FileExampleId::DebugBenchHeavy => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/debug/BenchmarkHeavy.toml"))
            }
        };

        self.source = cow.to_string().replace("\r\n", "\n");
        self.reset_canvas_scrolling_and_zoom();
        self.source_filename = None;
        self.is_source_dirty = false;
    }
}

// === === === === === === === === === === === === === === === === === === === ===

pub fn get_default_svg_path() -> String {
    let Ok(path) = std::env::current_dir() else {
        return String::from("");
    };

    String::from(path.join("diagram.svg").to_str().unwrap_or(""))
}

/// Tells OS to open a file at `filename` path. Used to open SVG file after SVG export, if user wants that.
pub fn open_file(filename: &str) -> std::io::Result<()> {
    open::that(filename)
}
