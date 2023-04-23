use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CompactLayout  {
	#[serde(rename = "fields")]
	pub fields: Option<Vec<String>>,
	#[serde(rename = "label")]
	pub label: String,
}
