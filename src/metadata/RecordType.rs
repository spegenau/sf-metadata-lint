use crate::metadata::RecordTypePicklistValue::RecordTypePicklistValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecordType  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "businessProcess")]
	pub business_process: Option<String>,
	#[serde(rename = "compactLayoutAssignment")]
	pub compact_layout_assignment: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "picklistValues")]
	pub picklist_values: Option<Vec<RecordTypePicklistValue>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
