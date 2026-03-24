use crate::App;
use crate::gui::widget;
use crate::logic::app_file::FileExampleId;
use std::cmp::PartialEq;

#[derive(Clone, PartialEq)]
pub enum BenchmarkType {
    Light,
    Heavy,
    Gradual,
}

impl BenchmarkType {
    fn from_choice_idx(idx: usize) -> Self {
        match idx {
            0 => Self::Light,
            1 => Self::Heavy,
            2 => Self::Gradual,
            _ => Self::Light,
        }
    }
}

pub struct BenchmarkData {
    pub type_choice_idx: usize,
    pub is_running: bool,
    pub running_type: BenchmarkType,
}

impl Default for BenchmarkData {
    fn default() -> Self {
        Self {
            type_choice_idx: 0,
            is_running: false,
            running_type: BenchmarkType::Light,
        }
    }
}

// (In benchmark type GRADUAL, nodes are being added to the canvas (they are added as pairs connected by arrow))
// (While benchmarking, we also scroll and zoom, so we have some movement)
// (Benchmarks type LIGHT and HEAVY have prepared TOML and just do scroll and zoom above them)
// What percentage of the window's width will the text editor occupy during the benchmark
const TEXTEDIT_WIDTH_RATIO: f32 = 0.28;

impl App {
    // This is called when user presses the 'Start benchmark' button
    pub fn benchmark_start(&mut self, ctx: &egui::Context) {
        // Update state
        self.benchmark_data.is_running = true;
        let btype = BenchmarkType::from_choice_idx(self.benchmark_data.type_choice_idx);
        self.benchmark_data.running_type = btype.clone();

        // Reset the view
        self.reset_canvas_scrolling_and_zoom();

        // Change the ratio between textedit and canvas to make canvas bigger (more things to see)
        self.central_split_ratio = TEXTEDIT_WIDTH_RATIO;

        // Reserve string space
        if btype == BenchmarkType::Gradual {
            self.source.reserve(1_000_000);
        }

        // Maximize the window
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));

        // Prepare the source
        match btype {
            BenchmarkType::Light => self.handle_open_example(FileExampleId::DebugBenchLight),
            BenchmarkType::Heavy => self.handle_open_example(FileExampleId::DebugBenchHeavy),
            BenchmarkType::Gradual => self.handle_regular_new(),
        }

        // Initialize helper variables
    }

    pub fn benchmark_update(&mut self) {
        //todo
    }

    pub fn benchmark_gui_update(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::ProgressBar::new(0.0)
                .animate(true)
                .text("\tBenchmark is running..."),
        );
        ui.separator();
        ui.add_space(widget::TINYSKIP);
        if ui.button("Stop").clicked() {
            self.benchmark_data.is_running = false;
        }
    }
}
