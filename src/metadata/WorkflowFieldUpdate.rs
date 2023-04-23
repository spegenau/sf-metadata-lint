use crate::metadata::FieldUpdateOperation::FieldUpdateOperation;
use crate::metadata::LookupValueType::LookupValueType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowFieldUpdate  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "formula")]
	pub formula: Option<String>,
	#[serde(rename = "literalValue")]
	pub literal_value: Option<String>,
	#[serde(rename = "lookupValue")]
	pub lookup_value: Option<String>,
	#[serde(rename = "lookupValueType")]
	pub lookup_value_type: Option<LookupValueType>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "notifyAssignee")]
	pub notify_assignee: bool,
	#[serde(rename = "operation")]
	pub operation: FieldUpdateOperation,
	#[serde(rename = "protected")]
	pub protected: bool,
	#[serde(rename = "reevaluateOnChange")]
	pub reevaluate_on_change: Option<bool>,
	#[serde(rename = "targetObject")]
	pub target_object: Option<String>,
}
