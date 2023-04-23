use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeCaseField  {
	#[serde(rename = "name")]
	pub name: Option<String>,
}
