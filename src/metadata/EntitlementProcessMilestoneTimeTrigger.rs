use crate::metadata::MilestoneTimeUnits::MilestoneTimeUnits;
use crate::metadata::WorkflowActionReference::WorkflowActionReference;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EntitlementProcessMilestoneTimeTrigger  {
	#[serde(rename = "actions")]
	pub actions: Option<Vec<WorkflowActionReference>>,
	#[serde(rename = "timeLength")]
	pub time_length: Option<i32>,
	#[serde(rename = "workflowTimeTriggerUnit")]
	pub workflow_time_trigger_unit: MilestoneTimeUnits,
}
