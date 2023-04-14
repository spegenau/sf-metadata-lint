
use glob::glob;
use std::{env};

pub fn get_files_by_pattern(project_path: &String, pattern: &String) -> Vec<std::path::PathBuf> {
    let glob_pattern = String::from(project_path) + pattern;

    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => files.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    files
}

pub fn get_project_path() -> String {
    let args: Vec<String> = env::args().collect();
    let mut project_path = &String::from(".");
    if args.len() == 2 {
        project_path = &args[1];
    }

    return String::from(project_path);
}