use std::time;

#[derive(Default)]
pub struct BenchmarkLogResults {
    pub timestamp: Vec<u128>,
    pub fps: Vec<u32>,
    pub n_nodes: Vec<u32>,
    pub mem_mib: Vec<f64>,
    pub cpu_usage: Vec<f32>,
}

impl BenchmarkLogResults {
    pub fn clear(&mut self) {
        self.timestamp.clear();
        self.fps.clear();
        self.n_nodes.clear();
        self.mem_mib.clear();
        self.cpu_usage.clear();
    }

    pub fn write_to_csv(&self, filename: &str) -> std::io::Result<()> {
        let mut result = String::from("timestamp,fps,n_nodes,mem_mib,cpu_usage\n");

        for i in 0..self.timestamp.len() {
            result.push_str(&format!(
                "{},{},{},{},{}\n",
                self.timestamp[i], self.fps[i], self.n_nodes[i], self.mem_mib[i], self.cpu_usage[i]
            ));
        }

        std::fs::write(filename, result)
    }
}

#[derive(Default)]
pub struct WidgetbenchLogResults {
    pub n_batches: Vec<u32>,
    pub batch_iter: Vec<u32>,
    pub duration: Vec<u128>,
    pub mem_mib: Vec<f64>,
    pub cpu_usage: Vec<f32>,
}

impl WidgetbenchLogResults {
    pub fn write_to_csv(&mut self, filename: &str) -> std::io::Result<()> {
        let mut result = String::from("n_batches,iter,duration,mem_mib,cpu_usage\n");

        for i in 0..self.n_batches.len() {
            result.push_str(&format!(
                "{},{},{},{},{}\n",
                self.n_batches[i],
                self.batch_iter[i],
                self.duration[i],
                self.mem_mib[i],
                self.cpu_usage[i]
            ));
        }

        std::fs::write(filename, result)
    }
}

pub fn get_os_id() -> String {
    std::env::consts::OS.chars().take(3).collect()
}

pub fn get_unix_timestamp()->u64 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}