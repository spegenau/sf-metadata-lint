use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowInputFieldAssignment::FlowInputFieldAssignment;
use crate::metadata::FlowRecordFilter::FlowRecordFilter;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowRecordUpdate  {
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "faultConnector")]
	pub fault_connector: Option<FlowConnector>,
	#[serde(rename = "filterLogic")]
	pub filter_logic: Option<String>,
	#[serde(rename = "filters")]
	pub filters: Option<Vec<FlowRecordFilter>>,
	#[serde(rename = "inputAssignments")]
	pub input_assignments: Option<Vec<FlowInputFieldAssignment>>,
	#[serde(rename = "inputReference")]
	pub input_reference: Option<String>,
	#[serde(rename = "object")]
	pub object: Option<String>,
}
