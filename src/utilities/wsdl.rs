use crate::util::read_file;
use regex::Regex;
use serde::{Deserialize};
use serde_xml_rs::from_str;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};
use convert_case::{Case, Casing};

#[derive(Debug, Deserialize)]
struct Element {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "type")]
    element_type: String,

    #[serde(rename = "minOccurs")]
    min_occurs: Option<String>,

    #[serde(rename = "maxOccurs")]
    _max_occurs: Option<String>,
}

impl Element {
    pub fn get_struct_field(&self, type_name: &String) -> String {
        let name = &self.name;
        let t = self.get_rust_type(type_name);

        let mut snake_case_name = name.to_case(Case::Snake);
        if snake_case_name.eq("type") || snake_case_name.eq("if") {
            let mut prefixed = String::from("_");
            prefixed.push_str(snake_case_name.as_str());
            snake_case_name = prefixed;
        }

        let mut field = String::from("\t#[serde(rename = ");
        field.push_str(r#"""#);
        field.push_str(name);
        field.push_str(r#"")]"#);
        field.push_str("\n");
        field.push_str(format!("\tpub {}: {},\n", snake_case_name, t).as_str());

        return field;
    }

    pub fn get_rust_type(&self, type_name: &String) -> String {
        let mut t = String::from(&self.element_type);
        t = match t.as_str() {
            "xsd:anyType" => String::from("String"),
            "xsd:base64Binary" => String::from("String"),
            "xsd:boolean" => String::from("bool"),
            "xsd:date" => String::from("String"),
            "xsd:dateTime" => String::from("String"),
            "xsd:double" => String::from("f32"),
            "xsd:int" => String::from("i32"),
            "xsd:string" => String::from("String"),
            "xsd:time" => String::from("String"),
            _ => String::from(&t).replace("tns:", ""),
        };

        
        if t.eq(type_name) {
            t = format!("Box<{}>", t);
        }

        if self._max_occurs.is_some() {
            t = format!("Vec<{}>", t);
        }

        if self.min_occurs.is_some() {
            t = format!("Option<{}>", t);
        }

        return t;
    }

    pub fn get_dependencies(&self) -> Option<String> {
        if self.element_type.starts_with("tns:") {
            return Some(self.element_type.replace("tns:", ""));
        } else {
            return None;
        }
    }
}

#[derive(Debug, Deserialize)]
struct Sequence {
    #[serde(rename = "element")]
    pub elements: Option<Vec<Element>>,
}

impl Sequence {
    pub fn get_lines(&self, type_name: &String) -> Option<String> {
        match &self.elements {
            Some(elements) => {
                let lines =
                    elements
                        .into_iter()
                        .map(|e| e.get_struct_field(type_name))
                        .reduce(|mut acc, s| {
                            acc.push_str(&s);
                            acc
                        });
                lines
            }
            None => None,
        }
    }

    pub fn get_dependencies(&self, name: String) -> Vec<String> {
        let mut deps = Vec::new();

        match &self.elements {
            Some(elements) => {
                deps = elements
                    .into_iter()
                    .map(|e| e.get_dependencies())
                    .filter(|o| o.is_some())
                    .map(|o| o.unwrap())
                    .filter(|dep| dep.ne(&name))
                    .collect();

                deps.sort();
                deps.dedup();
            },
            None => {},
        }

        return deps;
    }
}

#[derive(Debug, Deserialize)]
struct Enumeration {
    #[serde(rename = "value")]
    _value: String,
}

#[derive(Debug, Deserialize)]
struct Restriction {
    #[serde(rename = "enumeration")]
    _enumeration: Option<Vec<Enumeration>>,

    #[serde(rename = "base")]
    _base: String,
}

#[derive(Debug, Deserialize)]
struct MetadataType {
    #[serde(rename = "restriction")]
    _restriction: Option<Restriction>,

    #[serde(rename = "sequence")]
    _sequence: Option<Sequence>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "complexContent")]
    _complex_content: Option<ComplexContent>,
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
        let dependencies = self.get_sequences()
            .into_iter()
            .map(|s| s.get_dependencies(String::from(self.name.as_ref().unwrap())))
            .reduce(|mut acc, mut dep| {
                acc.append(&mut dep);
                acc
            });

