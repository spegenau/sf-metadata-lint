use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomLabel  {
	#[serde(rename = "categories")]
	pub categories: Option<String>,
	#[serde(rename = "language")]
	pub language: String,
	#[serde(rename = "protected")]
	pub protected: bool,
	#[serde(rename = "shortDescription")]
	pub short_description: String,
	#[serde(rename = "value")]
	pub value: String,
}
