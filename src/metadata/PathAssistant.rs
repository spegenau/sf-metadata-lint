use crate::metadata::PathAssistantStep::PathAssistantStep;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PathAssistant  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "entityName")]
	pub entity_name: String,
	#[serde(rename = "fieldName")]
	pub field_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "pathAssistantSteps")]
	pub path_assistant_steps: Option<Vec<PathAssistantStep>>,
	#[serde(rename = "recordTypeName")]
	pub record_type_name: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
