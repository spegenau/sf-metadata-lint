use crate::metadata::FlowCondition::FlowCondition;
use crate::metadata::FlowConnector::FlowConnector;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowRule  {
	#[serde(rename = "conditionLogic")]
	pub condition_logic: String,
	#[serde(rename = "conditions")]
	pub conditions: Option<Vec<FlowCondition>>,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "doesRequireRecordChangedToMeetCriteria")]
	pub does_require_record_changed_to_meet_criteria: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
}
