use quick_xml::DeError;
use serde::Deserialize;

use crate::{finding::{Finding, FindingType}, util::{get_files_by_pattern, read_file}};
use std::{collections::HashMap, path::PathBuf, fs};

pub trait SFXMLFile<T: for<'a> Deserialize<'a>> {
    fn pattern(&self) -> String;

    fn run_checks(&mut self, project_path: &PathBuf, fix_it: bool) -> Vec<Finding>;

    fn get_file_list(&self, project_path: &PathBuf) -> Vec<PathBuf> {
        get_files_by_pattern(project_path, &self.pattern())
    }

    fn get_structs(&self, project_path: &PathBuf) -> (HashMap<String, T>, Vec<Finding>) {
        let files = self.get_file_list(project_path);

        let mut structs: HashMap<String, T> = HashMap::new();
        let mut findings: Vec<Finding> = Vec::new();
    
        for path in files {
            let file_path = String::from(path.to_str().unwrap());
            let the_struct: Result<T, String> = self.parse_file_as_struct(path);
            match the_struct {
                Err(msg) => findings.push(Finding { file: file_path, line: None, position: None, message: msg, r#type: FindingType::ERROR, solution: None }),
                Ok(the_struct) => {
                    structs.insert(file_path, the_struct);
                },
            }
        }
    
        return (structs, findings);
    }

    fn parse_file_as_struct(&self, path: PathBuf) -> Result<T, String> {
        let xml = read_file(path); //.replace("<xsd:", "<").replace("</xsd:", "</");
        
        fs::write("debug.xml", xml.clone()).expect("Unable to write file");
        let result: Result<T, DeError> = quick_xml::de::from_str::<T>(&xml);
    
        match result {
            Ok(result) => Ok(result),
            Err(e) => {
                //debug_file(&xml);
                Err(e.to_string())
            },
        }
    }
}
