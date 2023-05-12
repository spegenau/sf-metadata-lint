
use std::collections::HashMap;

use serde::Deserialize;

use super::{restriction::Restriction, sequence::Sequence, complex_content::ComplexContent, element::Element, extension::Extension};
#[derive(Debug, Deserialize, Clone)]
pub struct MetadataType {
    #[serde(rename = "restriction")]
    pub _restriction: Option<Restriction>,

    #[serde(rename = "sequence")]
    pub sequence: Option<Sequence>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "complexContent")]
    pub complex_content: Option<ComplexContent>,
}


impl MetadataType {
    pub fn as_struct(&self) -> Option<String> {
        match &self.name {
            Some(name) => {
                let mut struct_str: String = self.get_dependencies();
                struct_str.push_str("use serde::{Deserialize};\n\n");
                struct_str.push_str("#[derive(Debug, Deserialize)]\n");
                struct_str.push_str(&format!("pub struct {} ", &name));
                struct_str.push_str(" {\n");
                struct_str.push_str(&self.get_field_lines());
                struct_str.push_str("}\n");

                Some(struct_str)
            }
            None => None,
        }
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    pub fn get_field_lines(&self) -> String {
        let sequences: Vec<&Sequence> = self.get_sequences();

        let type_name = String::from(self.name.as_ref().unwrap());

        let lines = sequences
            .into_iter()
            .map(|sequence| match sequence.get_lines(&type_name) {
                Some(the_lines) => the_lines,
                None => String::from(""),
            })
            .reduce(|mut acc, l| {
                acc.push_str(&l);

                acc
            });

        match lines {
            Some(the_lines) => the_lines,
            None => String::from(""),
        }
    }

    pub fn get_dependencies(&self) -> String {
        let dependencies = self
            .get_sequences()
            .into_iter()
            .map(|s| s.get_dependencies(String::from(self.name.as_ref().unwrap())))
            .reduce(|mut acc, mut dep| {
                acc.append(&mut dep);
                acc
            });

        match dependencies {
            Some(dependencies) => {
                let dependencies = dependencies
                    .iter()
                    //.map(|s| format!("pub mod {};\n", &s))
                    .map(|s| format!("use crate::metadata::{}::{};\n", &s, &s))
                    .reduce(|mut acc, d| {
                        acc.push_str(&d);

                        acc
                    });

                match dependencies {
                    Some(dependencies) => dependencies,
                    None => String::from(""),
                }
            }
            None => String::from(""),
        }
    }

    fn get_sequences(&self) -> Vec<&Sequence> {
        let mut sequence_options: Vec<&Option<Sequence>> = Vec::new();

        sequence_options.push(&self.sequence);

        match &self.complex_content {
            Some(complex_content) => {
                sequence_options.push(&complex_content.extension._sequence);
            }
            None => {}
        }

        let mut sequences: Vec<&Sequence> = Vec::new();

        sequence_options.iter_mut().for_each(|o| match o {
            Some(sequence) => sequences.push(sequence),
            None => {}
        });

        sequences
    }


    
    pub fn extend(&mut self, all_types: &HashMap<String, MetadataType>) {
        let other_type = self.get_extension(all_types);

        match other_type {
            Some(actual_other_type) => {
                let other_elements = actual_other_type.get_elements();

                let mut own_sequence: Option<&mut Sequence> = None;

                match &mut self.complex_content {
                    Some(content) => {
                        let extension: &mut Extension = &mut content.extension;
                        match &mut extension._sequence {
                            Some(sequence) => {
                                own_sequence = Some(sequence);
                            },
                            None => {},
                        }
                    },
                    None => {},
                }
                match &mut self.sequence {
                    Some(sequence) => {
                        own_sequence = Some(sequence);
                    },
                    None => {},
                }

                match &mut own_sequence {
                    Some(own_sequence) => {
                        own_sequence.add_elements(other_elements);
                    },
                    None => {
                        println!("Sequence empty of {}", self.name.as_ref().unwrap());
                    },
                }
            },
            None => {},
        }
    }

    fn get_elements(&self) -> Vec<Element> {
        match &self.sequence {
            Some(sequence) => {
                match &sequence.elements {
                    Some(elements) => elements.clone(),
                    None => Vec::new(),
                }
            },
            None => Vec::new(),
        }
    }

    fn get_extension<'a>(&'a self, all_types: &'a HashMap<String, MetadataType>) -> Option<&MetadataType> {
        match &self.complex_content {
            Some(content) => {
                match &content.extension._extension {
                    Some(extension) => {
                        let extension_without_prefix = extension.replace("tns:", "");
                        match all_types.get(&extension_without_prefix) {
                            Some(other_type) => {
                                Some(other_type)
                            },
                            None => {
                                println!("Metadata Type '{extension_without_prefix}' which extends {} not found", self.name.as_ref().unwrap());
                                None
                            },
                        }
                    },
                    None => None,
                }
            },
            None => None,
        }
    }


}
