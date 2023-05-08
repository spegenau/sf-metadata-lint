
#[derive(PartialEq)]
pub enum FindingType {
    WARNING,
    ERROR
}

pub struct Finding {
    pub file: String,
    pub line: Option<u32>,
    pub position: Option<u32>,
    pub message: String,
    pub solution: Option<String>,
    pub r#type: FindingType
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
}