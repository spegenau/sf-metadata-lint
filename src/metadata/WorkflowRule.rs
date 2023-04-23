use crate::metadata::FilterItem::FilterItem;
use crate::metadata::WorkflowActionReference::WorkflowActionReference;
use crate::metadata::WorkflowTimeTrigger::WorkflowTimeTrigger;
use crate::metadata::WorkflowTriggerTypes::WorkflowTriggerTypes;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowRule  {
	#[serde(rename = "actions")]
	pub actions: Option<Vec<WorkflowActionReference>>,
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "criteriaItems")]
	pub criteria_items: Option<Vec<FilterItem>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "failedMigrationToolVersion")]
	pub failed_migration_tool_version: Option<String>,
	#[serde(rename = "formula")]
	pub formula: Option<String>,
	#[serde(rename = "triggerType")]
	pub trigger_type: WorkflowTriggerTypes,
	#[serde(rename = "workflowTimeTriggers")]
	pub workflow_time_triggers: Option<Vec<WorkflowTimeTrigger>>,
}
