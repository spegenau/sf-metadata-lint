use std::collections::HashMap;
use std::path::PathBuf;

use crate::object_field::CheckObjectField;
use crate::Layout::Layout;
use crate::LayoutColumn::LayoutColumn;
use crate::LayoutItem::LayoutItem;
use crate::LayoutSection::LayoutSection;
use crate::{finding::Finding, sf_xml_file::SFXMLFile};

pub use crate::utilities::*;

use self::config::{Config, Rule};

pub struct CheckLayout {}

impl SFXMLFile<Layout> for CheckLayout {
    fn run_checks(&mut self, project_path: &PathBuf, config: &Config,  _fix_it: bool) -> Vec<Finding> {
        let (layouts, mut findings) = self.get_structs(project_path, config);

        if layouts.len() > 0 && config.should_execute(Rule::Layout_no_missing_fields) {
            findings.append(&mut self.validate_layout_sections(&layouts, project_path, config));
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
        config: &Config
    ) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();
        for (filename, layout) in layouts {
            let object = CheckLayout::get_object_name(filename);

            match &layout.layout_sections {
                Some(layout_sections) => {
                    let local_findings = layout_sections
                        .iter()
                        .map(|section| {
                            section.validate_fields(filename, project_path, object.as_str(), config)
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
        config: &Config
    ) -> Option<Vec<Finding>> {
        match &self.layout_columns {
            Some(layout_columns) => layout_columns
                .iter()
                .map(|col| col.validate_fields(file, project_path, object, config))
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
        config: &Config
    ) -> Option<Vec<Finding>> {
        match &self.layout_items {
            Some(layout_items) => {
                let findings = layout_items
                    .iter()
                    .map(|item| item.validate_field(file, project_path, object, config))
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
        config: &Config
    ) -> Option<Finding> {
        match &self.field {
            Some(field) => {
                if !CheckObjectField::is_standard_or_managed(field.as_str())
                    && !CheckObjectField::does_field_exist(project_path, object, field.as_str())
                {
                    Some(Finding::new(
                        file,
                        format!("Field does not exist: {field}"),
                        config,
                        config::Rule::Layout_no_missing_fields
                    ))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
