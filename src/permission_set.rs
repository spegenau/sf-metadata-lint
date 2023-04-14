use crate::{sf_xml_file::SFXMLFile, finding::Finding};

pub struct PermissionSet {

}

impl SFXMLFile for PermissionSet {
    fn run_checks(&mut self) -> Vec<Finding> {
        
        let mut findings: Vec<Finding> = Vec::new();
        
        findings.append(&mut self.check_field_availability());
        
        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.xml")
    }
}

impl PermissionSet {

    pub fn check_field_availability(&mut self) -> Vec<Finding> {
        let findings: Vec<Finding> = Vec::new();

        findings
    }
}