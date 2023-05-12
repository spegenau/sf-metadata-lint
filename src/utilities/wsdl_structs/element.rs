
use serde::Deserialize;
use convert_case::{Case, Casing};

#[derive(Debug, Deserialize, Clone)]
pub struct Element {
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

