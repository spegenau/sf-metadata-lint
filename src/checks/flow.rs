use std::collections::HashMap;
use std::path::PathBuf;

use crate::Flow::Flow;
use crate::{finding::Finding, sf_xml_file::SFXMLFile};

pub use crate::utilities::*;

pub struct CheckFlow {}

impl SFXMLFile<Flow> for CheckFlow {
    fn run_checks(&mut self, project_path: &PathBuf, _fix_it: bool) -> Vec<Finding> {
        let (flows, mut findings) = self.get_structs(project_path);

        if flows.len() > 0 {
            findings.append(&mut self.check_for_descriptions(&flows));
        }

        findings
    }

    fn pattern(&self) -> String {
        String::from("/**/*.flow-meta.xml")
    }
}

impl CheckFlow {
    pub fn check_for_descriptions(
        &mut self,
        flows: &HashMap<String, Flow>,
    ) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();
        for (filename, flow) in flows {
            if flow.description.is_none() {
                findings.push(Finding::new_warning(&filename, format!("Best practice: Add a description for this flow")));
            }
        }

        return findings;
    }

}
