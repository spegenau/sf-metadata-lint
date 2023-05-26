use std::collections::HashMap;
use std::path::PathBuf;

use crate::{finding::Finding, sf_xml_file::SFXMLFile, CustomApplication::CustomApplication};

pub use crate::utilities::*;

use self::{config::{Config, Rule}};

pub struct CheckApplication {}

impl SFXMLFile<CustomApplication> for CheckApplication {
    fn run_checks(&mut self, project_path: &PathBuf, config: &Config,  _fix_it: bool) -> Vec<Finding> {
        let (applications, mut findings) = self.get_structs(project_path, config);

        if applications.len() > 0 && config.should_execute(Rule::CustomApplication_no_missing_description) {
            findings.append(&mut self.check_for_descriptions(&applications, config));
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
        config: &Config
    ) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();
        for (filename, app) in applications {
            if app.description.is_none() {
                findings.push(Finding::new(filename.as_str(), format!("Best practice: Add a description for this custom application"), config, Rule::CustomApplication_no_missing_description));
            }
        }

        return findings;
    }

}
