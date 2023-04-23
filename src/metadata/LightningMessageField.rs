use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LightningMessageField  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "fieldName")]
	pub field_name: String,
}
