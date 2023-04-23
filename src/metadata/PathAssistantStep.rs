use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PathAssistantStep  {
	#[serde(rename = "fieldNames")]
	pub field_names: Option<Vec<String>>,
	#[serde(rename = "info")]
	pub info: Option<String>,
	#[serde(rename = "picklistValueName")]
	pub picklist_value_name: String,
}
