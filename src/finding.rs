
pub enum FindingType {
    WARNING,
    ERROR
}

pub struct Finding {
    pub file: String,
    pub line: Option<u32>,
    pub position: Option<u32>,
    pub message: String,
    pub r#type: FindingType
}

impl Finding {
    pub fn log(&self) {
        println!("{}: {}", self.file, self.message);
    }
}