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

    pub fn write_to_csv(&self) {
        //todo
        for i in 0..self.timestamp.len() {
            print!("{} | ", self.timestamp[i]);
            print!("{} | ", self.fps[i]);
            print!("{} | ", self.n_nodes[i]);
            print!("{} | ", self.mem_mib[i]);
            print!("{}", self.cpu_usage[i]);
            println!();
        }
    }
}
