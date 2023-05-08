use std::collections::HashMap;
use std::path::PathBuf;

use crate::object_field::CheckObjectField;
use crate::Layout::Layout;
use crate::LayoutColumn::LayoutColumn;
use crate::LayoutItem::LayoutItem;
use crate::LayoutSection::LayoutSection;
use crate::{finding::Finding, sf_xml_file::SFXMLFile};

pub use crate::utilities::*;

use self::util::get_structs;

pub struct CheckLayout {}

impl SFXMLFile for CheckLayout {
    fn run_checks(&mut self, project_path: &PathBuf) -> Vec<Finding> {
        let (layouts, mut findings) = get_structs::<Layout>(self, project_path);

        if layouts.len() > 0 {
            findings.append(&mut self.validate_layout_sections(&layouts, project_path));
        }

        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.layout-meta.xml")
    }
}

impl CheckLayout {
    pub fn validate_layout_sections(
        &mut self,
        layouts: &HashMap<String, Layout>,
        project_path: &PathBuf,
    ) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();
        for (filename, layout) in layouts {
            let object = CheckLayout::get_object_name(filename);

            match &layout.layout_sections {
                Some(layout_sections) => {
                    let local_findings = layout_sections
                        .iter()
                        .map(|section| {
                            section.validate_fields(filename, project_path, object.as_str())
                        })
                        .filter(|o| o.is_some())
                        .map(|findings| findings.unwrap())
                        .reduce(|mut acc, current| {
                            acc.extend(current);
                            acc
                        });

                    match local_findings {
                        Some(local_findings) => findings.extend(local_findings),
                        None => {}
                    }
                }
                None => {}
            }
        }

        return findings;
    }

    pub fn get_object_name(file_name: &String) -> String {
        let filename = String::from(
            PathBuf::from(file_name.as_str())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );

        let parts = filename
            .split("-")
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        let object_name = parts.get(0).unwrap();

        return String::from(object_name);
    }
}

impl LayoutSection {
    pub fn validate_fields(
        &self,
        file: &String,
        project_path: &PathBuf,
        object: &str,
    ) -> Option<Vec<Finding>> {
        match &self.layout_columns {
            Some(layout_columns) => layout_columns
                .iter()
                .map(|col| col.validate_fields(file, project_path, object))
                .filter(|o| o.is_some())
                .map(|findings| findings.unwrap())
                .reduce(|mut acc, new| {
                    acc.extend(new);
                    acc
                }),
            None => None,
        }
    }
}

impl LayoutColumn {
    pub fn validate_fields(
        &self,
        file: &String,
        project_path: &PathBuf,
        object: &str,
    ) -> Option<Vec<Finding>> {
        match &self.layout_items {
            Some(layout_items) => {
                let findings = layout_items
                    .iter()
                    .map(|item| item.validate_field(file, project_path, object))
                    .filter(|result| result.is_some())
                    .map(|finding| finding.unwrap())
                    .collect::<Vec<Finding>>();
                Some(findings)
            }
            None => None,
        }
    }
}

impl LayoutItem {
    pub fn validate_field(
        &self,
        file: &String,
        project_path: &PathBuf,
        object: &str,
    ) -> Option<Finding> {
        match &self.field {
            Some(field) => {
                if !CheckObjectField::is_standard_or_managed(field.as_str())
                    && !CheckObjectField::does_field_exist(project_path, object, field.as_str())
                {
                    Some(Finding::new_error(
                        file,
                        format!("Field does not exist: {field}"),
                    ))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
