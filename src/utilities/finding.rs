#[derive(PartialEq, Clone)]
pub enum FindingType {
    WARNING,
    ERROR,
    INFO,
}
#[derive(Clone)]
pub struct Finding {
    pub file: String,
    pub line: Option<u32>,
    pub position: Option<u32>,
    pub message: String,
    pub solution: Option<String>,
    pub r#type: FindingType,
}

impl Finding {
    pub fn get_message(&self) -> String {
        let filename = &self.file;
        format!("{}: {}", filename, self.message)
    }

    pub fn log(&self) {
        println!("{}", self.get_message());
    }

    pub fn new_error(file: &String, message: String) -> Finding {
        Finding {
            file: String::from(file.as_str()),
            line: None,
            position: None,
            message,
            solution: None,
            r#type: FindingType::ERROR,
        }
    }

    pub fn new_warning(file: &String, message: String) -> Finding {
        Finding {
            file: String::from(file.as_str()),
            line: None,
            position: None,
            message,
            solution: None,
            r#type: FindingType::WARNING,
        }
    }

    pub fn new_info(file: &String, message: String) -> Finding {
        Finding {
            file: String::from(file.as_str()),
            line: None,
            position: None,
            message,
            solution: None,
            r#type: FindingType::INFO,
        }
    }

    pub fn filter_by_type(findings: &Vec<Finding>, finding_type: FindingType) -> Vec<Finding> {
        return findings
            .iter()
            .filter(|f| f.r#type == finding_type)
            .map(|f| f.clone())
            .collect::<Vec<Finding>>();
    }
}
