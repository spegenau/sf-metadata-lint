use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowRecordFilter::FlowRecordFilter;
use crate::metadata::FlowSchedule::FlowSchedule;
use crate::metadata::FlowScheduledPath::FlowScheduledPath;
use crate::metadata::FlowTriggerType::FlowTriggerType;
use crate::metadata::RecordTriggerType::RecordTriggerType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowStart  {
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "doesRequireRecordChangedToMeetCriteria")]
	pub does_require_record_changed_to_meet_criteria: Option<bool>,
	#[serde(rename = "filterFormula")]
	pub filter_formula: Option<String>,
	#[serde(rename = "filterLogic")]
	pub filter_logic: Option<String>,
	#[serde(rename = "filters")]
	pub filters: Option<Vec<FlowRecordFilter>>,
	#[serde(rename = "object")]
	pub object: Option<String>,
	#[serde(rename = "objectContainer")]
	pub object_container: Option<String>,
	#[serde(rename = "recordTriggerType")]
	pub record_trigger_type: Option<RecordTriggerType>,
	#[serde(rename = "schedule")]
	pub schedule: Option<FlowSchedule>,
	#[serde(rename = "scheduledPaths")]
	pub scheduled_paths: Option<Vec<FlowScheduledPath>>,
	#[serde(rename = "segment")]
	pub segment: Option<String>,
	#[serde(rename = "triggerType")]
	pub trigger_type: Option<FlowTriggerType>,
}
