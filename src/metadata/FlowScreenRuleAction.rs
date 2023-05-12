use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowScreenRuleAction  {
	#[serde(rename = "attribute")]
	pub attribute: String,
	#[serde(rename = "fieldReference")]
	pub field_reference: String,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
