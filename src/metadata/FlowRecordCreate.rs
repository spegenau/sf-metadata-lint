use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowInputFieldAssignment::FlowInputFieldAssignment;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowRecordCreate  {
	#[serde(rename = "assignRecordIdToReference")]
	pub assign_record_id_to_reference: Option<String>,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "faultConnector")]
	pub fault_connector: Option<FlowConnector>,
	#[serde(rename = "inputAssignments")]
	pub input_assignments: Option<Vec<FlowInputFieldAssignment>>,
	#[serde(rename = "inputReference")]
	pub input_reference: Option<String>,
	#[serde(rename = "object")]
	pub object: Option<String>,
	#[serde(rename = "storeOutputAutomatically")]
	pub store_output_automatically: Option<bool>,
}
