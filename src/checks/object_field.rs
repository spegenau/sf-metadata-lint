use std::collections::HashMap;
use std::path::PathBuf;

use regex::Regex;

pub use crate::utilities::*;
use crate::CustomField::CustomField;

use self::finding::Finding;
use self::sf_xml_file::SFXMLFile;
pub struct CheckObjectField {}

impl SFXMLFile<CustomField> for CheckObjectField {
    fn run_checks(&mut self, project_path: &PathBuf, _fix_it: bool) -> Vec<Finding> {
        let (fields, mut findings) = self.get_structs(project_path);
        findings.extend(self.check_for_descriptions(fields));

        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.field-meta.xml")
    }
}

impl CheckObjectField {
    pub fn get_all_fields(
        &self,
        project_path: &PathBuf,
    ) -> (HashMap<String, HashMap<String, CustomField>>, Vec<Finding>) {
        let mut map: HashMap<String, HashMap<String, CustomField>> = HashMap::new();

        let (mut structs, findings) = self.get_structs(project_path);

        let fieldmatcher = CheckObjectField::get_object_field_matcher();
        for (path, the_field) in structs.drain() {
            let (object_name, field_name) =
                CheckObjectField::path_to_object_and_field(&path, &fieldmatcher);

            if !map.contains_key(&object_name) {
                let key = String::from(object_name.as_str());
                map.insert(key, HashMap::new());
            }

            let new_key = String::from(object_name.as_str());
            map.get_mut(&new_key).unwrap().insert(field_name, the_field);
        }

        return (map, findings);
    }

    pub fn does_field_exist(project_path: &PathBuf, object: &str, field: &str) -> bool {
        let mut field_definition_path: PathBuf = PathBuf::new();
        field_definition_path.push(project_path);
        field_definition_path.push("force-app");
        field_definition_path.push("main");
        field_definition_path.push("default");
        field_definition_path.push("objects");
        field_definition_path.push(object);
        field_definition_path.push("fields");
        field_definition_path.push(format!("{field}.field-meta.xml"));

        return field_definition_path.exists();
    }

    pub fn check_for_descriptions(&self, fields: HashMap<String, CustomField>) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();

        let fieldmatcher = CheckObjectField::get_object_field_matcher();
        for (filename, field) in fields {
            let (_object_name, field_name) =
                CheckObjectField::path_to_object_and_field(&filename, &fieldmatcher);
                if !CheckObjectField::is_standard_or_managed(field_name.as_str()) && field.description.is_none() {
                    findings.push(Finding::new_warning(&filename, format!("Best practice: Add a description for this custom field")));
                }
        }

        findings
    }

    pub fn is_standard_or_managed(field_name: &str) -> bool {
        let occurences = field_name.match_indices("__").count();
        occurences == 2 || occurences == 0
    }

    pub fn get_object_field_matcher() -> Regex {
        Regex::new(r"[a-zA-Z-\\/\.]+objects[\\/]([0-9a-zA-Z_-]+)[\\/]fields[\\/]([a-zA-Z0-9_\-\\\.]+)\.field\-meta\.xml")
            .unwrap()
    }

    fn path_to_object_and_field(file_path: &String, matcher: &Regex) -> (String, String) {
        let captures = matcher
            .captures(file_path)
            .expect(format!("Did not match {file_path} (Regex: {})", matcher.as_str()).as_str());

        let object = captures.get(1).unwrap().as_str();
        let field = captures.get(2).unwrap().as_str();

        return (String::from(object), String::from(field));
    }
}
