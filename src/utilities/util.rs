use glob::glob;
use quick_xml::DeError;
use serde::Deserialize;
use std::{fs, path::PathBuf};

use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub const MANAGED_PACKAGE_PATTERN: &str = "[a-zA-Z0-9_]+[_]{2}[a-zA-Z0-9_]+[__]{2}c";

pub fn get_files_by_pattern(project_path: &PathBuf, pattern: &String) -> Vec<std::path::PathBuf> {
    let path = String::from(project_path.to_str().unwrap());
    let glob_pattern = path + pattern;

    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for entry in glob(&glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => files.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    files
}

pub fn read_file(path_buf: PathBuf) -> String {
    let file_path = path_buf.to_str().unwrap();
    let content = fs::read_to_string(file_path).expect("Unable to read file");

    content
}

pub fn does_file_exist_in_project(rel_path: &String, project_path: &PathBuf) -> bool {
    let file_path = format!("{}/force-app/main/default/{rel_path}", project_path.to_str().unwrap());
    
    std::path::Path::new(&file_path).exists()
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
