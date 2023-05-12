use crate::metadata::IndexField::IndexField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Index  {
	#[serde(rename = "fields")]
	pub fields: Option<Vec<IndexField>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
