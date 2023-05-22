use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use regex::Regex;

use crate::CustomField::CustomField;
use crate::RecordType::RecordType;
use crate::global_value_set::CheckGlobalValueSet;
use crate::object_field::CheckObjectField;
use crate::{sf_xml_file::SFXMLFile, finding::Finding};
use urlencoding::decode;

pub use crate::utilities::*;

pub struct CheckRecordTypes {}

impl SFXMLFile<RecordType> for CheckRecordTypes {
    fn run_checks(&mut self, project_path: &PathBuf, _fix_it: bool) -> Vec<Finding> {        
        let (recordtypes, mut findings) = self.get_structs(project_path);
        
        if recordtypes.len() > 0 {
            findings.append(&mut self.validate_picklist_values(&recordtypes, project_path));
        }
            
        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.recordType-meta.xml")
    }
}

impl CheckRecordTypes {
    pub fn validate_picklist_values(&mut self, recordtypes: &HashMap<String, RecordType>, project_path: &PathBuf) -> Vec<Finding> {
        let (all_fields, mut findings) = (CheckObjectField {}).get_all_fields(project_path);

        let object_matcher = Regex::new(r"[a-zA-Z-\\/]+objects[\\/]([0-9a-zA-Z_-]+)[a-zA-Z0-9_\-\\\./]+").unwrap();

        let (global_pickist_values, picklistvalue_findings) = (CheckGlobalValueSet {}).get_global_picklists(project_path);
        findings.extend(picklistvalue_findings);


        for (filename, recordtype) in recordtypes {
            let object = CheckRecordTypes::get_object_name(filename, &object_matcher);

            match &recordtype.picklist_values {
                Some(picklist_values) => {
                    picklist_values.iter().for_each(|picklistvalue| {
                        let field_name = &picklistvalue.picklist;

                        if !CheckObjectField::is_standard_or_managed(&object) && !all_fields.contains_key(&object) {
                            findings.push(Finding::new_error(&filename, format!("Cannot find custom object '{object}'")));
                        } else if !CheckObjectField::is_standard_or_managed(&object) && !all_fields.get(&object).unwrap().contains_key(field_name) {
                            findings.push(Finding::new_error(&filename, format!("Cannot find custom field '{object}.{filename}'")));
                        } else if !CheckObjectField::is_standard_or_managed(field_name) {

                            match &picklistvalue.values {
                                Some(values) => {
                                    let all_fields = all_fields.get(&object);

                                    match all_fields {
                                        Some(all_fields) => {
                                            let field = all_fields.get(field_name);

                                            match field {
                                                Some(field) => {
                                                    let (available_picklist_values, get_values_findings) = self.get_picklist_values(&global_pickist_values, filename, &object, field);
                                                    findings.extend(get_values_findings);
                
                                                    for value in values {
                                                        let full_name = value.full_name.as_ref().unwrap();
                                                        let full_name: String = decode(full_name).expect("UTF-8").to_string();
                
                                                        if !available_picklist_values.contains(&full_name) {
                                                            findings.push(Finding::new_error(&filename, format!("Cannot find picklist value '{full_name}' on picklist '{object}.{field_name}'")));
                                                        }
                                                    }
                                                },
                                                None => {
                                                    findings.push(Finding::new_error(&filename, format!("Cannot find field '{object}.{field_name}'")));
                                                },
                                            }
                                        },
                                        None => {
                                            findings.push(Finding::new_error(&filename, format!("Cannot find object '{object}'")));
                                        },
                                    }                                    
                                },
                                None => {},
                            }
                        }
                    });
                },
                None => {},
            }
        }

        return findings;
    }

    fn get_object_name(filename: &String, matcher: &Regex) -> String {
        return String::from(matcher.captures(filename)
            .expect(format!("Did not match {filename} (Regex: {})", matcher.as_str()).as_str())
            .get(1)
            .unwrap()
            .as_str());
    }

    fn get_picklist_values(&self, global_pickist_values: &HashMap<String, HashSet<String>>, filename: &String, object: &String, field: &CustomField) -> (HashSet<String>, Vec<Finding>) {
        let mut findings: Vec<Finding> = Vec::new();
        let mut values: HashSet<String> = HashSet::new();

        match &field.value_set {
            Some(value_set) => {
                if value_set.value_set_name.is_some() {
                    // Global picklist
                    let picklist_name = value_set.value_set_name.as_ref().unwrap();

                    match global_pickist_values.get(picklist_name) {
                        Some(global_values) => {
                            values = global_values.clone();
                        },
                        None => {
                            findings.push(Finding::new_error(&filename, format!("Could not find global value set '{picklist_name}'")));
                        }, 
                    }
                } else if value_set.value_set_definition.is_some() {
                    // Normal picklist
                    let definition = value_set.value_set_definition.as_ref().unwrap();

                    match &definition.value {
                        Some(picklist_values) => {
                            for value in picklist_values {
                                values.insert(String::from(value.full_name.as_ref().unwrap()));
                            }
                        },
                        None => {
                            findings.push(Finding::new_error(&filename, format!("Picklist '{object}.{filename}' does not contain any values")));
                        },
                    }
                }
            },
            None => {
                findings.push(Finding::new_error(&filename, format!("Picklist '{object}.{filename}' does not contain a value set definition")));
            }
        }

        return (values, findings);
    }
}

