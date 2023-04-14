use std::{path::PathBuf, fs};

use quick_xml::{Reader, events::Event};

use crate::{sf_xml_file::SFXMLFile, util::{get_files_by_pattern, get_project_path}, finding::{Finding, FindingType}};

pub struct AllXmlFiles {

}

impl SFXMLFile for AllXmlFiles {
    fn run_checks(&mut self) -> Vec<Finding> {
        
        let mut findings: Vec<Finding> = Vec::new();
        
        findings.append(&mut self.validate_all_xml_structures(get_project_path(), self.pattern()));
        
        findings
    }

    fn pattern(&self) -> String {
        String::from("/force-app/**/*.xml")
    }
}

impl AllXmlFiles {
    fn validate_all_xml_structures(&self, project_path: String, pattern: String) -> Vec<Finding> {
        let files = get_files_by_pattern(&project_path, &pattern);
    
        let mut findings: Vec<Finding> = Vec::new();
        for file in files {
            findings.append(&mut AllXmlFiles::is_valid_xml_file(file));
        }

        findings
    }
    
    fn is_valid_xml_file(path: PathBuf) -> Vec<Finding> {
        let file_path = path.to_str().unwrap();
        let xml = fs::read_to_string(file_path).expect("Unable to read file");
    
        let mut reader = Reader::from_str(&xml);
        reader.trim_text(true);
    
        let mut buf = Vec::new();
    
        let mut findings: Vec<Finding> = Vec::new();
        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            // NOTE: this is the generic case when we don't know about the input BufRead.
            // when the input is a &str or a &[u8], we don't actually need to use another
            // buffer, we could directly call `reader.read_event()`
            match reader.read_event_into(&mut buf) {
                Err(e) => {
                    findings.push(Finding {
                        file: String::from(file_path),
                        line: None,
                        position: Some(reader.buffer_position() as u32),
                        message: e.to_string(),
                        r#type: FindingType::ERROR,
                    });
                    break;
                },
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
    
        findings
    }
}