use crate::analyzer::types::{AnalyzerResult, DailyAnalyzerResult};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Analyzes a log file line by line
///
/// # Arguments
/// * `file` - A reference to the file to analyze
pub fn analyze_file(file: &File) {
    let buf_reader = BufReader::new(file);
    let mut analyzer_result = AnalyzerResult::new();

    for line in buf_reader.lines() {
        match line {
            Ok(line) => {
                analyze_line(&line, &mut analyzer_result);
            }
            Err(e) => {
                eprintln!("ERROR: ===== Error reading line: {}", e);
            }
        }
    }

    analyzer_result.calculate_everything();
    analyzer_result.print_everything();
}

fn analyze_line(line: &str, analyzer_result: &mut AnalyzerResult) {
    let line_parts = line.split(" ").collect::<Vec<&str>>();
    let date = line_parts[3];
    let status_code = line_parts[8];

    // Extract just the day part (e.g., "24/Jun/2025" from "24/Jun/2025:08:14:32 +0000")
    let day_part = extract_day_part(date);

    // Check if a daily result for this day already exists
    let mut found = false;
    for daily_result in &mut analyzer_result.daily_results {
        if daily_result.date == day_part {
            update_daily_result(daily_result, status_code);
            found = true;
            break;
        }
    }

    // If no daily result exists for this day, create a new one
    if !found {
        let mut new_daily_result = DailyAnalyzerResult::new(day_part.clone());
        update_daily_result(&mut new_daily_result, status_code);
        analyzer_result.add_daily_result(new_daily_result);
    }
}

/// Extracts just the day part from a log timestamp
///
/// # Arguments
/// * `date` - Full date string like "24/Jun/2025:08:14:32 +0000"
///
/// # Returns
/// * Day part like "24/Jun/2025"
fn extract_day_part(date: &str) -> String {
    // Split by ':' and take the first part, then remove the leading '['
    date.split(':')
        .next()
        .unwrap_or(date)
        .trim_start_matches('[')
        .to_string()
}

fn update_daily_result(daily_result: &mut DailyAnalyzerResult, status_code: &str) {
    daily_result.add_daily_requests_number(1);
    if status_code.starts_with("2") {
        daily_result.add_daily_2xx_requests_number(1);
    }
    if status_code.starts_with("3") {
        daily_result.add_daily_3xx_requests_number(1);
    }
    if status_code.starts_with("4") {
        daily_result.add_daily_4xx_requests_number(1);
    }
    if status_code.starts_with("5") {
        daily_result.add_daily_5xx_requests_number(1);
    }
    daily_result.calculate_daily_error_rate();
}
