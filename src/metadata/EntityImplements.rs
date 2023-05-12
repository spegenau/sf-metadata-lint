use crate::metadata::FieldImplements::FieldImplements;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EntityImplements  {
	#[serde(rename = "fieldImplements")]
	pub field_implements: Option<Vec<FieldImplements>>,
	#[serde(rename = "isDefault")]
	pub is_default: Option<bool>,
	#[serde(rename = "isFullyMapped")]
	pub is_fully_mapped: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
