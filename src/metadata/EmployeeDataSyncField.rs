use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmployeeDataSyncField  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "isDefault")]
	pub is_default: bool,
	#[serde(rename = "isRequired")]
	pub is_required: bool,
	#[serde(rename = "sourceField")]
	pub source_field: String,
	#[serde(rename = "targetField")]
	pub target_field: String,
}
