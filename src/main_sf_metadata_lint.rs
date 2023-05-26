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
use std::str::FromStr;
use std::time::Instant;

use clap::Parser;

use crate::sf_xml_file::SFXMLFile;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Don't fail if warnings occur
    #[arg(short, long, default_value_t = false)]
    ignore_warnings: bool,

    /// Try to fix findings
    #[arg(short, long, default_value_t = false)]
    fix_it: bool,

    #[arg(short, long, value_name = "SFDX PROJECT FOLDER")]
    project_path: Option<PathBuf>,
}

fn main() {
    let start_time: Instant = Instant::now();

    let cli = Cli::parse();
    let project_path = cli.project_path.unwrap_or(PathBuf::from_str(".").unwrap());
    let fix_it = cli.fix_it;

    println!("Project Path: {}", project_path.to_str().unwrap());

    let mut findings: Vec<Finding> = Vec::new();
    findings.extend((all_xml_files::CheckAllXmlFiles {}).run_checks(&project_path, fix_it));
    findings.extend((permission_set::CheckPermissionSet {}).run_checks(&project_path, fix_it));
    findings.extend((object_field::CheckObjectField {}).run_checks(&project_path, fix_it));
    findings.extend((profile::CheckProfiles {}).run_checks(&project_path, fix_it));
    findings.extend((translation::CheckTranslations {}).run_checks(&project_path, fix_it));
    findings.extend((layout::CheckLayout {}).run_checks(&project_path, fix_it));
    findings.extend((recordtype::CheckRecordTypes {}).run_checks(&project_path, fix_it));
    findings.extend((flow::CheckFlow {}).run_checks(&project_path, fix_it));
    findings.extend((application::CheckApplication {}).run_checks(&project_path, fix_it));

    let exit_code = process_findings(&findings, &cli.ignore_warnings);

    let end_time = Instant::now();
    println!("\nlinting took {:?}", end_time.duration_since(start_time));

    std::process::exit(exit_code);
}

fn process_findings(findings: &Vec<Finding>, ignore_warnings: &bool) -> i32 {
    let mut return_code = exitcode::OK;

    let errors = Finding::filter_by_type(findings, FindingType::ERROR);
    let warnings = Finding::filter_by_type(findings, FindingType::WARNING);

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

    println!(
        "\n\nFound:\t{} errors,\t{} warnings",
        errors.len(),
        warnings.len()
    );

    return return_code;
}

fn print_messages(findings: &Vec<Finding>) {
    let mut messages: Vec<String> = findings.iter().map(|f| f.get_message()).collect();

    messages.sort();

    print!("{}", messages.join("\n"));
}
