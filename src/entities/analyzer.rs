use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use regex::Regex;
use once_cell::sync::Lazy;
use crate::entities::{AnalyzerResult, DailyAnalyzerResult};
use crate::utils::helpers::extract_day_part;

// Compile regex once at startup
static LOG_PATTERN_STATUS_CODE_AND_DATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^(\S+) - - \[([^\]]+)\] "([^"]+)" (\d+) (\d+) "-" "([^"]+)" ([\d.]+)$"#).unwrap()
});

pub struct Analyzer {
    pub file_path: String,
    pub analyzer_result: AnalyzerResult,
}

impl Analyzer {
    pub fn new(file_path: String) -> Self {
        Self { file_path, analyzer_result: AnalyzerResult::new() }
    }

    pub fn analyze_file(&mut self, file: &File) {
        let start_time = Instant::now();
        let buf_reader = BufReader::new(file);
        let mut analyzer_result = AnalyzerResult::new();
        let mut line_count = 0;
    
        for line in buf_reader.lines() {
            match line {
                Ok(line) => {
                   self.analyze_line(&line, &mut analyzer_result);
                   line_count += 1;
                }
                Err(e) => {
                    eprintln!("ERROR: ===== Error reading line: {}", e);
                }
            }
        }
    
        let analysis_time = Instant::now();
        analyzer_result.calculate_everything();
        let calculation_time = Instant::now();
        analyzer_result.print_everything();
        let total_time = Instant::now();
        
        // Performance metrics
        println!("\n=== PERFORMANCE METRICS ===");
        println!("Total lines processed: {}", line_count);
        println!("File reading + parsing: {:.3}ms", analysis_time.duration_since(start_time).as_millis());
        println!("Calculations: {:.3}ms", calculation_time.duration_since(analysis_time).as_millis());
        println!("Total execution time: {:.3}ms", total_time.duration_since(start_time).as_millis());
        println!("Lines per second: {:.0}", line_count as f64 / total_time.duration_since(start_time).as_secs_f64());
    }
    
    fn analyze_line(&mut self, line: &str, analyzer_result: &mut AnalyzerResult) {
        if let Some(captures) = LOG_PATTERN_STATUS_CODE_AND_DATE.captures(line) {
            let date = captures.get(2).unwrap().as_str();
            let status_code = captures.get(4).unwrap().as_str();
        
            // Extract just the day part (e.g., "24/Jun/2025" from "24/Jun/2025:08:14:32 +0000")
            let day_part = extract_day_part(date);
        
            // Check if a daily result for this day already exists
            if let Some(daily_result) = analyzer_result.get_daily_result_if_day_exists(&day_part) {
                daily_result.update_self(status_code);
            } else {
                let mut new_daily_result = DailyAnalyzerResult::new(day_part.clone());
                new_daily_result.update_self(status_code);
                analyzer_result.add_daily_result(new_daily_result);
            }
        } else {
            eprintln!("WARNING: Skipping malformed line: {}", line);
        }
    }
}