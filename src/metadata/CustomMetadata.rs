use crate::metadata::CustomMetadataValue::CustomMetadataValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomMetadata  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "protected")]
	pub protected: Option<bool>,
	#[serde(rename = "values")]
	pub values: Option<Vec<CustomMetadataValue>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
