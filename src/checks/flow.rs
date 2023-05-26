use std::collections::HashMap;
use std::path::PathBuf;

use crate::Flow::Flow;
use crate::{finding::Finding, sf_xml_file::SFXMLFile};

pub use crate::utilities::*;

use self::config::{Config, Rule};

pub struct CheckFlow {}

impl SFXMLFile<Flow> for CheckFlow {
    fn run_checks(
        &mut self,
        project_path: &PathBuf,
        config: &Config,
        _fix_it: bool,
    ) -> Vec<Finding> {
        let (flows, mut findings) = self.get_structs(project_path, config);

        if flows.len() > 0 && config.should_execute(Rule::Flow_no_missing_description) {
            findings.append(&mut self.check_for_descriptions(&flows, config));
        }

        findings
    }

    fn pattern(&self) -> String {
        String::from("/**/*.flow-meta.xml")
    }
}

impl CheckFlow {
    pub fn check_for_descriptions(&mut self, flows: &HashMap<String, Flow>, config: &Config) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();
        for (filename, flow) in flows {
            if flow.description.is_none() {
                findings.push(Finding::new(
                    filename.as_str(),
                    format!("Best practice: Add a description for this flow"),
                    config,
                    Rule::Flow_no_missing_description,
                ));
            }
        }

        return findings;
    }
}
