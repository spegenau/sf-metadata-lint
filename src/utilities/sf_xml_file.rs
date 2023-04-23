use crate::{util::{get_files_by_pattern, get_project_path}, finding::{Finding}};
use std::{path::PathBuf};


pub trait SFXMLFile {
    fn pattern(&self) -> String;
    fn run_checks(&mut self) -> Vec<Finding>;


    fn get_file_list(&self) -> Vec<PathBuf> {
        get_files_by_pattern(&get_project_path(), &self.pattern())
    }


}
