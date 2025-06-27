// Export the analyze module
pub mod analyze;
pub mod types;

// Re-export the main analysis function for convenience
pub use analyze::analyze_file;
