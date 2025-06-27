// Declare the submodules
pub mod analyzer_result;
pub mod daily_analyzer_result;
pub mod analyzer;
// Re-export the structs for easier access
pub use analyzer_result::AnalyzerResult;
pub use daily_analyzer_result::DailyAnalyzerResult;
pub use analyzer::Analyzer;