use crate::{sf_xml_file::SFXMLFile, finding::Finding};
use crate::Profile::Profile;

pub use crate::utilities::*;

use self::util::{get_structs, does_file_exist_in_project};

pub struct CheckProfiles {

}

impl SFXMLFile for CheckProfiles {
    fn run_checks(&mut self) -> Vec<Finding> {
        
        let mut findings: Vec<Finding> = Vec::new();
        
        findings.append(&mut self.validate_page_layout_availability());
        
        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.profile-meta.xml")
    }
}

impl CheckProfiles {
    pub fn validate_page_layout_availability(&mut self) -> Vec<Finding> {
        let (structs, mut findings) = get_structs::<Profile>(self);

        if structs.len() > 0 {
            for (filename, profile) in structs {
                match profile.layout_assignments {
                    Some(assignments) => {
                        assignments.iter().for_each(|assignment| {
                            let layout_file = format!("layouts/{}.layout-meta.xml", assignment.layout);

                            let layout_exists = does_file_exist_in_project(&layout_file);
                            if !layout_exists {
                                findings.push(Finding::new_error(&filename, format!("Could not find assigned layout: {}", layout_file)));
                            }
                        });
                    },
                    None => {},
                }
            }
        }

        return findings;
    }
}