use std::time::Instant;

pub struct Benchmark {
    start_time: Instant,
    checkpoints: Vec<(String, Instant)>,
}

impl Benchmark {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            checkpoints: Vec::new(),
        }
    }

    pub fn checkpoint(&mut self, name: &str) {
        self.checkpoints.push((name.to_string(), Instant::now()));
    }

    pub fn print_metrics(&self, total_lines: usize) {
        println!("\n=== PERFORMANCE METRICS ===");
        println!("Total lines processed: {}", total_lines);
        
        let mut last_time = self.start_time;
        for (name, checkpoint_time) in &self.checkpoints {
            let duration = checkpoint_time.duration_since(last_time);
            println!("{}: {:.3}ms", name, duration.as_millis());
            last_time = *checkpoint_time;
        }
        
        let total_duration = last_time.duration_since(self.start_time);
        println!("Total execution time: {:.3}ms", total_duration.as_millis());
        println!("Lines per second: {:.0}", total_lines as f64 / total_duration.as_secs_f64());
    }

    pub fn get_total_duration(&self) -> std::time::Duration {
        if let Some((_, last_checkpoint)) = self.checkpoints.last() {
            last_checkpoint.duration_since(self.start_time)
        } else {
            Instant::now().duration_since(self.start_time)
        }
    }

    pub fn get_lines_per_second(&self, total_lines: usize) -> f64 {
        total_lines as f64 / self.get_total_duration().as_secs_f64()
    }
} 