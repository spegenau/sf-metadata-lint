use std::collections::HashMap;
use std::path::PathBuf;

use crate::{finding::Finding, sf_xml_file::SFXMLFile, CustomApplication::CustomApplication};

pub use crate::utilities::*;

pub struct CheckApplication {}

impl SFXMLFile<CustomApplication> for CheckApplication {
    fn run_checks(&mut self, project_path: &PathBuf, _fix_it: bool) -> Vec<Finding> {
        let (applications, mut findings) = self.get_structs(project_path);

        if applications.len() > 0 {
            findings.append(&mut self.check_for_descriptions(&applications));
        }

        findings
    }

    fn pattern(&self) -> String {
        String::from("/**/*.app-meta.xml")
    }
}

impl CheckApplication {
    pub fn check_for_descriptions(
        &mut self,
        applications: &HashMap<String, CustomApplication>,
    ) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();
        for (filename, app) in applications {
            if app.description.is_none() {
                findings.push(Finding::new_warning(&filename, format!("Best practice: Add a description for this custom application")));
            }
        }

        return findings;
    }

}
