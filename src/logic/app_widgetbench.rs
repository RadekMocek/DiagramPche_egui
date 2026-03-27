use crate::App;
use crate::helper::benchmark_csv::{WidgetbenchLogResults, get_os_id, get_unix_timestamp};
use memory_stats::memory_stats;
use std::time;
pub struct WidgetBenchData {
    pub textedittext: String,
    pub is_running: bool,
    pub do_show_window: bool,
    pub n_batches: u32,
    pub batch_iter: u32,
    pub timestamp_window_queued: time::Instant,
    //
    pub log_results: WidgetbenchLogResults,
}

impl Default for WidgetBenchData {
    fn default() -> Self {
        Self {
            textedittext: String::from("GHCHIJK"),
            is_running: false,
            do_show_window: false,
            n_batches: 1,
            batch_iter: 1,
            timestamp_window_queued: time::Instant::now(),
            //
            log_results: WidgetbenchLogResults::default(),
        }
    }
}

//const WIDGETBENCH_STOP: u32 = 262144;
const WIDGETBENCH_STOP: u32 = 16;

impl App {
    pub fn handle_widgetbench(&mut self, ctx: &egui::Context) {
        // RUNNING
        if self.widgetbench_data.is_running {
            if self.widgetbench_data.do_show_window {
                // Window has been shown, do not show it in the next iteration, in which will set the var to show it again
                self.widgetbench_data.do_show_window = false;
                // --- --- --- --- --- --- --- --- --- --- --- ---
                // LOG N "ROWS"
                self.widgetbench_data
                    .log_results
                    .n_batches
                    .push(self.widgetbench_data.n_batches);
                self.widgetbench_data
                    .log_results
                    .batch_iter
                    .push(self.widgetbench_data.batch_iter);
                // LOG DURATION
                self.widgetbench_data.log_results.duration.push(
                    self.widgetbench_data
                        .timestamp_window_queued
                        .elapsed()
                        .as_millis(),
                );
                // LOG RAM
                if let Some(usage) = memory_stats() {
                    const MIBI: f64 = 1024.0 * 1024.0;
                    self.widgetbench_data
                        .log_results
                        .mem_mib
                        .push(usage.physical_mem as f64 / MIBI);
                }
                // LOG CPU
                self.system_info.refresh_cpu_usage();
                self.widgetbench_data
                    .log_results
                    .cpu_usage
                    .push(self.system_info.global_cpu_usage());
                // --- --- --- --- --- --- --- --- --- --- --- ---
                // Report progress
                self.source = format!(
                    "[node.\"{} {}\"]",
                    self.widgetbench_data.n_batches, self.widgetbench_data.batch_iter
                );
                // Prepare batch for the next iter
                self.widgetbench_data.batch_iter += 1;
                if self.widgetbench_data.batch_iter > 10 {
                    self.widgetbench_data.batch_iter = 0;
                    self.widgetbench_data.n_batches *= 2;
                }
            } else {
                // Widgetbench keep-going condition
                if self.widgetbench_data.n_batches <= WIDGETBENCH_STOP {
                    // This is where the widgetbench starts
                    // We'll set var to show the window next iter
                    self.widgetbench_data.timestamp_window_queued = time::Instant::now();
                    self.widgetbench_data.do_show_window = true;
                    self.system_info.refresh_cpu_usage();
                } else {
                    // This is where the widgetbench ends
                    self.widgetbench_data.is_running = false;
                    // Filename
                    let filename = format!(
                        "./widgetbechres_egui_{}_{}.csv",
                        get_os_id(),
                        get_unix_timestamp()
                    );
                    // Save
                    if let Err(err) = self.widgetbench_data.log_results.write_to_csv(&filename) {
                        self.show_error_modal(&err.to_string());
                    }
                    // Let know
                    self.source = String::from("[node.\"Widget benchmark done\"]");
                }
            }
        }
        // INIT
        if self.is_widgetbench_start_queued {
            self.is_widgetbench_start_queued = false;
            // (Re)set benchmark data
            self.widgetbench_data = WidgetBenchData::default();
            self.widgetbench_data.is_running = true;
            // Maximize the window
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
            // Refresh CPU usage
            self.system_info.refresh_cpu_usage();
        }
    }
}
