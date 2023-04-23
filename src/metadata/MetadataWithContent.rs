use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MetadataWithContent  {
	#[serde(rename = "content")]
	pub content: Option<String>,
}
