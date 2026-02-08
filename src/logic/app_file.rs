use crate::App;

impl App {
    pub fn load_source_from_example(&mut self, magic_string: &str) {
        // Terrible solution, but right now I don't want to spend time making build scripts to copy files to target directory
        let cow = match magic_string {
            "example1" => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/Example1.toml"))
            }
            "debug1" => {
                String::from_utf8_lossy(include_bytes!("../../assets/example/debug/Z-axis.toml"))
            }
            _ => unreachable!(),
        };

        self.source = cow.to_string().replace("\r\n", "\n");
        
        self.scrolling = crate::config::SCROLLING_DEFAULT;
    }
}
