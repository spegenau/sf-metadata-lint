use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordTypeTranslation  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
}
