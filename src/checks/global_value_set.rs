use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use regex::Regex;

pub use crate::utilities::*;
use crate::GlobalValueSet::GlobalValueSet;

use self::finding::Finding;
use self::sf_xml_file::SFXMLFile;
pub struct CheckGlobalValueSet {}

impl SFXMLFile<GlobalValueSet> for CheckGlobalValueSet {
    fn run_checks(&mut self, _project_path: &PathBuf, _fix_it: bool) -> Vec<Finding> {
        let findings: Vec<Finding> = Vec::new();

        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.globalValueSet-meta.xml")
    }
}

impl CheckGlobalValueSet {
    pub fn get_global_picklists(
        &self,
        project_path: &PathBuf,
    ) -> (HashMap<String, HashSet<String>>, Vec<Finding>) {
        let mut map = HashMap::new();
        let (global_value_sets, mut findings) = self.get_structs(project_path);

        let picklist_name_matcher =
            Regex::new(r"[a-zA-Z-\\]+globalValueSets[\\/]([a-zA-Z_-]+)[a-zA-Z0-9_\-\\\.]+")
                .unwrap();

        for (filename, global_value_set) in global_value_sets {
            let name = self.get_global_picklist_name(&filename, &picklist_name_matcher);

            match global_value_set.custom_value {
                Some(values) => {
                    let mut the_values = HashSet::new();
                    for value in values {
                        match value.full_name {
                            Some(full_name) => {
                                the_values.insert(full_name);
                            }
                            None => {
                                findings.push(Finding::new_error(
                                    &filename,
                                    format!("Picklist has a value without fullName attribute"),
                                ));
                            }
                        }
                    }
                    map.insert(name, the_values);
                }
                None => {
                    findings.push(Finding::new_error(
                        &filename,
                        format!("Picklist has no values at all"),
                    ));
                }
            }
        }

        return (map, findings);
    }

    fn get_global_picklist_name(&self, filename: &String, matcher: &Regex) -> String {
        return String::from(matcher.captures(filename).unwrap().get(1).unwrap().as_str());
    }
}
