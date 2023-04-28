use std::path::PathBuf;

use regex::Regex;

use crate::object_field::CheckObjectField;
pub use crate::utilities::*;
use crate::PermissionSet::PermissionSet;

use self::finding::Finding;
use self::sf_xml_file::SFXMLFile;
use self::util::{get_structs, MANAGED_PACKAGE_PATTERN};

pub struct CheckPermissionSet {}

impl SFXMLFile for CheckPermissionSet {
    fn run_checks(&mut self, project_path: &PathBuf) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();

        findings.append(&mut self.check_field_availability(project_path));

        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.permissionset-meta.xml")
    }
}

impl CheckPermissionSet {
    pub fn check_field_availability(&mut self, project_path: &PathBuf) -> Vec<Finding> {
        let (structs, mut findings) = get_structs::<PermissionSet>(self, project_path);

        if structs.len() > 0 {
            let field_checker = CheckObjectField {};
            let (fields, get_fields_findings) = field_checker.get_all_fields(project_path);
            findings.extend(get_fields_findings);

            let managed_package_expr = Regex::new(MANAGED_PACKAGE_PATTERN).unwrap();

            for (filename, permSet) in structs {
                match permSet.field_permissions {
                    Some(field_permissions) => {
                        field_permissions.iter()
                            .for_each(|perm: &crate::PermissionSetFieldPermissions::PermissionSetFieldPermissions| {
                                let parts: Vec<String> = perm.field.split(".").map(|s| String::from(s)).collect();

                                if parts.len() == 2 {
                                    let object = parts.get(0).unwrap();
                                    let field = parts.get(1).unwrap();

                                    let OBJECT_AVAILABLE: bool = fields.contains_key(object);
                                    let IS_STANDARD_FIELD: bool= !field.ends_with("__c");

                                    let IS_MANAGED_OBJECT: bool = managed_package_expr.is_match(&object);
                                    let IS_MANAGED_FIELD: bool = managed_package_expr.is_match(&field);

                                    if OBJECT_AVAILABLE {
                                        let FIELD_AVAILABLE: bool = fields.get(object).unwrap().contains_key(field);
                                        if !FIELD_AVAILABLE && IS_MANAGED_FIELD {

                                        } else if !FIELD_AVAILABLE && IS_STANDARD_FIELD {

                                        } else if !FIELD_AVAILABLE {
                                            let mut finding = Finding::new_error(&filename, format!("Custom Field '{}' on Object '{}' not found.", field, object));
                                            finding.solution = Some(format!("Add the field {} to the project.", field));
                                            findings.push(finding);
                                        }

                                    } else if !IS_STANDARD_FIELD && !IS_MANAGED_OBJECT {
                                            let mut finding = Finding::new_error(&filename, format!("Standard Object '{}' for Custom Field '{}' not found.", object, field));
                                            finding.solution = Some(format!("Add Object '{}' to the project.", object));
                                            findings.push(finding);
                                    }

                                } else {
                                    findings.push(Finding::new_error(&filename, format!("Invalid field format. Expecting Object.Field. Found '{}'", perm.field)));
                                }
                            });
                    }
                    None => {}
                }
            }
        }

        findings
    }
}
