use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TimeSheetTemplateAssignment  {
	#[serde(rename = "assignedTo")]
	pub assigned_to: Option<String>,
}
