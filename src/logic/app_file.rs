use crate::App;

impl App {
    pub fn load_source_from_example(&mut self) {
        let bytes = include_bytes!("../../assets/example/Example1.toml");
        self.source = String::from_utf8_lossy(bytes)
            .to_string()
            .replace("\r\n", "\n");
    }
}
