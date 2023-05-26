use serde::{Deserialize, Serialize};

use crate::config::{Rule, Config};

#[derive(PartialEq, Clone)]


#[derive(Debug, Deserialize, Serialize)]
pub enum FindingType {
    WARNING,
    ERROR,
    INFO,
    OFF,
}
#[derive(Clone)]
pub struct Finding {
    pub file: String,
    pub line: Option<u32>,
    pub position: Option<u32>,
    pub message: String,
    pub solution: Option<String>,
    pub finding_type: FindingType,
    pub rule: Rule,
}

impl Finding {
    pub fn get_message(&self) -> String {
        let filename = &self.file;
        format!("{}: {}", filename, self.message)
    }

    pub fn log(&self) {
        println!("{}", self.get_message());
    }

    pub fn new(file: &str, message: String, config: &Config, rule: Rule) -> Finding {
        Finding {
            file: String::from(file),
            line: None,
            position: None,
            message,
            solution: None,
            finding_type: config.rules.get(&rule).unwrap().clone(),
            rule,
        }
    }

    pub fn filter_by_type(findings: &Vec<Finding>, finding_type: FindingType) -> Vec<Finding> {
        return findings
            .iter()
            .filter(|f| f.finding_type == finding_type)
            .map(|f| f.clone())
            .collect::<Vec<Finding>>();
    }
}
