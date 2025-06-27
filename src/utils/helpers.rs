/// Extracts just the day part from a log timestamp
///
/// # Arguments
/// * `date` - Full date string like "24/Jun/2025:08:14:32 +0000"
///
/// # Returns
/// * Day part like "24/Jun/2025"
pub fn extract_day_part(date: &str) -> String {
    // Split by ':' and take the first part, then remove the leading '['
    date.split(':')
        .next()
        .unwrap_or(date)
        .trim_start_matches('[')
        .to_string()
}