        match dependencies {
            Some(dependencies) => {
                let dependencies = dependencies.iter()
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
            },
            None => String::from(""),
        }
    }

    fn get_sequences(&self) -> Vec<&Sequence> {
        let mut sequence_options: Vec<&Option<Sequence>> = Vec::new();

        sequence_options.push(&self._sequence);

        match &self._complex_content {
            Some(complex_content) => {
                sequence_options.push(&complex_content._extension._sequence);
            }
            None => {}
        }

        let mut sequences: Vec<&Sequence> = Vec::new();
        
        sequence_options.iter_mut()
            .for_each(|o| {
                match o {
                    Some(sequence) => sequences.push(sequence),
                    None => {},
                }
            });

        sequences
    }
}

#[derive(Debug, Deserialize)]
struct Extension {
    #[serde(rename = "sequence")]
    _sequence: Option<Sequence>,
}

#[derive(Debug, Deserialize)]
struct ComplexContent {
    #[serde(rename = "extension")]
    _extension: Extension,
}

#[derive(Debug, Deserialize)]
struct Schema {
    #[serde(rename = "metadataType")]
    metadata_types: Vec<MetadataType>,
}

#[derive(Debug, Deserialize)]
struct Types {
    #[serde(rename = "schema")]
    _schema: Schema,
}

#[derive(Debug, Deserialize)]
pub struct WSDL {
    #[serde(rename = "types")]
    _types: Types,
}

impl WSDL {
    pub fn generate_structs() {
        let path = PathBuf::from(r"metadata.xml");
        let mut xml = read_file(path);

        let element_start = Regex::new(r#"<xsd:element\sname="[a-zA-Z0-9]+">"#).unwrap();
        let element_end = Regex::new(r#"</xsd:element>"#).unwrap();
        xml = element_start.replace_all(&xml, "").to_string();
        xml = element_end.replace_all(&xml, "").to_string();

        xml = xml
            .replace("<xsd:", "<")
            .replace("</xsd:", "</")
            .replace("<complexType", "<metadataType")
            .replace("</complexType", "</metadataType")
            .replace("<simpleType", "<metadataType")
            .replace("</simpleType", "</metadataType");

        fs::write("debug.xml", xml.clone()).expect("Unable to write file");
        let wsdl: WSDL = from_str::<WSDL>(&xml).unwrap(); 

        wsdl.write_files();
    }

    fn write_files(&self) {
        // Delete folder and create new
        let path = "src/metadata";
        let _removal_result = fs::remove_dir_all(path);
        let _folder_created = fs::create_dir_all("src/metadata");

        let metadata_types = &self._types._schema.metadata_types;

        metadata_types
            .into_iter()
            .filter(|t| t.has_name())
            .for_each(|t| match t.as_struct() {
                Some(the_struct) => {
                    let filename = format!("src/metadata/{}.rs", t.name.as_ref().unwrap());
                    let file = OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(filename);

                    match file {
                        Ok(mut the_file) => {
                            let _write_result = the_file.write(the_struct.as_bytes());
                        }
                        Err(_) => todo!(),
                    }
                }
                None => {}
            });

            let mod_rs_content = metadata_types
            .into_iter()
            .filter(|t| t.has_name())
            .map(|the_type| {
                let name_option = &the_type.name.as_ref();
                let name = String::from(name_option.unwrap());
                format!("pub mod {};\n", name)
            }).reduce(|mut acc, s| {
                acc.push_str(&s);

                return acc;
            });


            match mod_rs_content {
                Some(content) => {
                    let filename = String::from("src/metadata/mod.rs");
                    let file = OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(filename);
                    
                        match file {
                            Ok(mut the_file) => {
                                let _write_result = the_file.write(content.as_bytes());
                            }
                            Err(_) => todo!(),
                        }
                },
                None => todo!(),
            }
    }
}
