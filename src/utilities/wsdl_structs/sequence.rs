
use serde::Deserialize;

use super::element::Element;


#[derive(Debug, Deserialize, Clone)]
pub struct Sequence {
    #[serde(rename = "element")]
    pub elements: Option<Vec<Element>>,
}

impl Sequence {
    pub fn get_lines(&self, type_name: &String) -> Option<String> {
        match &self.elements {
            Some(elements) => {
                let lines = elements
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
            }
            None => {}
        }

        return deps;
    }

    pub fn add_elements(&mut self, new_elements: Vec<Element>) {
        match &self.elements {
            Some(elements) => {
                // println!("Adding elements {:#?}", new_elements);
                let mut collection: Vec<Element> = elements.clone();
                collection.extend(new_elements);
                self.elements = Some(collection);
                // println!("Have now {:#?}", self.elements);
            },
            None => {
                self.elements = Some(new_elements);
            },
        }
    }
}
