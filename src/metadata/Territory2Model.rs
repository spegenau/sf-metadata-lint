use crate::metadata::FieldValue::FieldValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory2Model  {
	#[serde(rename = "customFields")]
	pub custom_fields: Option<Vec<FieldValue>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
