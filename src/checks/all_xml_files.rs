use std::{fs, path::PathBuf};

use quick_xml::{events::Event, Reader};
use serde::Deserialize;

use crate::{
    config::{Config, Rule},
    finding::{Finding},
    sf_xml_file::SFXMLFile,
    util::get_files_by_pattern,
};

#[derive(Debug, Deserialize)]
pub struct CheckAllXmlFiles {}

impl SFXMLFile<CheckAllXmlFiles> for CheckAllXmlFiles {
    fn run_checks(
        &mut self,
        project_path: &PathBuf,
        config: &Config,
        _fix_it: bool,
    ) -> Vec<Finding> {
        let mut findings: Vec<Finding> = Vec::new();

        if config.should_execute(Rule::XmlFiles_no_invalid_files) {
            findings.append(&mut self.validate_all_xml_structures(
                project_path,
                self.pattern(),
                config,
            ));
        }
            
        findings
    }

    fn pattern(&self) -> String {
        String::from("/**/*.xml")
    }
}

impl CheckAllXmlFiles {
    fn validate_all_xml_structures(
        &self,
        project_path: &PathBuf,
        pattern: String,
        config: &Config,
    ) -> Vec<Finding> {
        let files = get_files_by_pattern(&project_path, &pattern);

        let mut findings: Vec<Finding> = Vec::new();
        for file in files {
            //println!("Checking {}", file.as_os_str().to_str().unwrap());
            findings.append(&mut CheckAllXmlFiles::is_valid_xml_file(file, config));
        }

        findings
    }

    fn is_valid_xml_file(path: PathBuf, config: &Config) -> Vec<Finding> {
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
                    let mut finding: Finding = Finding::new(
                        &String::from(file_path),
                        format!("XML Parsing Error: {}", e.to_string()),
                        config,
                        Rule::XmlFiles_no_invalid_files
                    );
                    finding.position = Some(reader.buffer_position() as u32);
                    findings.push(finding);
                    break;
                }
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
