use crate::App;
use crate::logic::app_dialog::save_toml_dialog;

impl App {
    pub fn load_source_from_example(&mut self, magic_string: &str) {
        // Not good solution, but right now I don't want to spend time making build scripts to copy files to target directory
        let cow = match magic_string {
            "example1" => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/Example1.toml"))
            }
            "example2" => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/Example2.toml"))
            }
            "debug1" => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/debug/Z-axis.toml"))
            }
            unknown => {
                self.show_error_modal(&format!("There is no example with key '{unknown}'."));
                return;
            }
        };

        self.source = cow.to_string().replace("\r\n", "\n");

        self.reset_canvas_scrolling_and_zoom();
    }

    pub fn save_source_to_file(&mut self, filename: &str) -> bool {
        if let Err(err) = std::fs::write(filename, &self.source) {
            self.show_error_modal(&err.to_string());
            false
        } else {
            true
        }
    }

    pub fn save_source_to_file_from_dialog(&mut self) {
        if let Some(path) = save_toml_dialog() {
            if self.save_source_to_file(&path) {
                self.source_filename = path;
                self.is_source_dirty = false;
            }
        }
    }
}

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
