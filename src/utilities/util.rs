use glob::glob;
use quick_xml::DeError;
use serde::Deserialize;
use std::{env, fs, path::PathBuf, collections::HashMap};

use quick_xml::events::Event;
use quick_xml::reader::Reader;

use crate::{finding::{Finding, FindingType}, sf_xml_file::SFXMLFile};

pub const MANAGED_PACKAGE_PATTERN: &str = "[a-zA-Z0-9_]+[_]{2}[a-zA-Z0-9_]+[__]{2}c";

pub fn get_files_by_pattern(project_path: &String, pattern: &String) -> Vec<std::path::PathBuf> {
    let glob_pattern = String::from(project_path) + pattern;

    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => files.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    files
}

pub fn get_project_path() -> String {
    let args: Vec<String> = env::args().collect();
    let mut project_path = &String::from(".");
    if args.len() == 2 {
        project_path = &args[1];
    }

    return String::from(project_path);
}

pub fn read_file(path_buf: PathBuf) -> String {
    let file_path = path_buf.to_str().unwrap();
    let content = fs::read_to_string(file_path).expect("Unable to read file");

    content
}

pub fn parse_file_as_struct<T: for<'a> Deserialize<'a>>(path: PathBuf) -> Result<T, String> {
    let xml = read_file(path); //.replace("<xsd:", "<").replace("</xsd:", "</");
    
    fs::write("debug.xml", xml.clone()).expect("Unable to write file");
    let result: Result<T, DeError> = quick_xml::de::from_str::<T>(&xml);

    match result {
        Ok(result) => Ok(result),
        Err(e) => {
            //debug_file(&xml);
            Err(e.to_string())
        },
    }
}

pub fn get_structs<T: for<'a> Deserialize<'a>>(the_checker_struct: &impl SFXMLFile) -> (HashMap<String, T>, Vec<Finding>) {
    let files = the_checker_struct.get_file_list();

    let mut structs: HashMap<String, T> = HashMap::new();
    let mut findings: Vec<Finding> = Vec::new();

    for path in files {
        let file_path = String::from(path.to_str().unwrap());
        let the_struct: Result<T, String> = parse_file_as_struct::<T>(path);
        match the_struct {
            Err(msg) => findings.push(Finding { file: file_path, line: None, position: None, message: msg, r#type: FindingType::ERROR, solution: None }),
            Ok(the_struct) => {
                structs.insert(file_path, the_struct);
            },
        }
    }

    return (structs, findings);
}


pub fn debug_file(xml: &String) {

    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => println!("{:#?}", e.name()),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
}
