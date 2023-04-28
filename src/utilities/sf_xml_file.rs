use crate::{util::{get_files_by_pattern}, finding::{Finding}};
use std::{path::PathBuf};


pub trait SFXMLFile {
    fn pattern(&self) -> String;
    fn run_checks(&mut self, project_path: &PathBuf) -> Vec<Finding>;


    fn get_file_list(&self, project_path: &PathBuf) -> Vec<PathBuf> {
        get_files_by_pattern(project_path, &self.pattern())
    }


}
