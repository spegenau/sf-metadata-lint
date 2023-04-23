use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomHttpHeader  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "headerFieldName")]
	pub header_field_name: String,
	#[serde(rename = "headerFieldValue")]
	pub header_field_value: String,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
}
