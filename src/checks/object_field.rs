use std::collections::HashMap;
use std::path::PathBuf;

use crate::CustomField::CustomField;
pub use crate::utilities::*;

use self::finding::Finding;
use self::sf_xml_file::SFXMLFile;
use self::util::get_structs;

pub struct CheckObjectField {}

impl SFXMLFile for CheckObjectField {
    fn run_checks(&mut self, _project_path: &PathBuf) -> Vec<Finding> {
        
        let findings: Vec<Finding> = Vec::new();
                
        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.field-meta.xml")
    }
}

impl CheckObjectField {
    pub fn get_all_fields(&self, project_path: &PathBuf) -> (HashMap<String, HashMap<String, CustomField>>, Vec<Finding>) {
        let mut map: HashMap<String, HashMap<String, CustomField>> = HashMap::new();

        let (mut structs, findings) = get_structs::<CustomField>(self, project_path);

        for (path, the_field) in structs.drain() {
            let (object_name, field_name) = CheckObjectField::path_to_object_and_field(&path);

            if !map.contains_key(&object_name) {
                let key = String::from(object_name.as_str());
                map.insert(key, HashMap::new());
            }

            let new_key = String::from(object_name.as_str());
            map.get_mut(&new_key)
                .unwrap()
                .insert(field_name, the_field);
        }

        return (map, findings);
    }

    fn path_to_object_and_field(file_path: &String) -> (String, String) {
        let buf = PathBuf::from(file_path);
        let mut components = buf.components().collect::<Vec<_>>();

        let field = components.pop().unwrap().as_os_str().to_str().unwrap().replace(".field-meta.xml", "");
        let _fields_folder = components.pop();
        let object = components.pop().unwrap().as_os_str().to_str().unwrap();

        return (String::from(object), String::from(field));
    }
}
