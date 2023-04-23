use crate::metadata::WorkflowActionReference::WorkflowActionReference;
use crate::metadata::WorkflowTimeUnits::WorkflowTimeUnits;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowTimeTrigger  {
	#[serde(rename = "actions")]
	pub actions: Option<Vec<WorkflowActionReference>>,
	#[serde(rename = "offsetFromField")]
	pub offset_from_field: Option<String>,
	#[serde(rename = "timeLength")]
	pub time_length: Option<String>,
	#[serde(rename = "workflowTimeTriggerUnit")]
	pub workflow_time_trigger_unit: WorkflowTimeUnits,
}
