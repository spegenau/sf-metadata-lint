use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MetadataWithContent  {
	#[serde(rename = "content")]
	pub content: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
