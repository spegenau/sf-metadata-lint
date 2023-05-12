use crate::{
    util::read_file,
    wsdl_structs::{metadata_type::MetadataType, types::Types},
};
use regex::Regex;
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Deserialize, Clone)]
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

        let mut metadata_types = self._types._schema.metadata_types.clone();

        let mut metadata_type_map: HashMap<String, MetadataType> = HashMap::new();
        metadata_types
            .clone()
            .into_iter()
            .filter(|t| t.has_name())
            .for_each(|t| {
                metadata_type_map.insert(String::from(t.name.as_ref().unwrap()), t.clone());
            });

        metadata_types
            .iter_mut()
            .filter(|t| t.has_name())
            .for_each(|t| {
                t.extend(&metadata_type_map);
                match t.as_struct() {
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
                }
            });

        let mod_rs_content = metadata_types
            .into_iter()
            .filter(|t| t.has_name())
            .map(|the_type| {
                let name_option = &the_type.name.as_ref();
                let name = String::from(name_option.unwrap());
                format!("pub mod {};\n", name)
            })
            .reduce(|mut acc, s| {
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
            }
            None => todo!(),
        }
    }
}
