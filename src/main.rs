use std::{fs::File, path::Path};

pub mod utils;
pub mod entities;
use crate::entities::Analyzer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Incorrect number of arguments. Please provide a file to read from");
        return;
    }
    let file_path: &String = &args[1];
    if file_exists(file_path) {
        let mut analyzer = Analyzer::new(file_path.clone());
        match open_file(&analyzer.file_path) {
            Ok(file) => {
                analyzer.analyze_file(&file);
            }
            Err(e) => {
                eprintln!("ERROR: Failed to open file '{}': {}", file_path, e);
                std::process::exit(1);
            }
        }
    }
    else {
        println!("File does not exist");
        std::process::exit(1);
    }
}

fn file_exists(file_path: &String) -> bool {
    Path::new(file_path).exists()
}

fn open_file(file_path: &String) -> Result<File, std::io::Error> {
    File::open(file_path)
}
