use std::collections::HashMap;
use std::path::PathBuf;

use crate::{sf_xml_file::SFXMLFile, finding::Finding};
use crate::Profile::Profile;

pub use crate::utilities::*;

use self::config::{Config, Rule};
use self::util::{does_file_exist_in_project};

pub struct CheckProfiles {

}

impl SFXMLFile<Profile> for CheckProfiles {
    fn run_checks(&mut self, project_path: &PathBuf, config: &Config, _fix_it: bool) -> Vec<Finding> {
        
        
        let (profiles, mut findings) = self.get_structs(project_path, config);
        
        if profiles.len() > 0 {
            if config.should_execute(Rule::Profile_no_missing_page_layouts) {   
                findings.append(&mut self.validate_page_layout_availability(&profiles, project_path, config));
            }

            if config.should_execute(Rule::Profile_no_unwanted_parts) {
                findings.append(&mut self.check_for_unwanted_parts(&profiles, config));
            }
        }
            
        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.profile-meta.xml")
    }
}

impl CheckProfiles {
    pub fn validate_page_layout_availability(&mut self, profiles: &HashMap<String, Profile>, project_path: &PathBuf, config: &Config) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();

            for (filename, profile) in profiles {
                match &profile.layout_assignments {
                    Some(assignments) => {
                        assignments.iter().for_each(|assignment| {
                            let layout_file = format!("layouts/{}.layout-meta.xml", assignment.layout);

                            let layout_exists = does_file_exist_in_project(&layout_file, project_path);
                            if !layout_exists {
                                findings.push(Finding::new(&filename, format!("Could not find assigned layout: {}", layout_file), config, Rule::Profile_no_missing_page_layouts));
                            }
                        });
                    },
                    None => {},
                }
            }

        return findings;
    }

    pub fn check_for_unwanted_parts(&mut self, profiles: &HashMap<String, Profile>, config: &Config) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();

        for (filename, profile) in profiles {
            if profile.class_accesses.is_some() {
                findings.push(Finding::new(filename, String::from("Profile contains apex class accesses. Best Practice: Use Permission Sets to grant access."), config, Rule::Profile_no_unwanted_parts));
            }
            if profile.field_permissions.is_some() {
                findings.push(Finding::new(filename, String::from("Profile contains field permissions. Best Practice: Use Permission Sets to grant access."), config, Rule::Profile_no_unwanted_parts));
            }
            if profile.user_permissions.is_some() {
                findings.push(Finding::new(filename, String::from("Profile contains user permissions."), config, Rule::Profile_no_unwanted_parts));
            }
            if profile.page_accesses.is_some() {
                findings.push(Finding::new(filename, String::from("Profile contains page accesses. Best Practice: Use Permission Sets to grant access."), config, Rule::Profile_no_unwanted_parts));
            }
            if profile.flow_accesses.is_some() {
                findings.push(Finding::new(filename, String::from("Profile contains page accesses. Best Practice: Use Permission Sets to grant access."), config, Rule::Profile_no_unwanted_parts));
            }
            if profile.object_permissions.is_some() {
                findings.push(Finding::new(filename, String::from("Profile contains object permissions. Best Practice: Use Permission Sets to grant access."), config, Rule::Profile_no_unwanted_parts));
            }
            if profile.record_type_visibilities.is_some() {
                findings.push(Finding::new(filename, String::from("Profile contains record type visibilities. Best Practice: Use Permission Sets to grant access."), config, Rule::Profile_no_unwanted_parts));
            }
            if profile.tab_visibilities.is_some() {
                findings.push(Finding::new(filename, String::from("Profile contains tab visibilities. Best Practice: Use Permission Sets to grant access."), config, Rule::Profile_no_unwanted_parts));
            }
        }

        return findings;
    }
}