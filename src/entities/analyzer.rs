use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use once_cell::sync::Lazy;
use crate::entities::{AnalyzerResult, DailyAnalyzerResult};
use crate::utils::helpers::extract_day_part;
use crate::utils::benchmark::Benchmark;

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
        let mut benchmark = Benchmark::new();
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
    
        benchmark.checkpoint("File reading + parsing");
        analyzer_result.calculate_everything();
        benchmark.checkpoint("Calculations");
        analyzer_result.print_everything();
        benchmark.checkpoint("Output");
        
        benchmark.print_metrics(line_count);
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