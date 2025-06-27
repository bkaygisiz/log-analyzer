use std::fs::File;

// Import the analyzer module
mod analyzer;
use analyzer::analyze_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Incorrect number of arguments. Please provide a file to read from");
        return;
    }
    let file_path: &String = &args[1];
    let file = open_file(file_path);
    analyze_file(&file);
}

fn open_file(file_path: &String) -> File {
    let file_content = File::open(file_path);
    match file_content {
        Ok(file) => file,
        Err(e) => {
            eprintln!("ERROR: ===== Error opening file '{}': {}", file_path, e);
            std::process::exit(1); // Exit gracefully with error code
        }
    }
}
