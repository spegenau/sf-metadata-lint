use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ListMetadataQuery  {
	#[serde(rename = "folder")]
	pub folder: Option<String>,
	#[serde(rename = "type")]
	pub _type: String,
}
