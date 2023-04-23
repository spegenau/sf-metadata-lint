use crate::metadata::EntitlementProcessMilestoneTimeTrigger::EntitlementProcessMilestoneTimeTrigger;
use crate::metadata::FilterItem::FilterItem;
use crate::metadata::WorkflowActionReference::WorkflowActionReference;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EntitlementProcessMilestoneItem  {
	#[serde(rename = "businessHours")]
	pub business_hours: Option<String>,
	#[serde(rename = "criteriaBooleanFilter")]
	pub criteria_boolean_filter: Option<String>,
	#[serde(rename = "milestoneCriteriaFilterItems")]
	pub milestone_criteria_filter_items: Option<Vec<FilterItem>>,
	#[serde(rename = "milestoneCriteriaFormula")]
	pub milestone_criteria_formula: Option<String>,
	#[serde(rename = "milestoneName")]
	pub milestone_name: Option<String>,
	#[serde(rename = "minutesCustomClass")]
	pub minutes_custom_class: Option<String>,
	#[serde(rename = "minutesToComplete")]
	pub minutes_to_complete: Option<i32>,
	#[serde(rename = "successActions")]
	pub success_actions: Option<Vec<WorkflowActionReference>>,
	#[serde(rename = "timeTriggers")]
	pub time_triggers: Option<Vec<EntitlementProcessMilestoneTimeTrigger>>,
	#[serde(rename = "useCriteriaStartTime")]
	pub use_criteria_start_time: Option<bool>,
}
