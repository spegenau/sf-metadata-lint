use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomMetadataValue  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "value")]
	pub value: String,
}
