use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::entities::{AnalyzerResult, DailyAnalyzerResult};
use crate::utils::helpers::extract_day_part;

pub struct Analyzer {
    pub file_path: String,
    pub analyzer_result: AnalyzerResult,
}

impl Analyzer {
    pub fn new(file_path: String) -> Self {
        Self { file_path, analyzer_result: AnalyzerResult::new() }
    }

    pub fn analyze_file(&mut self, file: &File) {
        let buf_reader = BufReader::new(file);
        let mut analyzer_result = AnalyzerResult::new();
    
        for line in buf_reader.lines() {
            match line {
                Ok(line) => {
                   self.analyze_line(&line, &mut analyzer_result);
                }
                Err(e) => {
                    eprintln!("ERROR: ===== Error reading line: {}", e);
                }
            }
        }
    
        analyzer_result.calculate_everything();
        analyzer_result.print_everything();
    }
    
    fn analyze_line(&mut self,line: &str, analyzer_result: &mut AnalyzerResult) {
        let line_parts = line.split(" ").collect::<Vec<&str>>();
        let date = line_parts[3];
        let status_code = line_parts[8];
    
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
    }
}