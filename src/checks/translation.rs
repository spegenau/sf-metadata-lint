use std::collections::HashMap;
use std::path::PathBuf;

use crate::FlowDefinitionTranslation::FlowDefinitionTranslation;
use crate::FlowTranslation::FlowTranslation;
use crate::Translations::Translations;
use crate::{finding::Finding, sf_xml_file::SFXMLFile};

pub use crate::utilities::*;

pub struct CheckTranslations {}

impl SFXMLFile<Translations> for CheckTranslations {
    fn run_checks(&mut self, project_path: &PathBuf, _fix_it: bool) -> Vec<Finding> {
        let (translations, mut findings) = self.get_structs(project_path);

        if translations.len() > 0 {
            findings.append(&mut self.check_for_empty_translations(&translations));
        }

        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.translation-meta.xml")
    }
}

impl CheckTranslations {
    pub fn check_for_empty_translations(
        &mut self,
        translations: &HashMap<String, Translations>,
    ) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();

        for (filename, translation) in translations {
            match &translation.custom_applications {
                Some(applications) => {
                    let missing_labels: Vec<Finding> = applications
                        .iter()
                        .filter(|a| a.label.is_empty())
                        .map(|a| {
                            CheckTranslations::create_missing_label_warning(
                                "customApplications",
                                filename,
                                a.name.as_str(),
                            )
                        })
                        .collect();
                    findings.extend(missing_labels);
                }
                None => {}
            }

            match &translation.custom_labels {
                Some(labels) => {
                    let missing_labels: Vec<Finding> = labels
                        .iter()
                        .filter(|a| a.label.is_empty())
                        .map(|a| {
                            CheckTranslations::create_missing_label_warning(
                                "customLabels",
                                filename,
                                a.name.as_str(),
                            )
                        })
                        .collect();
                    findings.extend(missing_labels);
                }
                None => {}
            }

            match &translation.custom_tabs {
                Some(tabs) => {
                    let missing_labels: Vec<Finding> = tabs
                        .iter()
                        .filter(|a| a.label.is_empty())
                        .map(|a| {
                            CheckTranslations::create_missing_label_warning(
                                "customTabs",
                                filename,
                                a.name.as_str(),
                            )
                        })
                        .collect();
                    findings.extend(missing_labels);
                }
                None => {}
            }

            match &translation.quick_actions {
                Some(actions) => {
                    let missing_labels: Vec<Finding> = actions
                        .iter()
                        .filter(|a| a.label.is_empty())
                        .map(|a| {
                            CheckTranslations::create_missing_label_warning(
                                "quickActions",
                                filename,
                                a.name.as_str(),
                            )
                        })
                        .collect();
                    findings.extend(missing_labels);
                }
                None => {}
            }

            match &translation.flow_definitions {
                Some(actions) => {
                    let missing_labels: Vec<Finding> = actions
                        .iter()
                        .filter(|a| a.is_empty())
                        .map(|a| {
                            CheckTranslations::create_missing_label_warning(
                                "flowDefinitions",
                                filename,
                                &a.full_name.as_str(),
                            )
                        })
                        .collect();
                    findings.extend(missing_labels);
                }
                None => {}
            }
        }

        return findings;
    }

    pub fn create_missing_label_warning(type_of: &str, filename: &str, api_name: &str) -> Finding {
        let mut finding = Finding::new_warning(
            &String::from(filename),
            format!("the {type_of} '{api_name}' misses a translation."),
        );
        finding.solution = Some(format!(
            "If you don't provide a translation remove it from the file"
        ));

        return finding;
    }
}


impl FlowDefinitionTranslation {
    pub fn is_empty(&self) -> bool {
        match &self.flows {
            Some(flows) => {
                flows.is_empty() || (flows.len() == 1 && flows.get(0).unwrap().is_empty())
            },
            None => true,
        }
    }
}

impl FlowTranslation {
    pub fn is_empty(&self) -> bool {
        (self.choices.is_none() || self.choices.as_ref().unwrap().is_empty())
        && (self.full_name.is_none() || self.full_name.as_ref().unwrap().is_empty())
        && (self.label.is_none() || self.label.as_ref().unwrap().is_empty())
        && (self.screens.is_none() || self.screens.as_ref().unwrap().is_empty())
        && (self.stages.is_none() || self.stages.as_ref().unwrap().is_empty())
        && (self.text_templates.is_none() || self.text_templates.as_ref().unwrap().is_empty())
    }
}

