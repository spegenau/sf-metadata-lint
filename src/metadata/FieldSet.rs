use crate::metadata::FieldSetItem::FieldSetItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldSet  {
	#[serde(rename = "availableFields")]
	pub available_fields: Option<Vec<FieldSetItem>>,
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "displayedFields")]
	pub displayed_fields: Option<Vec<FieldSetItem>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
