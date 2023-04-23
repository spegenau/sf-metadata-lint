use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowSubflowInputAssignment::FlowSubflowInputAssignment;
use crate::metadata::FlowSubflowOutputAssignment::FlowSubflowOutputAssignment;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowSubflow  {
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "flowName")]
	pub flow_name: String,
	#[serde(rename = "inputAssignments")]
	pub input_assignments: Option<Vec<FlowSubflowInputAssignment>>,
	#[serde(rename = "outputAssignments")]
	pub output_assignments: Option<Vec<FlowSubflowOutputAssignment>>,
	#[serde(rename = "storeOutputAutomatically")]
	pub store_output_automatically: Option<bool>,
}
