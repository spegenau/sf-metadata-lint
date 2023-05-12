use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowOutputFieldAssignment  {
	#[serde(rename = "assignToReference")]
	pub assign_to_reference: String,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
