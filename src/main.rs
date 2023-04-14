pub mod permission_set;
pub mod sf_xml_file;
pub mod util;
pub mod finding;
mod all_xml_files;

use std::vec;

use all_xml_files::AllXmlFiles;
use permission_set::PermissionSet;
use sf_xml_file::SFXMLFile;
use finding::Finding;


fn main() {
    let checkers: Vec<Box<dyn SFXMLFile>> = vec![
        Box::new(AllXmlFiles {}),
        Box::new(PermissionSet {})
    ];
    
    let findings: Option<Vec<Finding>> = checkers.into_iter()
        .map(|mut b| b.run_checks() as Vec<Finding>)
        .reduce(|mut acc, mut new_findings| {
            acc.append(&mut new_findings);

            acc
        });

    match findings {
        Some(actual_findings) => {
            if actual_findings.len() == 0 {
                println!("No issues found");
            } else {
                actual_findings.iter().for_each(|f| f.log());
            }
        },
        None => println!("No issues found"),
    }
}


