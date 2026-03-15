fn pathbuf_to_string(result: Option<std::path::PathBuf>) -> Option<String> {
    let Some(result_path) = result else {
        return None;
    };

    let Some(result_path_str) = result_path.to_str() else {
        return None;
    };

    Some(String::from(result_path_str))
}

pub fn save_svg_dialog() -> Option<String> {
    pathbuf_to_string(
        rfd::FileDialog::new()
            .add_filter("Scalable Vector Graphics", &["svg"])
            .set_file_name("diagram.svg")
            .set_title("Export to SVG")
            .save_file(),
    )
}

pub fn save_toml_dialog() -> Option<String> {
    pathbuf_to_string(
        rfd::FileDialog::new()
            .add_filter("Diagram", &["toml"])
            .set_file_name("diagram.toml")
            .set_title("Save diagram to file")
            .save_file(),
    )
}

pub fn open_toml_dialog() -> Option<String> {
    pathbuf_to_string(
        rfd::FileDialog::new()
            .add_filter("Diagram", &["toml"])
            .set_title("Open diagram file")
            .pick_file(),
    )
}
