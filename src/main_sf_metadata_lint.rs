#![allow(non_snake_case)]
pub mod metadata;
pub use metadata::*;

pub mod utilities;
use profile::finding::FindingType;
pub use utilities::*;

pub mod checks;
pub use checks::*;
use utilities::finding::Finding;

use std::path::PathBuf;
use std::time::Instant;
use std::vec;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Don't fail if warnings occur
    #[arg(short, long, default_value_t = false)]
    ignore_warnings: bool,

    #[arg(short, long, value_name = "SFDX PROJECT FOLDER")]
    project_path: Option<PathBuf>,
}

fn main() {
    let start_time: Instant = Instant::now();

    let cli = Cli::parse();

    let mut project_path = PathBuf::new();
    match cli.project_path {
        Some(path) => project_path = path,
        None => {
            project_path.push(".");
        }
    }

    println!("Project Path: {}", project_path.to_str().unwrap());

    let checkers: Vec<Box<dyn sf_xml_file::SFXMLFile>> = vec![
        Box::new(all_xml_files::CheckAllXmlFiles {}),
        Box::new(permission_set::CheckPermissionSet {}),
        Box::new(object_field::CheckObjectField {}),
        Box::new(profile::CheckProfiles {}),
        Box::new(translation::CheckTranslations {}),
    ];

    let findings: Option<Vec<finding::Finding>> = run_all_checks(checkers, &project_path);

    let exit_code = process_findings(findings, &cli.ignore_warnings);

    let end_time = Instant::now();
    println!("\nlinting took {:?}", end_time.duration_since(start_time));

    std::process::exit(exit_code);
}

fn run_all_checks(
    checkers: Vec<Box<dyn sf_xml_file::SFXMLFile>>,
    project_path: &PathBuf,
) -> Option<Vec<finding::Finding>> {
    let findings: Option<Vec<finding::Finding>> = checkers
        .into_iter()
        .map(|mut b| b.run_checks(project_path) as Vec<Finding>)
        .reduce(|mut acc, mut new_findings| {
            acc.append(&mut new_findings);

            acc
        });

    return findings;
}

fn process_findings(
    findings: Option<Vec<Finding>>,
    ignore_warnings: &bool,
) -> i32 {
    let mut return_code = exitcode::OK;

    let findings: Vec<Finding> = findings.unwrap_or(Vec::new());

    let errors = findings
        .iter()
        .filter(|f| f.r#type == FindingType::ERROR)
        .collect::<Vec<&Finding>>();
    let warnings = findings
        .iter()
        .filter(|f| f.r#type == FindingType::WARNING)
        .collect::<Vec<&Finding>>();

    if warnings.len() > 0 {
        println!("\nWARNINGS:");
        print_messages(&warnings);
        if !ignore_warnings {
            return_code = exitcode::DATAERR;
        }
    }

    if errors.len() > 0 {
        println!("\nERRORS:");
        print_messages(&errors);
        return_code = exitcode::DATAERR;
    }

    println!("\n\nFound:\t{} errors,\t{} warnings", errors.len(), warnings.len());

    return return_code;
}

fn print_messages(findings: &Vec<&Finding>) {
    let mut messages: Vec<String> = findings
        .iter()
        .map(|f| f.get_message())
        .collect();

    messages.sort();

    print!("{}", messages.join("\n"));
}
