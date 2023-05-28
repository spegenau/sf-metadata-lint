use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Result};
use crate::finding::FindingType;

pub const CONFIG_FILE: &str = ".sf-metadata-lint.json";

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Rule {
    CustomApplication_no_missing_description,
    CustomField_no_missing_descriptions,
    Flow_no_missing_description,
    Layout_no_missing_fields,
    PermissionSet_no_invalid_field_names,
    PermissionSet_no_missing_fields,
    PermissionSet_no_missing_objects,
    PermissionSet_no_permission_on_required_field,
    Picklist_no_empty_values,
    Picklist_no_missing_full_names,
    Profile_no_missing_page_layouts,
    Profile_no_unwanted_parts,
    RecordType_no_missing_fields,
    RecordType_no_missing_objects,
    RecordType_no_missing_picklist_values,
    Translations_no_empty_translations,
    XmlFiles_no_invalid_files,
    XmlFiles_no_invalid_structs,
    Picklist_no_missing_global_value_set,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub rules: HashMap<Rule, FindingType>
}

impl Config {
    pub fn load(_defaults: HashMap<String, FindingType>, project_path: &str) -> Config {
        let config = Config::read_file(&String::from(project_path));
        
        return config;
    }

    pub fn should_execute(&self, rule: Rule) -> bool {
        return self.rules.get(&rule).unwrap() != &FindingType::OFF;
    }

    fn read_file(project_path: &String) -> Config {
        let mut path_buf = PathBuf::from(project_path);
        path_buf.push(CONFIG_FILE);
        print!("Config file at {:?}: ", path_buf.to_str().unwrap());
        let file_content = fs::read_to_string(path_buf);

        let default_config = Config::get_default();

        match file_content {
            Ok(content) => {
                let parsed_file: Result<Config> = serde_json::from_str(&content.as_str());
                match parsed_file {
                    Ok(config) => {
                        println!("Found");
                        config
                    },
                    Err(_) => {
                        println!("INVALID -> Using defaults");
                        default_config
                    },
                }
            },
            Err(_err) => {
                println!("Not found -> Using defaults");
                default_config
            }, 
        }
    }

    pub fn get_default() -> Config {
        Config {
            rules: HashMap::from([
                (Rule::CustomApplication_no_missing_description, FindingType::WARNING),
                (Rule::CustomField_no_missing_descriptions, FindingType::WARNING),
                (Rule::Flow_no_missing_description, FindingType::WARNING),
                (Rule::Layout_no_missing_fields, FindingType::ERROR),
                (Rule::PermissionSet_no_invalid_field_names, FindingType::ERROR),
                (Rule::PermissionSet_no_missing_fields, FindingType::ERROR),
                (Rule::PermissionSet_no_missing_objects, FindingType::ERROR),
                (Rule::PermissionSet_no_permission_on_required_field, FindingType::ERROR),
                (Rule::Picklist_no_empty_values, FindingType::ERROR),
                (Rule::Picklist_no_missing_full_names, FindingType::ERROR),
                (Rule::Picklist_no_missing_global_value_set, FindingType::ERROR),
                (Rule::Profile_no_missing_page_layouts, FindingType::ERROR),
                (Rule::Profile_no_unwanted_parts, FindingType::WARNING),
                (Rule::RecordType_no_missing_fields, FindingType::ERROR),
                (Rule::RecordType_no_missing_objects, FindingType::ERROR),
                (Rule::RecordType_no_missing_picklist_values, FindingType::ERROR),
                (Rule::Translations_no_empty_translations, FindingType::WARNING),
                (Rule::XmlFiles_no_invalid_files, FindingType::ERROR),
                (Rule::XmlFiles_no_invalid_structs, FindingType::ERROR),
            ])
        }
    }

    pub fn write_default(project_path: &PathBuf) {
        let mut path_buf = project_path.clone();
        path_buf.push(CONFIG_FILE);
        let json = serde_json::to_string_pretty(&Config::get_default()).unwrap();
        fs::write(path_buf, json).unwrap();
    }
}