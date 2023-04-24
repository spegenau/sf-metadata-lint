#![allow(non_snake_case)]
pub mod metadata;
pub use metadata::*;

pub mod utilities;
pub use utilities::*;

pub mod checks;
pub use checks::*;
use utilities::finding::Finding;

use std::{vec};
use std::time::Instant;

use crate::util::get_project_path;


fn main() {
    let start_time: Instant = Instant::now();

    println!("Project Path: {}", get_project_path());

    let checkers: Vec<Box<dyn sf_xml_file::SFXMLFile>> = vec![
        Box::new(all_xml_files::CheckAllXmlFiles {}),
        Box::new(permission_set::CheckPermissionSet {}),
        Box::new(object_field::CheckObjectField {}),
        Box::new(profile::CheckProfiles {}),
    ];
    
    let findings: Option<Vec<finding::Finding>> = run_all_checks(checkers);

    let exit_code = process_findings(findings);

    let end_time = Instant::now();
    println!("\nlinting took {:?}", end_time.duration_since(start_time));

    std::process::exit(exit_code);
}

fn run_all_checks(checkers: Vec<Box<dyn sf_xml_file::SFXMLFile>>) -> Option<Vec<finding::Finding>> {
    let findings: Option<Vec<finding::Finding>> = checkers.into_iter()
    .map(|mut b| b.run_checks() as Vec<Finding>)
    .reduce(|mut acc, mut new_findings| {
        acc.append(&mut new_findings);

        acc
    });
    
    return findings;
}

fn process_findings(findings: Option<Vec<Finding>>) -> i32 {
    match findings {
        Some(actual_findings) => {
            if actual_findings.len() == 0 {
                println!("No issues found");
                exitcode::OK
            } else {
                let mut messages: Vec<String> = actual_findings
                    .iter()
                    .map(|f| f.get_message())
                    .collect();

                messages.sort();

                print!("{}", messages.join("\n"));
                exitcode::DATAERR
            }
        },
        None => {
            println!("No issues found");

            exitcode::OK
        },
    }
}


