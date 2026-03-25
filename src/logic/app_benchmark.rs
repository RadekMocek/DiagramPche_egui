use crate::App;
use crate::config;
use crate::gui::widget;
use crate::logic::app_file::FileExampleId;
use std::cmp::PartialEq;

#[derive(Clone, PartialEq, Default)]
pub enum BenchmarkType {
    #[default]
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

#[derive(Default)]
pub struct BenchmarkData {
    pub type_choice_idx: usize,
    pub is_running: bool,
    pub running_type: BenchmarkType,
    pub time_counter: f32,
    pub _node_counter_total_pairs: u32,
    pub _node_counter_row_pairs: u32,
    pub _x_cor: u32,
    pub _y_cor: u32,
    pub _color: (u8, u8, u8),
    // In Dear ImGui this is used to change the zoom level, here we change it differently,
    // but still need this value to change colors and to know when to log to CSV.
    pub _notional_zoom_level: u32,
}

// (In benchmark type GRADUAL, nodes are being added to the canvas (they are added as pairs connected by arrow))
// (While benchmarking, we also scroll and zoom, so we have some movement)
// (Benchmarks type LIGHT and HEAVY have prepared TOML and just do scroll and zoom above them)
// What percentage of the window's width will the text editor occupy during the benchmark
const TEXTEDIT_WIDTH_RATIO: f32 = 0.28;
// After this passes, add new batch of nodes
const TIME_INTERVAL: f32 = 0.3; // s
// How many nodes to add in a batch?
const N_NODES_IN_INTERVAL: u32 = 35;
// Add this to each node x coordinate
const X_COR_ADDITION: u32 = 12;
// How many Z layers we use? Each node has Z one greater than the previous, moduled by this
const Z_MODULO: u32 = crate::helper::draw_layer::N_DL_USER_CHANNELS as u32;
// How many nodes on a row we want?
const MAX_NODES_ON_ROW: u32 = 220;
// When we reach `MAX_NODES_ON_ROW`, we go on a new row, this is the offset of the new row
const Y_COR_ADDITION: u32 = 100;
// How many rows do we want? When we have this much of rows, benchmark ends
const MAX_ROWS: u32 = 21;
// Used for the ending condition
const MAX_Y_COR: u32 = Y_COR_ADDITION * MAX_ROWS;
// (While benchmarking, we also scroll and zoom, so we have some movement)
// Amount of scrolling right after each node batch added
const AUTO_SCROLL_STEP_X: f32 = 10.0;
// When to wrap to the beggining with the scrolling
const AUTO_SCROLL_MODULO_X: f32 = 600.0;
// How many zoom levels we iterate, this corresponds to the slider and MW behavior
const ZOOM_LEVEL_MODULO: u32 = 6;
// Precalculated
const BENCHMARK_LIGHT_N_NODES: u32 = 12;
const BENCHMARK_HEAVY_N_NODES: u32 = 10780;

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
        self.benchmark_data._node_counter_total_pairs = 0;
        self.benchmark_data._node_counter_row_pairs = 0;
        self.benchmark_data._x_cor = 0;
        self.benchmark_data._y_cor = 0;
        self.benchmark_data._color = (255, 255, 255);
        self.benchmark_data._notional_zoom_level = 0;
    }

    pub fn benchmark_update(&mut self, ctx: &egui::Context) {
        // Get delta time from egui
        self.benchmark_data.time_counter += ctx.input(|i| i.unstable_dt);

        // Do the next batch only when certain amount of time has passed
        if self.benchmark_data.time_counter > TIME_INTERVAL {
            self.benchmark_data.time_counter -= TIME_INTERVAL;

            let old_nzm = self.benchmark_data._notional_zoom_level;
            self.benchmark_data._notional_zoom_level = (old_nzm + 1) % ZOOM_LEVEL_MODULO;

            // Zoom frenzy
            self.canvas_font_size += config::FONT_SIZE_CANVAS_STEP;
            if self.canvas_font_size > config::FONT_SIZE_CANVAS_MAX {
                self.canvas_font_size = config::FONT_SIZE_CANVAS_MIN;
            }
            self.update_canvas_zoom();

            // Add a new batch of nodes
            for i in 0..N_NODES_IN_INTERVAL {
                if self.benchmark_data.running_type == BenchmarkType::Gradual {
                    let t = self.benchmark_data._node_counter_total_pairs;
                    let x = self.benchmark_data._x_cor;
                    let y = self.benchmark_data._y_cor;
                    let z = self.benchmark_data._node_counter_row_pairs % Z_MODULO;
                    let (r, g, b) = self.benchmark_data._color;
                    self.source.push_str(&format!(
                        "[node.\"A{t}\"]\nxy=[{x},{y}]\nz={z}\ncolor=[{r},{g},{b},128]\n\
                        [node.\"B{t}\"]\nxy=[\"A{t}\",\"bottom-right\",10,10]\nz={z}\ntype=\"ellipse\"\n\
                        [[path]]\nstart=[\"A{t}\",\"left\",0,0]\nend=[\"B{t}\",\"right\",0,0]\n",
                    ));
                }
                // Update values for next iteration
                self.benchmark_data._node_counter_total_pairs += 1;
                self.benchmark_data._node_counter_row_pairs += 1;
                self.benchmark_data._x_cor += X_COR_ADDITION;
                benchmark_change_color(
                    &mut self.benchmark_data._color,
                    self.benchmark_data._notional_zoom_level,
                );
            }

            // Auto scrolling
            self.scrolling.x -= AUTO_SCROLL_STEP_X;
            if self.scrolling.x < -AUTO_SCROLL_MODULO_X {
                self.scrolling.x = 0.0;
            }

            // Jump to new row if needed
            if self.benchmark_data._node_counter_row_pairs > MAX_NODES_ON_ROW {
                self.benchmark_data._node_counter_row_pairs = 0;
                self.benchmark_data._x_cor = 0;
                self.benchmark_data._y_cor += Y_COR_ADDITION;
            }

            // Stats
            //todo

            // End the benchmark check
            if self.benchmark_data._y_cor > MAX_Y_COR {
                self.benchmark_data.is_running = false;
            }
        }
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

// === === === === === === === === === === === === === === === === === === === === === ===

fn benchmark_change_color_impl(r: &mut u8, g: &mut u8, b: &mut u8) {
    if *r > 0 {
        *r -= 1;
    } else if *g > 0 {
        *g -= 1;
    } else if *b > 0 {
        *b -= 1;
    } else {
        *r = 255;
        *g = 255;
        *b = 255;
    }
}

fn benchmark_change_color(color: &mut (u8, u8, u8), modifier_0_to_5: u32) {
    match modifier_0_to_5 {
        1 => benchmark_change_color_impl(&mut color.0, &mut color.2, &mut color.1),
        2 => benchmark_change_color_impl(&mut color.1, &mut color.0, &mut color.2),
        3 => benchmark_change_color_impl(&mut color.1, &mut color.2, &mut color.0),
        4 => benchmark_change_color_impl(&mut color.2, &mut color.0, &mut color.1),
        5 => benchmark_change_color_impl(&mut color.2, &mut color.1, &mut color.0),
        _ => benchmark_change_color_impl(&mut color.0, &mut color.1, &mut color.2),
    }
}
