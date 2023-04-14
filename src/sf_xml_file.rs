use crate::{util::{get_files_by_pattern, get_project_path}, finding::Finding};
use std::{path::PathBuf, fs};


pub trait SFXMLFile {
    fn pattern(&self) -> String;
    fn run_checks(&mut self) -> Vec<Finding>;


    fn get_file_list(&self) -> Vec<PathBuf> {
        get_files_by_pattern(&get_project_path(), &self.pattern())
    }

    fn read_file(&self, path_buf: PathBuf) -> String {
        let file_path = path_buf.to_str().unwrap();
        let content = fs::read_to_string(file_path).expect("Unable to read file");

        content
    }
}
