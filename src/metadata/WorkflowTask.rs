use crate::metadata::ActionTaskAssignedToTypes::ActionTaskAssignedToTypes;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowTask  {
	#[serde(rename = "assignedTo")]
	pub assigned_to: Option<String>,
	#[serde(rename = "assignedToType")]
	pub assigned_to_type: ActionTaskAssignedToTypes,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "dueDateOffset")]
	pub due_date_offset: i32,
	#[serde(rename = "notifyAssignee")]
	pub notify_assignee: bool,
	#[serde(rename = "offsetFromField")]
	pub offset_from_field: Option<String>,
	#[serde(rename = "priority")]
	pub priority: String,
	#[serde(rename = "protected")]
	pub protected: bool,
	#[serde(rename = "status")]
	pub status: String,
	#[serde(rename = "subject")]
	pub subject: String,
}
