pub struct BenchmarkData {
    pub is_benchmark_running: bool,
}

impl Default for BenchmarkData {
    fn default() -> Self {
        Self {
            is_benchmark_running: false,
        }
    }
}
