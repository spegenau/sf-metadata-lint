use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use crate::metadata::FlowTransformValueAction::FlowTransformValueAction;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTransformValue  {
	#[serde(rename = "inputReference")]
	pub input_reference: Option<String>,
	#[serde(rename = "transformValueActions")]
	pub transform_value_actions: Option<Vec<FlowTransformValueAction>>,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
