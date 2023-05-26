#![allow(non_snake_case)]
pub mod metadata;
pub use metadata::*;

pub mod utilities;
use profile::finding::FindingType;
pub use utilities::*;

pub mod checks;
pub use checks::*;
use utilities::finding::Finding;

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

use clap::Parser;

use crate::config::Config;
use crate::sf_xml_file::SFXMLFile;

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Don't fail if warnings occur
    #[arg(short, long, default_value_t = false)]
    ignore_warnings: bool,

    /// Try to fix findings (currently not working)
    #[arg(short, long, default_value_t = false)]
    fix_it: bool,

    /// Generate config file
    #[arg(short, long, default_value_t = false)]
    generate_config: bool,

    #[arg(short, long, value_name = "SFDX PROJECT FOLDER")]
    project_path: Option<PathBuf>,
}

fn main() {
    let start_time: Instant = Instant::now();

    let cli = Cli::parse();
    let project_path = cli.project_path.unwrap_or(PathBuf::from_str(".").unwrap());
    let fix_it = cli.fix_it;

    println!("Project Path: {}", project_path.to_str().unwrap());

    if cli.generate_config {
        Config::write_default(&project_path);
        std::process::exit(exitcode::OK);
    }

    let config = Config::load(HashMap::new(), project_path.to_str().unwrap());
    
    let mut findings: Vec<Finding> = Vec::new();
    findings.extend((all_xml_files::CheckAllXmlFiles {}).run_checks(&project_path, &config, fix_it));
    findings.extend((permission_set::CheckPermissionSet {}).run_checks(&project_path, &config, fix_it));
    findings.extend((object_field::CheckObjectField {}).run_checks(&project_path, &config, fix_it));
    findings.extend((profile::CheckProfiles {}).run_checks(&project_path, &config, fix_it));
    findings.extend((translation::CheckTranslations {}).run_checks(&project_path, &config, fix_it));
    findings.extend((layout::CheckLayout {}).run_checks(&project_path, &config, fix_it));
    findings.extend((recordtype::CheckRecordTypes {}).run_checks(&project_path, &config, fix_it));
    findings.extend((flow::CheckFlow {}).run_checks(&project_path, &config, fix_it));
    findings.extend((application::CheckApplication {}).run_checks(&project_path, &config, fix_it));

    let exit_code = process_findings(&findings, &cli.ignore_warnings);
    
    let end_time = Instant::now();
    println!("\nlinting took {:?}", end_time.duration_since(start_time));
    
    std::process::exit(exit_code);
}

fn process_findings(findings: &Vec<Finding>, ignore_warnings: &bool) -> i32 {
    let mut return_code = exitcode::OK;

    let mut errors = Finding::filter_by_type(findings, FindingType::ERROR);
    let mut infos = Finding::filter_by_type(findings, FindingType::INFO);
    let mut warnings = Finding::filter_by_type(findings, FindingType::WARNING);

    if infos.len() > 0 {
        println!("INFOS:");
        print_messages(&mut infos);
    }

    if warnings.len() > 0 {
        println!("WARNINGS:");
        print_messages(&mut warnings);
        if !ignore_warnings {
            return_code = exitcode::DATAERR;
        }
    }

    if errors.len() > 0 {
        println!("ERRORS:");
        print_messages(&mut errors);
        return_code = exitcode::DATAERR;
    }

    println!(
        "Found:\t{} infos, \t{} warnings,\t{} errors",
        infos.len(),
        warnings.len(),
        errors.len(),
    );

    return return_code;
}

fn print_messages(findings: &mut Vec<Finding>) {
    findings.sort_by_key(|f| String::from(f.file.as_str()));

    let mut max_len_filename = 0;
    for finding in &mut *findings {
        let file_len = finding.file.len();
        if file_len > max_len_filename {
            max_len_filename = file_len;
        }
    }

    let mut lines: String = String::from("");
    
    for finding in &mut *findings {
        let padded_filename = format!("{: <width$}", finding.file, width = max_len_filename);
        lines += format!("{padded_filename}   {} (Rule: {:?})\n", finding.message, finding.rule).as_str();
    }

    println!("{lines}");
}
