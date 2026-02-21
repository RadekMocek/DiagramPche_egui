pub fn save_svg_dialog(default_path:&str) -> Option<String> {
    let result = rfd::FileDialog::new()
        .add_filter("Scalable Vector Graphics", &["svg"])
        .set_directory(default_path)
        .set_file_name("diagram.svg")
        .set_title("Export to SVG")
        .save_file();

    let Some(result_path) = result else {
        return None;
    };

    let Some(result_path_str) = result_path.to_str() else {
        return None;
    };

    Some(String::from(result_path_str))
}
